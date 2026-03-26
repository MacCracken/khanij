/// Rate of physical weathering (simplified, relative scale 0-1).
/// Increases with temperature range and moisture.
#[must_use]
pub fn physical_weathering_rate(temp_range_celsius: f32, moisture_fraction: f32) -> f32 {
    (temp_range_celsius / 50.0 * moisture_fraction).clamp(0.0, 1.0)
}

/// Rate of chemical weathering (simplified). Increases with temperature and rainfall.
#[must_use]
pub fn chemical_weathering_rate(mean_temp_celsius: f32, annual_rainfall_mm: f32) -> f32 {
    let temp_factor = ((mean_temp_celsius + 10.0) / 40.0).clamp(0.0, 1.0);
    let rain_factor = (annual_rainfall_mm / 2000.0).clamp(0.0, 1.0);
    temp_factor * rain_factor
}

/// Erosion rate estimate (Revised Universal Soil Loss Equation, simplified).
/// Returns relative erosion rate (higher = more erosion).
#[must_use]
pub fn erosion_rate(rainfall_intensity: f32, slope_degrees: f32, vegetation_cover: f32) -> f32 {
    let slope_factor = (slope_degrees / 45.0).clamp(0.0, 1.0);
    let cover_factor = (1.0 - vegetation_cover).clamp(0.0, 1.0);
    rainfall_intensity * slope_factor * cover_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn more_moisture_more_weathering() {
        let dry = physical_weathering_rate(20.0, 0.2);
        let wet = physical_weathering_rate(20.0, 0.8);
        assert!(wet > dry);
    }

    #[test]
    fn warmer_faster_chemical() {
        let cold = chemical_weathering_rate(0.0, 1000.0);
        let hot = chemical_weathering_rate(30.0, 1000.0);
        assert!(hot > cold);
    }

    #[test]
    fn vegetation_reduces_erosion() {
        let bare = erosion_rate(50.0, 15.0, 0.0);
        let covered = erosion_rate(50.0, 15.0, 0.9);
        assert!(covered < bare);
    }

    #[test]
    fn flat_ground_no_erosion() {
        let e = erosion_rate(50.0, 0.0, 0.5);
        assert!(e.abs() < f32::EPSILON);
    }
}
