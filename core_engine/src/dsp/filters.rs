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

pub fn apply_filters_in_place(rgba: &mut [u8], width: usize, height: usize, params: &FilterParams) {
    // Process pixels in parallel for color corrections
    rgba.par_chunks_exact_mut(4).for_each(|pixel| {
        let mut r = pixel[0] as f32;
        let mut g = pixel[1] as f32;
        let mut b = pixel[2] as f32;
        
        // Exposure Gain
        r *= params.exposure_gain;
        g *= params.exposure_gain;
        b *= params.exposure_gain;
        
        // White Balance
        if params.auto_white_balance {
            r *= params.wb_red_gain;
            b *= params.wb_blue_gain;
        }
        
        // Simple local contrast / HDR approximation
        if params.hdr_enabled {
            let lum = 0.299 * r + 0.587 * g + 0.114 * b;
            if lum > 0.0 {
                let factor = (lum / 255.0).powf(0.8) * 255.0 / lum;
                r *= factor;
                g *= factor;
                b *= factor;
            }
        }
        
        pixel[0] = r.clamp(0.0, 255.0) as u8;
        pixel[1] = g.clamp(0.0, 255.0) as u8;
        pixel[2] = b.clamp(0.0, 255.0) as u8;
    });
    
    // Basic Flip operations
    if params.flip_vertical {
        for y in 0..(height / 2) {
            let row1_idx = y * width * 4;
            let row2_idx = (height - 1 - y) * width * 4;
            for x in 0..(width * 4) {
                rgba.swap(row1_idx + x, row2_idx + x);
            }
        }
    }
    
    if params.flip_horizontal {
        for y in 0..height {
            let row_idx = y * width * 4;
            for x in 0..(width / 2) {
                let p1 = row_idx + x * 4;
                let p2 = row_idx + (width - 1 - x) * 4;
                for c in 0..4 {
                    rgba.swap(p1 + c, p2 + c);
                }
            }
        }
    }
}
