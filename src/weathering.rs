use hisab::calc;

/// Rate of physical weathering (relative scale 0-1).
/// Uses numerical integration over a temperature-moisture interaction model.
#[must_use]
pub fn physical_weathering_rate(temp_range_celsius: f32, moisture_fraction: f32) -> f32 {
    let tr = temp_range_celsius as f64;
    let mf = moisture_fraction as f64;
    if tr <= 0.0 || mf <= 0.0 {
        return 0.0;
    }
    // Integrate the freeze-thaw contribution over the temperature range.
    // The integrand models increasing damage with wider thermal cycling.
    let rate = calc::integral_simpson(|t| (t / tr).powi(2) * mf, 0.0, tr, 20)
        .unwrap_or(0.0);
    // Normalise: max rate at temp_range=50, moisture=1 → integral ≈ 50/3 ≈ 16.67
    let normalised = rate / (50.0 / 3.0);
    (normalised as f32).clamp(0.0, 1.0)
}

/// Rate of chemical weathering. Uses Arrhenius-style exponential temperature
/// dependence combined with rainfall, computed via hisab calculus.
#[must_use]
pub fn chemical_weathering_rate(mean_temp_celsius: f32, annual_rainfall_mm: f32) -> f32 {
    let temp = mean_temp_celsius as f64;
    let rain = annual_rainfall_mm as f64;
    // Arrhenius-like: rate doubles per ~10°C increase.
    // Integrate the exponential curve from 0 to temp+10 and normalise.
    let temp_factor = calc::integral_simpson(
        |t| (0.07 * t).exp(),
        0.0,
        (temp + 10.0).clamp(0.0, 50.0),
        20,
    )
    .unwrap_or(0.0);
    // Normalise against max (50°C → integral of exp(0.07t) from 0..50 ≈ 47.2)
    let temp_norm = (temp_factor / 47.2).min(1.0);
    let rain_factor = (rain / 2000.0).clamp(0.0, 1.0);
    (temp_norm * rain_factor) as f32
}

/// Erosion rate estimate (Revised Universal Soil Loss Equation, improved).
/// Uses hisab lerp for smooth interpolation of cover factor.
#[must_use]
pub fn erosion_rate(rainfall_intensity: f32, slope_degrees: f32, vegetation_cover: f32) -> f32 {
    let slope_rad = (slope_degrees as f64).to_radians();
    // RUSLE slope factor: sin(θ) gives a more realistic nonlinear response than linear θ/45
    let slope_factor = slope_rad.sin().clamp(0.0, 1.0);
    // Cover factor: exponential decay — dense cover is much more effective than sparse
    let cover = vegetation_cover as f64;
    let cover_factor = calc::lerp(1.0, (-3.0_f64 * cover).exp(), cover);
    (rainfall_intensity as f64 * slope_factor * cover_factor) as f32
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

    #[test]
    fn zero_temp_range_no_physical_weathering() {
        let rate = physical_weathering_rate(0.0, 1.0);
        assert!(rate.abs() < 0.01);
    }

    #[test]
    fn max_conditions_high_rate() {
        let rate = physical_weathering_rate(50.0, 1.0);
        assert!(rate > 0.9);
    }
}

// ---------------------------------------------------------------------------
// Chemistry-gated weathering (kimiya)
// ---------------------------------------------------------------------------

/// Mineral dissolution rate at a given temperature using true Arrhenius kinetics
/// from kimiya. Returns the rate constant k in s⁻¹.
///
/// - `pre_exponential`: frequency factor A (s⁻¹)
/// - `activation_energy_j`: Eₐ in J/mol (typical silicate: 50-80 kJ/mol)
/// - `temperature_k`: temperature in kelvin
///
/// Requires the `chemistry` feature.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn arrhenius_weathering_rate(
    pre_exponential: f64,
    activation_energy_j: f64,
    temperature_k: f64,
) -> Option<f64> {
    kimiya::arrhenius_rate(pre_exponential, activation_energy_j, temperature_k).ok()
}

/// Mineral dissolution half-life given a first-order rate constant.
/// Returns time in seconds for half of the mineral to dissolve.
///
/// Requires the `chemistry` feature.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn dissolution_half_life(rate_constant: f64) -> Option<f64> {
    kimiya::kinetics::half_life_first_order(rate_constant).ok()
}

/// Remaining mineral fraction after time `t` seconds at rate constant `k`,
/// assuming first-order dissolution kinetics: C(t) = C₀ · e^(-kt).
///
/// Requires the `chemistry` feature.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn remaining_mineral_fraction(initial: f64, rate_constant: f64, time_seconds: f64) -> f64 {
    kimiya::kinetics::first_order_concentration(initial, rate_constant, time_seconds)
}

#[cfg(all(test, feature = "chemistry"))]
mod chemistry_tests {
    use super::*;

    #[test]
    fn arrhenius_rate_increases_with_temperature() {
        // Typical silicate dissolution: Ea ≈ 60 kJ/mol
        let cold = arrhenius_weathering_rate(1e10, 60_000.0, 283.15).unwrap(); // 10°C
        let hot = arrhenius_weathering_rate(1e10, 60_000.0, 313.15).unwrap();  // 40°C
        assert!(hot > cold);
    }

    #[test]
    fn dissolution_half_life_positive() {
        let k = arrhenius_weathering_rate(1e10, 60_000.0, 298.15).unwrap();
        let t_half = dissolution_half_life(k).unwrap();
        assert!(t_half > 0.0);
    }

    #[test]
    fn remaining_fraction_decreases() {
        let k = 0.001; // s⁻¹
        let early = remaining_mineral_fraction(1.0, k, 100.0);
        let late = remaining_mineral_fraction(1.0, k, 1000.0);
        assert!(late < early);
        assert!(early < 1.0);
        assert!(late > 0.0);
    }
}
