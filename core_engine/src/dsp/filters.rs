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
            let x = i as f32 / 255.0;
            // Piecewise quadratic S-Curve
            let s_curve = if x < 0.5 {
                2.0 * x * x
            } else {
                1.0 - 2.0 * (1.0 - x) * (1.0 - x)
            };
            lut[i] = s_curve * 255.0 / (i as f32);
        }
        lut[0] = 0.0;
        lut
    })
}
pub fn apply_filters_in_place(rgb: &mut [u8], width: usize, height: usize, params: &FilterParams) {
    let needs_color_correction = (params.exposure_gain - 1.0).abs() > f32::EPSILON 
        || params.auto_white_balance 
        || params.hdr_enabled;

    if needs_color_correction {
        let row_bytes = width * 3;
        let hdr_lut = get_hdr_lut();
        
        // Process pixels in parallel by row to reduce Rayon scheduler overhead
        rgb.par_chunks_exact_mut(row_bytes).for_each(|row| {
            for pixel in row.chunks_exact_mut(3) {
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
        let row_bytes = width * 3;
        let (top, bottom) = rgb.split_at_mut((height / 2) * row_bytes);
        top.par_chunks_exact_mut(row_bytes)
           .zip(bottom.par_chunks_exact_mut(row_bytes).rev())
           .for_each(|(top_row, bottom_row)| {
               top_row.swap_with_slice(bottom_row);
           });
    }
    
    if params.flip_horizontal {
        let row_bytes = width * 3;
        rgb.par_chunks_exact_mut(row_bytes).for_each(|row| {
            for x in 0..(width / 2) {
                let p1 = x * 3;
                let p2 = (width - 1 - x) * 3;
                for c in 0..3 {
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
        let mut rgb = [100, 100, 100];
        let mut params = FilterParams::default();
        params.exposure_gain = 2.0;

        apply_filters_in_place(&mut rgb, 1, 1, &params);
        assert_eq!(rgb, [200, 200, 200]);
    }

    #[test]
    fn test_exposure_clamping() {
        let mut rgb = [200, 200, 200];
        let mut params = FilterParams::default();
        params.exposure_gain = 2.0;

        apply_filters_in_place(&mut rgb, 1, 1, &params);
        assert_eq!(rgb, [255, 255, 255]); // Clamped
    }

    #[test]
    fn test_flip_horizontal() {
        // 2x1 image
        let mut rgb = [
            10, 20, 30, // Pixel 0 (left)
            90, 80, 70, // Pixel 1 (right)
        ];
        let mut params = FilterParams::default();
        params.flip_horizontal = true;

        apply_filters_in_place(&mut rgb, 2, 1, &params);
        assert_eq!(
            rgb,
            [
                90, 80, 70, // Pixel 1 now on the left
                10, 20, 30, // Pixel 0 now on the right
            ]
        );
    }

    #[test]
    fn test_hdr_contrast() {
        // Create a synthetic gradient (test video stream row)
        // Dark pixel (10), Mid pixel (100), Bright pixel (240)
        let mut rgb = [
            10, 10, 10,
            100, 100, 100,
            240, 240, 240,
        ];
        
        let original_dark = rgb[0];
        let original_mid = rgb[3];
        let original_bright = rgb[6];
        
        let mut params = FilterParams::default();
        params.hdr_enabled = true;

        apply_filters_in_place(&mut rgb, 3, 1, &params);
        
        let hdr_dark = rgb[0];
        let hdr_mid = rgb[3];
        let hdr_bright = rgb[6];
        
        // Assert shadows are deepened
        assert!(hdr_dark < original_dark, "Dark regions should be deepened");
        
        // Assert highlights are boosted
        assert!(hdr_bright > original_bright, "Highlights should be boosted");
        
        // Measure global contrast
        let original_contrast = original_bright as f32 / original_dark as f32;
        let hdr_contrast = hdr_bright as f32 / hdr_dark as f32;
        
        // Since HDR is now an S-curve, it mathematically INCREASES global contrast ratio
        assert!(hdr_contrast > original_contrast, "S-Curve mathematically increases global contrast ratio");
        
        println!("Original -> Dark: {}, Mid: {}, Bright: {}, Contrast Ratio: {}", original_dark, original_mid, original_bright, original_contrast);
        println!("HDR -> Dark: {}, Mid: {}, Bright: {}, Contrast Ratio: {}", hdr_dark, hdr_mid, hdr_bright, hdr_contrast);
    }
}
