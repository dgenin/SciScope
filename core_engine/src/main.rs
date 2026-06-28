use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

const FRAME_WIDTH: usize = 3840;
const FRAME_HEIGHT: usize = 2160;
const CHANNELS: usize = 4; // RGBA
const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * CHANNELS;
const NUM_BUFFERS: usize = 4;
const TARGET_FPS: u64 = 60;
const FRAME_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);

/// A single frame buffer with its capture timestamp.
struct Frame {
    data: Vec<u8>,
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
                data: vec![0; FRAME_SIZE],
                capture_time: Instant::now(),
            }));
        }
        Self { frames }
    }
}

fn main() {
    println!("Initializing Core Engine Prototype...");
    
    // 1. Create the RingBufferPool with pre-allocated 4K RGBA buffers.
    let pool = Arc::new(RingBufferPool::new(NUM_BUFFERS));

    // 2. Set up channels for passing buffer indices.
    // We use bounded sync_channels with capacity equal to NUM_BUFFERS.
    let (empty_tx, empty_rx): (SyncSender<usize>, Receiver<usize>) = sync_channel(NUM_BUFFERS);
    let (dsp_tx, dsp_rx): (SyncSender<usize>, Receiver<usize>) = sync_channel(NUM_BUFFERS);
    let (ui_tx, ui_rx): (SyncSender<usize>, Receiver<usize>) = sync_channel(NUM_BUFFERS);

    // Initialize the empty queue with all available indices.
    for i in 0..NUM_BUFFERS {
        empty_tx.send(i).unwrap();
    }

    // Clone Arcs for threads
    let pool_ingest = Arc::clone(&pool);
    let pool_dsp = Arc::clone(&pool);
    let pool_ui = Arc::clone(&pool);

    // 3. Thread 1: Camera Ingest & Hardware Control (SyntheticCamera)
    let ingest_thread = thread::spawn(move || {
        let mut frame_count = 0u8;
        loop {
            let start = Instant::now();
            
            // Acquire an empty buffer index. 
            if let Ok(idx) = empty_rx.recv() {
                {
                    let mut frame = pool_ingest.frames[idx].lock().unwrap();
                    // Simulate filling the buffer with a shifting color gradient (dummy data)
                    frame.data[0] = frame_count.wrapping_add(10);
                    frame.data[1] = frame_count.wrapping_add(50);
                    frame.data[2] = frame_count.wrapping_add(100);
                    frame.data[3] = 255;
                    
                    frame.capture_time = Instant::now();
                }
                
                // Pass ownership of this buffer to the DSP pipeline
                let _ = dsp_tx.send(idx);
                frame_count = frame_count.wrapping_add(1);
            } else {
                break; // Channel closed
            }

            // Maintain strict 60 FPS
            let elapsed = start.elapsed();
            if elapsed < FRAME_INTERVAL {
                thread::sleep(FRAME_INTERVAL - elapsed);
            }
        }
    });

    // Thread 2: DSP & Image Processing Pipeline
    let dsp_thread = thread::spawn(move || {
        loop {
            if let Ok(idx) = dsp_rx.recv() {
                {
                    let mut frame = pool_dsp.frames[idx].lock().unwrap();
                    // Simulate DSP workload (dummy op)
                    frame.data[0] = 255 - frame.data[0];
                    frame.data[1] = 255 - frame.data[1];
                    frame.data[2] = 255 - frame.data[2];
                }
                
                // Pass ownership of this buffer to the UI rendering thread
                let _ = ui_tx.send(idx);
            } else {
                break;
            }
        }
    });

    // Thread 3: UI & Rendering (Main/Receiver Thread)
    let ui_thread = thread::spawn(move || {
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
            } else {
                break;
            }
        }
    });

    // Run for 5 seconds to demonstrate the metrics
    thread::sleep(Duration::from_secs(5));
    println!("Test complete. Exiting.");
}
