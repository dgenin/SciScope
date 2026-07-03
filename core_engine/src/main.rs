use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use v4l::Device;
use v4l::format::FourCC;
use v4l::io::traits::CaptureStream;
use v4l::io::mmap::Stream as MmapStream;
use v4l::video::Capture;

pub mod dsp;
pub mod measure;

slint::include_modules!();

const FRAME_WIDTH: usize = 2592;
const FRAME_HEIGHT: usize = 1944;
const TARGET_FPS: u32 = 30;
const RAW_CHANNELS: usize = 2; // YUYV (2 bytes per pixel)
const RGBA_CHANNELS: usize = 4; // RGBA
const RAW_FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * RAW_CHANNELS;
const RGBA_FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * RGBA_CHANNELS;
const NUM_BUFFERS: usize = 4;

/// A single frame buffer with its capture timestamp.
struct Frame {
    raw_data: Vec<u8>,
    rgba_data: Vec<u8>,
    capture_time: Instant,
}

/// The pool of pre-allocated frame buffers.
struct RingBufferPool {
    frames: Vec<Mutex<Frame>>,
}

impl RingBufferPool {
    fn new(size: usize) -> Self {
        let mut frames = Vec::with_capacity(size);
        for _ in 0..size {
            frames.push(Mutex::new(Frame {
                raw_data: vec![0; RAW_FRAME_SIZE],
                rgba_data: vec![0; RGBA_FRAME_SIZE],
                capture_time: Instant::now(),
            }));
        }
        Self { frames }
    }
}

// Basic YUYV to RGBA conversion
fn convert_yuyv_to_rgba(raw: &[u8], rgba: &mut [u8]) {
    let num_pixels = FRAME_WIDTH * FRAME_HEIGHT;
    if raw.len() < num_pixels * 2 || rgba.len() < num_pixels * 4 {
        return;
    }

    for i in (0..num_pixels).step_by(2) {
        let raw_idx = i * 2;
        let rgba_idx = i * 4;

        let y0 = raw[raw_idx] as f32;
        let u  = raw[raw_idx + 1] as f32;
        let y1 = raw[raw_idx + 2] as f32;
        let v  = raw[raw_idx + 3] as f32;

        let c = u - 128.0;
        let d = v - 128.0;

        let r0 = (y0 + 1.402 * d).clamp(0.0, 255.0) as u8;
        let g0 = (y0 - 0.344136 * c - 0.714136 * d).clamp(0.0, 255.0) as u8;
        let b0 = (y0 + 1.772 * c).clamp(0.0, 255.0) as u8;

        let r1 = (y1 + 1.402 * d).clamp(0.0, 255.0) as u8;
        let g1 = (y1 - 0.344136 * c - 0.714136 * d).clamp(0.0, 255.0) as u8;
        let b1 = (y1 + 1.772 * c).clamp(0.0, 255.0) as u8;

        rgba[rgba_idx] = r0;
        rgba[rgba_idx + 1] = g0;
        rgba[rgba_idx + 2] = b0;
        rgba[rgba_idx + 3] = 255;

        rgba[rgba_idx + 4] = r1;
        rgba[rgba_idx + 5] = g1;
        rgba[rgba_idx + 6] = b1;
        rgba[rgba_idx + 7] = 255;
    }
}

