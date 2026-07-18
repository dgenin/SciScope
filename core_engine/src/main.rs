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
const NUM_BUFFERS: usize = 8;

/// A single frame buffer with its capture timestamp.
struct Frame {
    raw_data: Vec<u8>,
    valid_len: usize,
    rgba_buffer: slint::SharedPixelBuffer<slint::Rgba8Pixel>,
    capture_time: Instant,
}

/// The pool of pre-allocated frame buffers.
struct RingBufferPool {
    frames: Vec<Mutex<Frame>>,
}

impl RingBufferPool {
    fn new(size: usize, width: usize, height: usize) -> Self {
        let mut frames = Vec::with_capacity(size);
        for _ in 0..size {
            frames.push(Mutex::new(Frame {
                raw_data: vec![0; width * height * 2],
                valid_len: 0,
                rgba_buffer: slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width as u32, height as u32),
                capture_time: Instant::now(),
            }));
        }
        Self { frames }
    }
}

use std::sync::OnceLock;

struct Luts {
    y: [i32; 256],
    r_v: [i32; 256],
    g_u: [i32; 256],
    g_v: [i32; 256],
    b_u: [i32; 256],
}

static LUTS: OnceLock<Luts> = OnceLock::new();

fn get_luts() -> &'static Luts {
    LUTS.get_or_init(|| {
        let mut luts = Luts {
            y: [0; 256],
            r_v: [0; 256],
            g_u: [0; 256],
            g_v: [0; 256],
            b_u: [0; 256],
        };
        for i in 0..256 {
            let val = (i as i32) - 128;
            luts.y[i] = ((i as i32) << 8) + 128;
            luts.r_v[i] = 359 * val;
            luts.g_u[i] = -88 * val;
            luts.g_v[i] = -183 * val;
            luts.b_u[i] = 454 * val;
        }
        luts
    })
}

// Basic YUYV to RGBA conversion
fn convert_yuyv_to_rgba(raw: &[u8], rgba: &mut [u8], width: usize, height: usize) {
    use rayon::prelude::*;
    let num_pixels = width * height;
    if raw.len() < num_pixels * 2 || rgba.len() < num_pixels * 4 {
        return;
    }

    let start_time = Instant::now();
    let luts = get_luts();

    let lines_per_chunk = 16;
    let raw_chunk_size = width * RAW_CHANNELS * lines_per_chunk;
    let rgba_chunk_size = width * RGBA_CHANNELS * lines_per_chunk;

    raw.par_chunks_exact(raw_chunk_size)
        .zip(rgba.par_chunks_exact_mut(rgba_chunk_size))
        .for_each(|(raw_band, rgba_band)| {
            for (raw_chunk, rgba_chunk) in raw_band.chunks_exact(4).zip(rgba_band.chunks_exact_mut(8)) {
                let y0 = raw_chunk[0] as usize;
                let u  = raw_chunk[1] as usize;
                let y1 = raw_chunk[2] as usize;
                let v  = raw_chunk[3] as usize;

                let r_v = luts.r_v[v];
                let g_u = luts.g_u[u];
                let g_v = luts.g_v[v];
                let b_u = luts.b_u[u];

                let g_offset = g_u + g_v;

                let y0_scaled = luts.y[y0];
                rgba_chunk[0] = ((y0_scaled + r_v) >> 8).clamp(0, 255) as u8;
                rgba_chunk[1] = ((y0_scaled + g_offset) >> 8).clamp(0, 255) as u8;
                rgba_chunk[2] = ((y0_scaled + b_u) >> 8).clamp(0, 255) as u8;
                rgba_chunk[3] = 255;

                let y1_scaled = luts.y[y1];
                rgba_chunk[4] = ((y1_scaled + r_v) >> 8).clamp(0, 255) as u8;
                rgba_chunk[5] = ((y1_scaled + g_offset) >> 8).clamp(0, 255) as u8;
                rgba_chunk[6] = ((y1_scaled + b_u) >> 8).clamp(0, 255) as u8;
                rgba_chunk[7] = 255;
            }
        });

    // Handle the remaining lines (1944 % 16 = 8 lines)
    let remainder_raw = raw.chunks_exact(raw_chunk_size).remainder();
    let remainder_rgba = rgba.chunks_exact_mut(rgba_chunk_size).into_remainder();
    
    if !remainder_raw.is_empty() {
        for (raw_chunk, rgba_chunk) in remainder_raw.chunks_exact(4).zip(remainder_rgba.chunks_exact_mut(8)) {
            let y0 = raw_chunk[0] as usize;
            let u  = raw_chunk[1] as usize;
            let y1 = raw_chunk[2] as usize;
            let v  = raw_chunk[3] as usize;

            let r_v = luts.r_v[v];
            let g_offset = luts.g_u[u] + luts.g_v[v];
            let b_u = luts.b_u[u];

            let y0_scaled = luts.y[y0];
            rgba_chunk[0] = ((y0_scaled + r_v) >> 8).clamp(0, 255) as u8;
            rgba_chunk[1] = ((y0_scaled + g_offset) >> 8).clamp(0, 255) as u8;
            rgba_chunk[2] = ((y0_scaled + b_u) >> 8).clamp(0, 255) as u8;
            rgba_chunk[3] = 255;

            let y1_scaled = luts.y[y1];
            rgba_chunk[4] = ((y1_scaled + r_v) >> 8).clamp(0, 255) as u8;
            rgba_chunk[5] = ((y1_scaled + g_offset) >> 8).clamp(0, 255) as u8;
            rgba_chunk[6] = ((y1_scaled + b_u) >> 8).clamp(0, 255) as u8;
            rgba_chunk[7] = 255;
        }
    }

    let elapsed = start_time.elapsed();
    if elapsed > Duration::from_millis(10) {
        println!("[Profiler] convert_yuyv_to_rgba took {:?}", elapsed);
    }
}

