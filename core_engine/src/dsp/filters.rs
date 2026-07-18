use rayon::prelude::*;

pub struct FilterParams {
    pub auto_white_balance: bool,
    pub wb_red_gain: f32,
    pub wb_blue_gain: f32,
    pub exposure_gain: f32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub hdr_enabled: bool,
    pub noise_reduction_enabled: bool,
}

impl Default for FilterParams {
    fn default() -> Self {
        Self {
            auto_white_balance: false,
            wb_red_gain: 1.0,
            wb_blue_gain: 1.0,
            exposure_gain: 1.0,
            flip_horizontal: false,
            flip_vertical: false,
            hdr_enabled: false,
            noise_reduction_enabled: false,
        }
    }
}
use std::sync::OnceLock;

static HDR_LUT: OnceLock<[f32; 256]> = OnceLock::new();

fn get_hdr_lut() -> &'static [f32; 256] {
    HDR_LUT.get_or_init(|| {
        let mut lut = [0.0; 256];
        for i in 1..256 {
            lut[i] = (i as f32 / 255.0).powf(0.8) * 255.0 / (i as f32);
        }
        lut[0] = 0.0;
        lut
    })
}
pub fn apply_filters_in_place(rgba: &mut [u8], width: usize, height: usize, params: &FilterParams) {
    let needs_color_correction = (params.exposure_gain - 1.0).abs() > f32::EPSILON 
        || params.auto_white_balance 
        || params.hdr_enabled;

    if needs_color_correction {
        let row_bytes = width * 4;
        let hdr_lut = get_hdr_lut();
        
        // Process pixels in parallel by row to reduce Rayon scheduler overhead
        rgba.par_chunks_exact_mut(row_bytes).for_each(|row| {
            for pixel in row.chunks_exact_mut(4) {
                let mut r = pixel[0] as f32;
                let mut g = pixel[1] as f32;
                let mut b = pixel[2] as f32;
                
                // Exposure Gain
                if (params.exposure_gain - 1.0).abs() > f32::EPSILON {
                    r *= params.exposure_gain;
                    g *= params.exposure_gain;
                    b *= params.exposure_gain;
                }
                
                // White Balance
                if params.auto_white_balance {
                    r *= params.wb_red_gain;
                    b *= params.wb_blue_gain;
                }
                
                // Extremely fast HDR approximation via precomputed LUT
                if params.hdr_enabled {
                    let lum = (0.299 * r + 0.587 * g + 0.114 * b) as usize;
                    let factor = hdr_lut[lum.clamp(0, 255)];
                    r *= factor;
                    g *= factor;
                    b *= factor;
                }
                
                pixel[0] = r.clamp(0.0, 255.0) as u8;
                pixel[1] = g.clamp(0.0, 255.0) as u8;
                pixel[2] = b.clamp(0.0, 255.0) as u8;
            }
        });
    }
    
    // Basic Flip operations (Parallelized)
    if params.flip_vertical {
        let row_bytes = width * 4;
        let (top, bottom) = rgba.split_at_mut((height / 2) * row_bytes);
        top.par_chunks_exact_mut(row_bytes)
           .zip(bottom.par_chunks_exact_mut(row_bytes).rev())
           .for_each(|(top_row, bottom_row)| {
               top_row.swap_with_slice(bottom_row);
           });
    }
    
    if params.flip_horizontal {
        let row_bytes = width * 4;
        rgba.par_chunks_exact_mut(row_bytes).for_each(|row| {
            for x in 0..(width / 2) {
                let p1 = x * 4;
                let p2 = (width - 1 - x) * 4;
                for c in 0..4 {
                    row.swap(p1 + c, p2 + c);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exposure_gain() {
        let mut rgba = [100, 100, 100, 255];
        let mut params = FilterParams::default();
        params.exposure_gain = 2.0;

        apply_filters_in_place(&mut rgba, 1, 1, &params);
        assert_eq!(rgba, [200, 200, 200, 255]);
    }

    #[test]
    fn test_exposure_clamping() {
        let mut rgba = [200, 200, 200, 255];
        let mut params = FilterParams::default();
        params.exposure_gain = 2.0;

        apply_filters_in_place(&mut rgba, 1, 1, &params);
        assert_eq!(rgba, [255, 255, 255, 255]); // Clamped
    }

    #[test]
    fn test_flip_horizontal() {
        // 2x1 image
        let mut rgba = [
            10, 20, 30, 255, // Pixel 0 (left)
            90, 80, 70, 255, // Pixel 1 (right)
        ];
        let mut params = FilterParams::default();
        params.flip_horizontal = true;

        apply_filters_in_place(&mut rgba, 2, 1, &params);
        assert_eq!(
            rgba,
            [
                90, 80, 70, 255, // Pixel 1 now on the left
                10, 20, 30, 255, // Pixel 0 now on the right
            ]
        );
    }
}
