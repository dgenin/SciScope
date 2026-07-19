pub mod dsp;
pub mod measure;

use std::sync::OnceLock;
use rayon::prelude::*;

pub const RAW_CHANNELS: usize = 2; // YUYV (2 bytes per pixel)
pub const RGB_CHANNELS: usize = 3; // RGB

pub struct Luts {
    pub y: [i32; 256],
    pub r_v: [i32; 256],
    pub g_u: [i32; 256],
    pub g_v: [i32; 256],
    pub b_u: [i32; 256],
}

static LUTS: OnceLock<Luts> = OnceLock::new();

pub fn get_luts() -> &'static Luts {
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
pub fn convert_yuyv_to_rgb(raw: &[u8], rgb: &mut [u8], width: usize, height: usize) {
    let num_pixels = width * height;
    if raw.len() < num_pixels * 2 || rgb.len() < num_pixels * 3 {
        return;
    }

    let start_time = std::time::Instant::now();
    let luts = get_luts();

    // 1944 is evenly divisible by 8 (1944 / 8 = 243).
    // Using 8 avoids remainder chunks and duplicate processing code.
    let lines_per_chunk = 8;
    let raw_chunk_size = width * RAW_CHANNELS * lines_per_chunk;
    let rgb_chunk_size = width * RGB_CHANNELS * lines_per_chunk;

    rgb.par_chunks_exact_mut(rgb_chunk_size)
        .zip(raw.par_chunks_exact(raw_chunk_size))
        .for_each(|(rgb_band, raw_band)| {
            for (raw_chunk, rgb_chunk) in raw_band.chunks_exact(4).zip(rgb_band.chunks_exact_mut(6)) {
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
                rgb_chunk[0] = ((y0_scaled + r_v) >> 8).clamp(0, 255) as u8;
                rgb_chunk[1] = ((y0_scaled + g_offset) >> 8).clamp(0, 255) as u8;
                rgb_chunk[2] = ((y0_scaled + b_u) >> 8).clamp(0, 255) as u8;

                let y1_scaled = luts.y[y1];
                rgb_chunk[3] = ((y1_scaled + r_v) >> 8).clamp(0, 255) as u8;
                rgb_chunk[4] = ((y1_scaled + g_offset) >> 8).clamp(0, 255) as u8;
                rgb_chunk[5] = ((y1_scaled + b_u) >> 8).clamp(0, 255) as u8;
            }
        });

    let elapsed = start_time.elapsed();
    if elapsed > std::time::Duration::from_millis(10) {
        println!("[Profiler] convert_yuyv_to_rgb took {:?}", elapsed);
    }
}