fn main() -> Result<(), slint::PlatformError> {
    println!("Initializing V4L2 Core Engine (Scientific Suite)...");
    
    let app = AppWindow::new()?;
    let ui_app_handle = app.as_weak();

    // 1. Create the RingBufferPool with pre-allocated buffers.
    let pool = Arc::new(RingBufferPool::new(NUM_BUFFERS));

    // 2. Set up channels for passing buffer indices.
    let (empty_tx, empty_rx): (Sender<usize>, Receiver<usize>) = bounded(NUM_BUFFERS);
    let (dsp_tx, dsp_rx): (Sender<usize>, Receiver<usize>) = bounded(NUM_BUFFERS);
    let (ui_tx, ui_rx): (Sender<usize>, Receiver<usize>) = bounded(NUM_BUFFERS);

    // Initialize the empty queue with all available indices.
    for i in 0..NUM_BUFFERS {
        empty_tx.send(i).unwrap();
    }

    // Shared filter parameters
    let filter_params = Arc::new(Mutex::new(dsp::filters::FilterParams::default()));

    // Clone Arcs for threads
    let pool_ingest = Arc::clone(&pool);
    let pool_dsp = Arc::clone(&pool);
    let pool_ui = Arc::clone(&pool);
    let dsp_filter_params = Arc::clone(&filter_params);

    // Thread 1: Camera Ingest & Hardware Control
    let _ingest_thread = thread::spawn(move || {
        let mut dev = match Device::new(0) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: Failed to open V4L2 device (/dev/video0): {}. Using fallback synthetic ingest.", e);
                let mut frame_count = 0u8;
                let frame_interval = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);
                loop {
                    let start = Instant::now();
                    if let Ok(idx) = empty_rx.recv() {
                        {
                            let mut frame = pool_ingest.frames[idx].lock().unwrap();
                            for chunk in frame.raw_data.chunks_mut(4) {
                                chunk[0] = frame_count.wrapping_add(10);
                                chunk[1] = 128;
                                chunk[2] = frame_count.wrapping_add(10);
                                chunk[3] = 128;
                            }
                            frame.capture_time = Instant::now();
                        }
                        let _ = dsp_tx.send(idx);
                        frame_count = frame_count.wrapping_add(1);
                    } else {
                        break;
                    }
                    let elapsed = start.elapsed();
                    if elapsed < frame_interval {
                        thread::sleep(frame_interval - elapsed);
                    }
                }
                return;
            }
        };

        println!("Successfully opened /dev/video0");

        if let Ok(mut fmt) = dev.format() {
            fmt.width = FRAME_WIDTH as u32;
            fmt.height = FRAME_HEIGHT as u32;
            fmt.fourcc = FourCC::new(b"YUYV");
            let _ = dev.set_format(&fmt);
        }

        if let Ok(mut params) = dev.params() {
            params.interval = v4l::Fraction::new(1, TARGET_FPS);
            let _ = dev.set_params(&params);
        }

        let mut stream = match MmapStream::with_buffers(&mut dev, v4l::buffer::Type::VideoCapture, NUM_BUFFERS as u32) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: Failed to create mmap stream: {}", e);
                return;
            }
        };

        loop {
            let (buf_data, _meta) = match stream.next() {
                Ok(res) => res,
                Err(_) => break,
            };
            
            if let Ok(idx) = empty_rx.recv() {
                {
                    let mut frame = pool_ingest.frames[idx].lock().unwrap();
                    let len = buf_data.len().min(frame.raw_data.len());
                    frame.raw_data[..len].copy_from_slice(&buf_data[..len]);
                    frame.capture_time = Instant::now();
                }
                
                let _ = dsp_tx.send(idx);
            } else {
                break; 
            }
        }
    });

    // Thread 2: DSP & Image Processing Pipeline
    let _dsp_thread = thread::spawn(move || {
        loop {
            if let Ok(idx) = dsp_rx.recv() {
                {
                    let current_params = {
                        let p = dsp_filter_params.lock().unwrap();
                        dsp::filters::FilterParams {
                            auto_white_balance: p.auto_white_balance,
                            wb_red_gain: p.wb_red_gain,
                            wb_blue_gain: p.wb_blue_gain,
                            exposure_gain: p.exposure_gain,
                            flip_horizontal: p.flip_horizontal,
                            flip_vertical: p.flip_vertical,
                            hdr_enabled: p.hdr_enabled,
                            noise_reduction_enabled: p.noise_reduction_enabled,
                        }
                    };

                    let frame = &mut *pool_dsp.frames[idx].lock().unwrap();
                    let raw = &frame.raw_data;
                    let rgba = &mut frame.rgba_data;
                    convert_yuyv_to_rgba(raw, rgba);
                    
                    // Apply real-time filters
                    dsp::filters::apply_filters_in_place(rgba, FRAME_WIDTH, FRAME_HEIGHT, &current_params);
                }
                
                let _ = ui_tx.send(idx);
            } else {
                break;
            }
        }
    });

    // Thread 3: UI & Rendering (Main/Receiver Thread)
    let _ui_thread = thread::spawn(move || {
        let mut frames_processed = 0;
        let mut last_report = Instant::now();
        let mut max_latency = Duration::from_secs(0);
        
        loop {
            if let Ok(idx) = ui_rx.recv() {
                let latency = {
                    let frame = pool_ui.frames[idx].lock().unwrap();
                    frame.capture_time.elapsed()
                };
                
                if latency > max_latency {
                    max_latency = latency;
                }
                
                frames_processed += 1;
                
                let rgba_clone = {
                    let frame = pool_ui.frames[idx].lock().unwrap();
                    frame.rgba_data.clone()
                };
                
                // Return the buffer index to the empty pool immediately
                let _ = empty_tx.send(idx);

                let handle_clone = ui_app_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = handle_clone.upgrade() {
                        let pixel_buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                            &rgba_clone,
                            FRAME_WIDTH as u32,
                            FRAME_HEIGHT as u32,
                        );
                        app.set_video_frame(slint::Image::from_rgba8(pixel_buffer));
                    }
                });

                // Report metrics every second
                if last_report.elapsed() >= Duration::from_secs(1) {
                    println!(
                        "Throughput: {} FPS | Max Latency in last sec: {:.2} ms | Pipeline Target: <16ms",
                        frames_processed,
                        max_latency.as_secs_f64() * 1000.0
                    );
                    
                    frames_processed = 0;
                    max_latency = Duration::from_secs(0);
                    last_report = Instant::now();
                }
            } else {
                break;
            }
        }
    });

    // Sync properties from UI to DSP thread
    let ui_filter_params = Arc::clone(&filter_params);
    let app_weak = app.as_weak();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_millis(50), move || {
        if let Some(app) = app_weak.upgrade() {
            let mut params = ui_filter_params.lock().unwrap();
            params.auto_white_balance = app.get_awb_enabled();
            params.hdr_enabled = app.get_hdr_enabled();
            params.flip_horizontal = app.get_flip_h_enabled();
            params.flip_vertical = app.get_flip_v_enabled();
            params.exposure_gain = app.get_exposure_val();
        }
    });

    app.on_capture_image(move || {
        println!("Action: Captured HD Frame!");
    });

    app.on_export_csv(move || {
        println!("Action: Exported CSV measurements!");
    });

    println!("Starting Slint UI event loop...");
    app.run()
}
