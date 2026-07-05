use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use v4l::Device;
use v4l::format::FourCC;
use v4l::io::traits::CaptureStream;
use v4l::io::mmap::Stream as MmapStream;
use v4l::video::Capture;

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

fn main() {
    println!("Initializing V4L2 Core Engine (AmScope MD500A profile)...");
    
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

    // Clone Arcs for threads
    let pool_ingest = Arc::clone(&pool);
    let pool_dsp = Arc::clone(&pool);
    let pool_ui = Arc::clone(&pool);

    // 3. Thread 1: Camera Ingest & Hardware Control
    let _ingest_thread = thread::spawn(move || {
        // Attempt to open /dev/video0. If it fails, fallback to a synthetic stream for testing.
        let mut dev = match Device::new(0) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: Failed to open V4L2 device (/dev/video0): {}. Using fallback synthetic ingest.", e);
                // Fallback loop
                let mut frame_count = 0u8;
                let frame_interval = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);
                while let Ok(idx) = empty_rx.recv() {
                    let start = Instant::now();
                    {
                        let mut frame = pool_ingest.frames[idx].lock().unwrap();
                        // Generate dummy YUYV data
                        for chunk in frame.raw_data.chunks_mut(4) {
                            chunk[0] = frame_count.wrapping_add(10);
                            chunk[1] = 128; // U
                            chunk[2] = frame_count.wrapping_add(10);
                            chunk[3] = 128; // V
                        }
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

        // Try to set format
        if let Ok(mut fmt) = dev.format() {
            fmt.width = FRAME_WIDTH as u32;
            fmt.height = FRAME_HEIGHT as u32;
            fmt.fourcc = FourCC::new(b"YUYV");
            if let Err(e) = dev.set_format(&fmt) {
                eprintln!("Warning: Could not set YUYV format: {}", e);
            } else {
                println!("Successfully configured YUYV format at {}x{}", FRAME_WIDTH, FRAME_HEIGHT);
            }
        }

        // Try to set params for FPS
        if let Ok(mut params) = dev.params() {
            params.interval = v4l::Fraction::new(1, TARGET_FPS);
            if let Err(e) = dev.set_params(&params) {
                eprintln!("Warning: Could not set FPS to {}: {}", TARGET_FPS, e);
            }
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
        while let Ok(idx) = dsp_rx.recv() {
            {
                let mut frame_guard = pool_dsp.frames[idx].lock().unwrap();
                let frame = &mut *frame_guard;
                // YUYV to RGBA Conversion without allocating
                let raw = &frame.raw_data;
                let rgba = &mut frame.rgba_data;
                convert_yuyv_to_rgba(raw, rgba);
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
                // Simulate uploading texture to GPU
                let _first_pixel_r = frame.rgba_data[0];
                frame.capture_time.elapsed()
            };
            
            if latency > max_latency {
                max_latency = latency;
            }
            
            frames_processed += 1;
            
            // Return the buffer index to the empty pool
            let _ = empty_tx.send(idx);

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

    // Run for 5 seconds to demonstrate the metrics
    thread::sleep(Duration::from_secs(5));
    println!("Test complete. Exiting.");
}