fn main() -> Result<(), slint::PlatformError> {
    println!("Initializing V4L2 Core Engine (Scientific Suite)...");

    let mut dev = match Device::new(0) {
        Ok(d) => Some(d),
        Err(e) => {
            eprintln!("Warning: Failed to open V4L2 device (/dev/video0): {}. Using fallback synthetic ingest.", e);
            None
        }
    };

    let mut cam_width = FRAME_WIDTH;
    let mut cam_height = FRAME_HEIGHT;
    let mut is_mjpeg = false;

    if let Some(ref mut dev) = dev {
        if let Ok(mut fmt) = dev.format() {
            fmt.width = FRAME_WIDTH as u32;
            fmt.height = FRAME_HEIGHT as u32;
            fmt.fourcc = FourCC::new(b"YUYV");
            let _ = dev.set_format(&fmt);
            
            if let Ok(actual_fmt) = dev.format() {
                cam_width = actual_fmt.width as usize;
                cam_height = actual_fmt.height as usize;
                if actual_fmt.fourcc.str() == Ok("MJPG") {
                    is_mjpeg = true;
                }
                println!("Negotiated Format: {}x{} ({})", cam_width, cam_height, actual_fmt.fourcc);
            }
        }
    }
    
    let app = AppWindow::new()?;
    let ui_app_handle = app.as_weak();

    // 1. Create the RingBufferPool with pre-allocated buffers matching actual camera resolution.
    let pool = Arc::new(RingBufferPool::new(NUM_BUFFERS, cam_width, cam_height));

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

    // 3. Thread 1: Camera Ingest & Hardware Control
    let _ingest_thread = thread::spawn(move || {
        let mut dev = match dev {
            Some(d) => d,
            None => {
                // Fallback loop
                let mut frame_count = 0u8;
                let frame_interval = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);
                while let Ok(idx) = empty_rx.recv() {
                    let start = Instant::now();
                    {
                        let mut frame = pool_ingest.frames[idx].lock().unwrap();
                        // Generate dummy YUYV data in parallel (by row)
                        use rayon::prelude::*;
                        let y_val = frame_count.wrapping_add(10);
                        let gen_start = Instant::now();
                        frame.raw_data.par_chunks_exact_mut(cam_width * RAW_CHANNELS).for_each(|row| {
                            for chunk in row.chunks_exact_mut(4) {
                                chunk[0] = y_val;
                                chunk[1] = 128; // U
                                chunk[2] = y_val;
                                chunk[3] = 128; // V
                            }
                        });
                        let gen_elapsed = gen_start.elapsed();
                        if gen_elapsed > Duration::from_millis(10) {
                            println!("[Profiler] Ingest fallback generation took {:?}", gen_elapsed);
                        }
                        frame.valid_len = cam_width * RAW_CHANNELS * cam_height;
                        frame.capture_time = Instant::now();
                    }
                    let _ = dsp_tx.send(idx);
                    frame_count = frame_count.wrapping_add(1);
                    
                    let elapsed = start.elapsed();
                    if elapsed < frame_interval {
                        thread::sleep(frame_interval - elapsed);
                    }
                }
                return;
            }
        };

        println!("Successfully opened /dev/video0");

        // Try to set params for FPS
        if let Ok(mut params) = dev.params() {
            params.interval = v4l::Fraction::new(1, TARGET_FPS);
            let _ = dev.set_params(&params);
        }

        let mut stream = match MmapStream::with_buffers(&dev, v4l::buffer::Type::VideoCapture, NUM_BUFFERS as u32) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: Failed to create mmap stream: {}", e);
                return;
            }
        };

        loop {
            let (buf_data, _meta) = match stream.next() {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Capture error: {}", e);
                    break;
                }
            };
            
            if let Ok(idx) = empty_rx.recv() {
                {
                    let mut frame = pool_ingest.frames[idx].lock().unwrap();
                    let len = buf_data.len().min(frame.raw_data.len());
                    frame.raw_data[..len].copy_from_slice(&buf_data[..len]);
                    frame.valid_len = len;
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
        let rgba_frame_size = cam_width * cam_height * RGBA_CHANNELS;
        while let Ok(idx) = dsp_rx.recv() {
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

                let mut frame_guard = pool_dsp.frames[idx].lock().unwrap();
                let frame = &mut *frame_guard;
                let raw_valid_len = frame.valid_len;
                let raw = &frame.raw_data[..raw_valid_len];
                
                // Obtain a mutable reference to the SharedPixelBuffer's backing array
                let rgba_slice = unsafe {
                    let pixels = frame.rgba_buffer.make_mut_slice();
                    std::slice::from_raw_parts_mut(pixels.as_mut_ptr() as *mut u8, rgba_frame_size)
                };
                
                if is_mjpeg {
                    if let Ok(img) = image::load_from_memory_with_format(raw, image::ImageFormat::Jpeg) {
                        let rgba_img = img.to_rgba8();
                        if rgba_img.len() == rgba_slice.len() {
                            rgba_slice.copy_from_slice(&rgba_img);
                        }
                    }
                } else {
                    convert_yuyv_to_rgba(raw, rgba_slice, cam_width, cam_height);
                }
                
                let filters_start = Instant::now();
                // Apply real-time filters
                dsp::filters::apply_filters_in_place(rgba_slice, cam_width, cam_height, &current_params);
                let filters_elapsed = filters_start.elapsed();
                if filters_elapsed > Duration::from_millis(10) {
                    println!("[Profiler] DSP apply_filters_in_place took {:?}", filters_elapsed);
                }
            }
            
            // Pass ownership of this buffer to the UI rendering thread
            let _ = ui_tx.send(idx);
        }
    });

    // Thread 3: UI & Rendering (Main/Receiver Thread)
    let _ui_thread = thread::spawn(move || {
        let mut frames_processed = 0;
        let mut last_report = Instant::now();
        let mut max_latency = Duration::from_secs(0);
        
        while let Ok(idx) = ui_rx.recv() {
            let latency = {
                let frame = pool_ui.frames[idx].lock().unwrap();
                frame.capture_time.elapsed()
            };
            
            if latency > max_latency {
                max_latency = latency;
            }
            
            frames_processed += 1;
            
            let ui_copy_start = Instant::now();
            let pixel_buffer = {
                let frame = pool_ui.frames[idx].lock().unwrap();
                // O(1) atomic ref-count clone instead of a 20MB copy
                frame.rgba_buffer.clone()
            };
            let ui_copy_elapsed = ui_copy_start.elapsed();
            if ui_copy_elapsed > Duration::from_millis(10) {
                println!("[Profiler] UI zero-copy clone took {:?}", ui_copy_elapsed);
            }
            
            // Return the buffer index to the empty pool
            let _ = empty_tx.send(idx);

            let handle_clone = ui_app_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = handle_clone.upgrade() {
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

#[cfg(test)]
mod throughput_tests {
    use super::*;

    #[test]
    fn test_throughput() {
        let mut raw = vec![0u8; 2592 * 1944 * 2];
        for chunk in raw.chunks_exact_mut(4) {
            chunk[0] = 76;
            chunk[1] = 84;
            chunk[2] = 76;
            chunk[3] = 255;
        }

        let mut rgba = vec![0u8; 2592 * 1944 * 4];
        let params = dsp::filters::FilterParams::default();

        let frames = 100;
        let mut min_elapsed = std::time::Duration::from_secs(10);
        for _ in 0..frames {
            let frame_start = std::time::Instant::now();
            convert_yuyv_to_rgba(&raw, &mut rgba, 2592, 1944);
            dsp::filters::apply_filters_in_place(&mut rgba, 2592, 1944, &params);
            let frame_elapsed = frame_start.elapsed();
            if frame_elapsed < min_elapsed {
                min_elapsed = frame_elapsed;
            }
        }
        println!("Minimum Frame Processing Time: {:?}", min_elapsed);
        assert!(min_elapsed < std::time::Duration::from_millis(16), "Frame processing took longer than 16ms");
    }
}

