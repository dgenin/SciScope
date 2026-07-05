use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalibrationProfile {
    pub name: String,
    pub pixels_per_um: f32,
}

pub fn get_default_profiles() -> Vec<CalibrationProfile> {
    vec![
        CalibrationProfile { name: "4x Objective".into(), pixels_per_um: 1.2 },
        CalibrationProfile { name: "10x Objective".into(), pixels_per_um: 3.0 },
        CalibrationProfile { name: "40x Objective".into(), pixels_per_um: 12.0 },
        CalibrationProfile { name: "100x Objective".into(), pixels_per_um: 30.0 },
    ]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurement {
    pub label: String,
    pub length_um: f32,
}

pub fn calculate_distance_um(x1: f32, y1: f32, x2: f32, y2: f32, profile: &CalibrationProfile) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let pixels = (dx * dx + dy * dy).sqrt();
    pixels / profile.pixels_per_um
}
