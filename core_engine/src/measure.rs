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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance_um() {
        let profile = CalibrationProfile {
            name: "Test".into(),
            pixels_per_um: 2.0,
        };

        // Distance in pixels is 10.0 (from 0,0 to 6,8)
        // 10.0 pixels / 2.0 pixels_per_um = 5.0 um
        let dist = calculate_distance_um(0.0, 0.0, 6.0, 8.0, &profile);
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn test_get_default_profiles() {
        let profiles = get_default_profiles();
        assert_eq!(profiles.len(), 4);
        assert_eq!(profiles[0].name, "4x Objective");
    }
}
