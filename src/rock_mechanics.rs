//! Rock mechanics — seismic velocities, failure criteria, elastic properties,
//! and slope stability powered by the dravya materials science crate.
//!
//! Requires the `mechanics` feature.

use dravya::{Material, StressTensor};

// ---------------------------------------------------------------------------
// Rock elastic property presets
// ---------------------------------------------------------------------------

/// Create a dravya `Material` with typical granite elastic properties.
#[must_use]
pub fn granite_material() -> Material {
    Material {
        name: "Granite".into(),
        youngs_modulus: 50e9,
        poisson_ratio: 0.25,
        yield_strength: 150e6,
        ultimate_tensile_strength: 170e6,
        density: 2700.0,
        thermal_expansion: 8e-6,
    }
}

/// Create a dravya `Material` with typical basalt elastic properties.
#[must_use]
pub fn basalt_material() -> Material {
    Material {
        name: "Basalt".into(),
        youngs_modulus: 70e9,
        poisson_ratio: 0.27,
        yield_strength: 200e6,
        ultimate_tensile_strength: 250e6,
        density: 3000.0,
        thermal_expansion: 6e-6,
    }
}

/// Create a dravya `Material` with typical limestone elastic properties.
#[must_use]
pub fn limestone_material() -> Material {
    Material {
        name: "Limestone".into(),
        youngs_modulus: 30e9,
        poisson_ratio: 0.30,
        yield_strength: 60e6,
        ultimate_tensile_strength: 80e6,
        density: 2500.0,
        thermal_expansion: 8e-6,
    }
}

/// Create a dravya `Material` with typical sandstone elastic properties.
#[must_use]
pub fn sandstone_material() -> Material {
    Material {
        name: "Sandstone".into(),
        youngs_modulus: 15e9,
        poisson_ratio: 0.20,
        yield_strength: 40e6,
        ultimate_tensile_strength: 50e6,
        density: 2300.0,
        thermal_expansion: 11e-6,
    }
}

/// Create a dravya `Material` with typical marble elastic properties.
#[must_use]
pub fn marble_material() -> Material {
    Material {
        name: "Marble".into(),
        youngs_modulus: 55e9,
        poisson_ratio: 0.25,
        yield_strength: 80e6,
        ultimate_tensile_strength: 100e6,
        density: 2700.0,
        thermal_expansion: 7e-6,
    }
}

/// Create a dravya `Material` with typical shale elastic properties.
#[must_use]
pub fn shale_material() -> Material {
    Material {
        name: "Shale".into(),
        youngs_modulus: 10e9,
        poisson_ratio: 0.25,
        yield_strength: 30e6,
        ultimate_tensile_strength: 40e6,
        density: 2400.0,
        thermal_expansion: 10e-6,
    }
}

/// Create a dravya `Material` with typical quartzite elastic properties.
#[must_use]
pub fn quartzite_material() -> Material {
    Material {
        name: "Quartzite".into(),
        youngs_modulus: 80e9,
        poisson_ratio: 0.15,
        yield_strength: 250e6,
        ultimate_tensile_strength: 300e6,
        density: 2650.0,
        thermal_expansion: 12e-6,
    }
}

/// Create a dravya `Material` with typical gneiss elastic properties.
#[must_use]
pub fn gneiss_material() -> Material {
    Material {
        name: "Gneiss".into(),
        youngs_modulus: 60e9,
        poisson_ratio: 0.25,
        yield_strength: 160e6,
        ultimate_tensile_strength: 200e6,
        density: 2700.0,
        thermal_expansion: 7e-6,
    }
}

// ---------------------------------------------------------------------------
// Seismic velocities
// ---------------------------------------------------------------------------

/// P-wave (compressional) velocity from rock material properties.
///
/// Vp = √((K + 4G/3) / ρ)
///
/// Returns velocity in m/s.
#[must_use]
pub fn p_wave_velocity(material: &Material) -> f64 {
    let k = material.bulk_modulus();
    let g = material.shear_modulus();
    ((k + 4.0 * g / 3.0) / material.density).sqrt()
}

/// S-wave (shear) velocity from rock material properties.
///
/// Vs = √(G / ρ)
///
/// Returns velocity in m/s.
#[must_use]
pub fn s_wave_velocity(material: &Material) -> f64 {
    (material.shear_modulus() / material.density).sqrt()
}

/// Vp/Vs ratio — diagnostic for rock type and fluid content.
///
/// - Dry rock: ~1.5-1.7
/// - Saturated rock: ~1.7-2.0
/// - Unconsolidated sediment: > 2.0
#[must_use]
pub fn vp_vs_ratio(material: &Material) -> f64 {
    let vp = p_wave_velocity(material);
    let vs = s_wave_velocity(material);
    if vs > 0.0 { vp / vs } else { f64::INFINITY }
}

/// Poisson's ratio from Vp/Vs ratio.
///
/// ν = (R² - 2) / (2(R² - 1))  where R = Vp/Vs
#[must_use]
pub fn poisson_from_velocities(vp: f64, vs: f64) -> Option<f64> {
    if vs <= 0.0 {
        return None;
    }
    let r2 = (vp / vs).powi(2);
    if r2 <= 1.0 {
        return None;
    }
    Some((r2 - 2.0) / (2.0 * (r2 - 1.0)))
}

// ---------------------------------------------------------------------------
// Mohr-Coulomb failure
// ---------------------------------------------------------------------------

/// Mohr-Coulomb shear strength of a rock at a given normal stress.
///
/// τ = c + σ_n · tan(φ)
///
/// - `cohesion`: c in Pa
/// - `friction_angle_rad`: φ in radians (typical rock: 30-45°)
/// - `normal_stress`: σ_n in Pa (compressive positive)
///
/// Returns shear strength τ in Pa.
#[must_use]
pub fn mohr_coulomb_strength(cohesion: f64, friction_angle_rad: f64, normal_stress: f64) -> f64 {
    cohesion + normal_stress * friction_angle_rad.tan()
}

/// Check if a stress state exceeds Mohr-Coulomb failure criterion.
///
/// Uses the maximum shear stress and hydrostatic stress from the tensor.
/// Returns `true` if the rock has failed.
#[must_use]
pub fn mohr_coulomb_failure(stress: &StressTensor, cohesion: f64, friction_angle_rad: f64) -> bool {
    let tau_max = stress.max_shear();
    let sigma_n = stress.hydrostatic();
    tau_max > mohr_coulomb_strength(cohesion, friction_angle_rad, sigma_n)
}

/// Mohr-Coulomb factor of safety.
///
/// FoS = τ_strength / τ_applied
#[must_use]
pub fn mohr_coulomb_safety_factor(
    stress: &StressTensor,
    cohesion: f64,
    friction_angle_rad: f64,
) -> f64 {
    let tau_max = stress.max_shear();
    if tau_max <= 0.0 {
        return f64::INFINITY;
    }
    let sigma_n = stress.hydrostatic();
    mohr_coulomb_strength(cohesion, friction_angle_rad, sigma_n) / tau_max
}

/// Convert Mohr-Coulomb parameters to Drucker-Prager parameters
/// for use with dravya's `drucker_prager_check`.
///
/// Returns `(alpha, k)` for the Drucker-Prager yield surface.
#[must_use]
pub fn mohr_coulomb_to_drucker_prager(friction_angle_rad: f64, cohesion: f64) -> (f64, f64) {
    dravya::yield_criteria::drucker_prager_from_mohr_coulomb(friction_angle_rad, cohesion)
}

// ---------------------------------------------------------------------------
// Brittle-ductile transition
// ---------------------------------------------------------------------------

/// Failure mode of rock under given stress conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureMode {
    /// Rock fractures along planes (shallow, low confining pressure).
    Brittle,
    /// Rock deforms plastically (deep, high confining pressure).
    Ductile,
    /// Stress is below failure threshold.
    Stable,
}

/// Classify failure mode based on confining pressure and material properties.
///
/// At low confining pressure, rocks fail in a brittle manner (Mohr-Coulomb).
/// At high confining pressure, the Mohr-Coulomb strength exceeds the von Mises
/// yield strength, and rocks deform ductilely.
///
/// - `confining_pressure`: σ₃ in Pa
/// - `differential_stress`: (σ₁ - σ₃) in Pa
/// - `cohesion`: Mohr-Coulomb cohesion in Pa
/// - `friction_angle_rad`: Mohr-Coulomb friction angle in radians
/// - `yield_strength`: von Mises yield strength in Pa
#[must_use]
pub fn classify_failure_mode(
    confining_pressure: f64,
    differential_stress: f64,
    cohesion: f64,
    friction_angle_rad: f64,
    yield_strength: f64,
) -> FailureMode {
    let mc_strength = mohr_coulomb_strength(cohesion, friction_angle_rad, confining_pressure);
    let is_brittle_failure = differential_stress / 2.0 > mc_strength;
    let is_ductile_failure = differential_stress > yield_strength;

    if is_ductile_failure {
        FailureMode::Ductile
    } else if is_brittle_failure {
        FailureMode::Brittle
    } else {
        FailureMode::Stable
    }
}

/// Estimate the brittle-ductile transition depth for a given rock type.
///
/// Finds the depth at which the Mohr-Coulomb strength (increasing with
/// confining pressure) exceeds the von Mises yield strength.
///
/// - `material`: rock material properties
/// - `cohesion`: Mohr-Coulomb c in Pa
/// - `friction_angle_rad`: Mohr-Coulomb φ in radians
/// - `gravity`: m/s² (9.81)
///
/// Returns depth in metres.
#[must_use]
pub fn brittle_ductile_transition_depth(
    material: &Material,
    cohesion: f64,
    friction_angle_rad: f64,
    gravity: f64,
) -> f64 {
    // At the transition, Mohr-Coulomb strength = yield strength / 2
    // c + ρ·g·z · tan(φ) = σ_y / 2
    // z = (σ_y/2 - c) / (ρ·g·tan(φ))
    let numerator = material.yield_strength / 2.0 - cohesion;
    let denominator = material.density * gravity * friction_angle_rad.tan();
    if denominator <= 0.0 || numerator <= 0.0 {
        return 0.0;
    }
    numerator / denominator
}

// ---------------------------------------------------------------------------
// Slope stability
// ---------------------------------------------------------------------------

/// Factor of safety for an infinite slope (simplified Mohr-Coulomb).
///
/// FoS = (c + γ·z·cos²α·tan(φ)) / (γ·z·sinα·cosα)
///
/// - `cohesion`: c in Pa
/// - `friction_angle_rad`: φ in radians
/// - `unit_weight`: γ = ρ·g in N/m³
/// - `depth_m`: z, depth to failure plane in metres
/// - `slope_angle_rad`: α, slope angle in radians
///
/// Returns the factor of safety (> 1.0 = stable).
#[must_use]
pub fn infinite_slope_safety_factor(
    cohesion: f64,
    friction_angle_rad: f64,
    unit_weight: f64,
    depth_m: f64,
    slope_angle_rad: f64,
) -> f64 {
    let sin_a = slope_angle_rad.sin();
    let cos_a = slope_angle_rad.cos();
    let driving = unit_weight * depth_m * sin_a * cos_a;
    if driving <= 0.0 {
        return f64::INFINITY;
    }
    let resisting = cohesion + unit_weight * depth_m * cos_a.powi(2) * friction_angle_rad.tan();
    resisting / driving
}

// ---------------------------------------------------------------------------
// Seismic velocity-depth profile (requires thermodynamics + mechanics)
// ---------------------------------------------------------------------------

/// Seismic P-wave velocity at depth, accounting for temperature effects.
///
/// Velocity decreases with temperature as elastic moduli soften:
/// Vp(T) = Vp_0 × (1 - α·(T - T_ref))
///
/// where α ≈ 0.0004 per °C (typical for crustal rocks).
///
/// - `material`: rock material properties
/// - `temperature_c`: temperature at depth in °C
/// - `reference_temp_c`: reference temperature (typically 20°C)
///
/// Requires the `mechanics` feature (thermodynamics optional — caller provides T).
#[must_use]
pub fn p_wave_at_temperature(
    material: &Material,
    temperature_c: f64,
    reference_temp_c: f64,
) -> f64 {
    let vp0 = p_wave_velocity(material);
    let alpha = 0.0004; // velocity reduction coefficient per °C
    let correction = 1.0 - alpha * (temperature_c - reference_temp_c);
    vp0 * correction.max(0.5) // never reduce below 50% of reference
}

/// Seismic S-wave velocity at depth with temperature correction.
#[must_use]
pub fn s_wave_at_temperature(
    material: &Material,
    temperature_c: f64,
    reference_temp_c: f64,
) -> f64 {
    let vs0 = s_wave_velocity(material);
    let alpha = 0.0005; // shear modulus more sensitive to temperature
    let correction = 1.0 - alpha * (temperature_c - reference_temp_c);
    vs0 * correction.max(0.5)
}

/// Build a velocity-depth profile.
///
/// - `material`: rock material properties
/// - `surface_temp_c`: surface temperature in °C
/// - `gradient_c_per_km`: geothermal gradient (typical: 25°C/km)
/// - `max_depth_km`: maximum depth in km
/// - `steps`: number of depth points
///
/// Returns `Vec<(depth_km, Vp_m_s, Vs_m_s)>`.
#[must_use]
pub fn velocity_depth_profile(
    material: &Material,
    surface_temp_c: f64,
    gradient_c_per_km: f64,
    max_depth_km: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let step_size = max_depth_km / steps as f64;
    (0..=steps)
        .map(|i| {
            let depth = step_size * i as f64;
            let temp = surface_temp_c + gradient_c_per_km * depth;
            let vp = p_wave_at_temperature(material, temp, surface_temp_c);
            let vs = s_wave_at_temperature(material, temp, surface_temp_c);
            (depth, vp, vs)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Weathering-mechanics feedback
// ---------------------------------------------------------------------------

/// Degraded material properties after weathering.
///
/// Weathering reduces Young's modulus, yield strength, and increases effective
/// porosity. The `weathering_fraction` (0.0 = fresh, 1.0 = fully weathered)
/// controls the degradation.
///
/// Returns a new `Material` with reduced mechanical properties.
#[must_use]
pub fn weathered_material(material: &Material, weathering_fraction: f64) -> Material {
    let w = weathering_fraction.clamp(0.0, 1.0);
    Material {
        name: format!("{} (weathered {:.0}%)", material.name, w * 100.0),
        youngs_modulus: material.youngs_modulus * (1.0 - 0.9 * w), // up to 90% reduction
        poisson_ratio: material.poisson_ratio * (1.0 + 0.3 * w),   // slight increase
        yield_strength: material.yield_strength * (1.0 - 0.95 * w), // nearly total loss
        ultimate_tensile_strength: material.ultimate_tensile_strength * (1.0 - 0.95 * w),
        density: material.density * (1.0 - 0.15 * w), // slight decrease
        thermal_expansion: material.thermal_expansion * (1.0 + 0.5 * w), // increase
    }
}

/// Estimate how many years of weathering until rock fails under its own weight.
///
/// Finds when the degraded yield strength falls below the lithostatic stress
/// at a given depth.
///
/// - `material`: fresh rock properties
/// - `depth_m`: depth of interest in metres
/// - `gravity`: m/s² (9.81)
/// - `weathering_rate`: fraction weathered per year (e.g., 1e-6 for slow granite)
///
/// Returns time to failure in years, or `None` if the rock can sustain the load
/// indefinitely.
#[must_use]
pub fn time_to_weathering_failure(
    material: &Material,
    depth_m: f64,
    gravity: f64,
    weathering_rate: f64,
) -> Option<f64> {
    if weathering_rate <= 0.0 {
        return None;
    }
    let stress = material.density * gravity * depth_m;
    // Find w where yield_strength * (1 - 0.95w) = stress
    // w = (1 - stress/yield_strength) / 0.95
    let w_fail = (1.0 - stress / material.yield_strength) / 0.95;
    if w_fail <= 0.0 {
        Some(0.0) // already failed
    } else if w_fail >= 1.0 {
        None // never fails
    } else {
        Some(w_fail / weathering_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_6;

    // --- Seismic velocity tests ---

    #[test]
    fn granite_vp_realistic() {
        let mat = granite_material();
        let vp = p_wave_velocity(&mat);
        // Granite Vp typically 4500-6500 m/s depending on E
        assert!(
            vp > 4000.0 && vp < 7000.0,
            "Granite Vp should be ~4500-6500, got {vp}"
        );
    }

    #[test]
    fn granite_vs_realistic() {
        let mat = granite_material();
        let vs = s_wave_velocity(&mat);
        // Granite Vs typically 3000-3500 m/s
        assert!(
            vs > 2500.0 && vs < 4000.0,
            "Granite Vs should be ~3000-3500, got {vs}"
        );
    }

    #[test]
    fn vp_always_greater_than_vs() {
        for mat in [
            granite_material(),
            basalt_material(),
            limestone_material(),
            sandstone_material(),
            marble_material(),
            shale_material(),
            quartzite_material(),
            gneiss_material(),
        ] {
            let vp = p_wave_velocity(&mat);
            let vs = s_wave_velocity(&mat);
            assert!(vp > vs, "Vp should always exceed Vs for {}", mat.name);
        }
    }

    #[test]
    fn vp_vs_ratio_reasonable() {
        let mat = granite_material();
        let ratio = vp_vs_ratio(&mat);
        // For ν=0.25, Vp/Vs = √3 ≈ 1.732
        assert!(
            ratio > 1.5 && ratio < 2.5,
            "Vp/Vs should be ~1.7, got {ratio}"
        );
    }

    #[test]
    fn poisson_from_velocities_roundtrip() {
        let mat = granite_material();
        let vp = p_wave_velocity(&mat);
        let vs = s_wave_velocity(&mat);
        let nu = poisson_from_velocities(vp, vs).unwrap();
        assert!(
            (nu - mat.poisson_ratio).abs() < 0.01,
            "Recovered ν should match, got {nu}"
        );
    }

    // --- Mohr-Coulomb tests ---

    #[test]
    fn mohr_coulomb_strength_increases_with_pressure() {
        let phi = std::f64::consts::FRAC_PI_4 * 0.75; // ~34°
        let s1 = mohr_coulomb_strength(10e6, phi, 10e6);
        let s2 = mohr_coulomb_strength(10e6, phi, 50e6);
        assert!(s2 > s1);
    }

    #[test]
    fn mohr_coulomb_failure_at_high_shear() {
        let stress = StressTensor::new(100e6, 0.0, 0.0, 0.0, 0.0, 0.0); // uniaxial 100 MPa
        let failed = mohr_coulomb_failure(&stress, 10e6, FRAC_PI_6);
        assert!(failed, "Should fail under 100 MPa uniaxial stress");
    }

    #[test]
    fn mohr_coulomb_stable_at_low_stress() {
        let stress = StressTensor::new(1e6, 1e6, 1e6, 0.0, 0.0, 0.0); // 1 MPa hydrostatic
        let failed = mohr_coulomb_failure(&stress, 10e6, FRAC_PI_6);
        assert!(!failed, "Should be stable under 1 MPa hydrostatic");
    }

    #[test]
    fn mohr_coulomb_safety_factor_above_one_is_stable() {
        let stress = StressTensor::new(5e6, 5e6, 5e6, 0.0, 0.0, 0.0);
        let fos = mohr_coulomb_safety_factor(&stress, 10e6, FRAC_PI_6);
        assert!(fos > 1.0);
    }

    // --- Failure mode tests ---

    #[test]
    fn shallow_stress_is_brittle() {
        let mode = classify_failure_mode(
            5e6,   // low confining
            100e6, // high differential
            10e6,  // cohesion
            FRAC_PI_6, 150e6, // yield strength
        );
        assert_eq!(mode, FailureMode::Brittle);
    }

    #[test]
    fn deep_stress_is_ductile() {
        let mode = classify_failure_mode(
            500e6, // high confining
            200e6, // differential > yield
            10e6, FRAC_PI_6, 150e6,
        );
        assert_eq!(mode, FailureMode::Ductile);
    }

    #[test]
    fn low_stress_is_stable() {
        let mode = classify_failure_mode(
            50e6, // moderate confining
            10e6, // low differential
            10e6, FRAC_PI_6, 150e6,
        );
        assert_eq!(mode, FailureMode::Stable);
    }

    #[test]
    fn brittle_ductile_depth_positive() {
        let mat = granite_material();
        let depth = brittle_ductile_transition_depth(&mat, 10e6, FRAC_PI_6, 9.81);
        assert!(depth > 0.0);
        // For granite: ~(75e6 - 10e6) / (2700 * 9.81 * tan(30°)) ≈ 4.2 km
        assert!(
            depth > 1000.0 && depth < 10_000.0,
            "BDT depth should be ~4 km, got {depth}"
        );
    }

    // --- Slope stability tests ---

    #[test]
    fn flat_slope_infinite_safety() {
        let fos = infinite_slope_safety_factor(10e3, FRAC_PI_6, 26_500.0, 5.0, 0.0);
        assert_eq!(fos, f64::INFINITY);
    }

    #[test]
    fn steep_slope_lower_safety() {
        let gentle =
            infinite_slope_safety_factor(10e3, FRAC_PI_6, 26_500.0, 5.0, 15.0_f64.to_radians());
        let steep =
            infinite_slope_safety_factor(10e3, FRAC_PI_6, 26_500.0, 5.0, 45.0_f64.to_radians());
        assert!(gentle > steep);
    }

    #[test]
    fn cohesion_increases_safety() {
        let low_c =
            infinite_slope_safety_factor(5e3, FRAC_PI_6, 26_500.0, 5.0, 30.0_f64.to_radians());
        let high_c =
            infinite_slope_safety_factor(50e3, FRAC_PI_6, 26_500.0, 5.0, 30.0_f64.to_radians());
        assert!(high_c > low_c);
    }

    // --- Material preset tests ---

    #[test]
    fn quartzite_stiffest_rock() {
        let rocks = [
            granite_material(),
            basalt_material(),
            limestone_material(),
            sandstone_material(),
            marble_material(),
            shale_material(),
            quartzite_material(),
            gneiss_material(),
        ];
        let max_e = rocks
            .iter()
            .max_by(|a, b| a.youngs_modulus.partial_cmp(&b.youngs_modulus).unwrap())
            .unwrap();
        assert_eq!(max_e.name, "Quartzite");
    }

    #[test]
    fn all_presets_positive_density() {
        for mat in [
            granite_material(),
            basalt_material(),
            limestone_material(),
            sandstone_material(),
            marble_material(),
            shale_material(),
            quartzite_material(),
            gneiss_material(),
        ] {
            assert!(mat.density > 0.0, "{} density should be positive", mat.name);
        }
    }

    #[test]
    fn drucker_prager_conversion() {
        let (alpha, k) = mohr_coulomb_to_drucker_prager(FRAC_PI_6, 10e6);
        assert!(alpha > 0.0);
        assert!(k > 0.0);
    }

    // --- Seismic attenuation tests ---

    #[test]
    fn vp_decreases_with_temperature() {
        let mat = granite_material();
        let cold = p_wave_at_temperature(&mat, 20.0, 20.0);
        let hot = p_wave_at_temperature(&mat, 500.0, 20.0);
        assert!(hot < cold);
    }

    #[test]
    fn vs_decreases_with_temperature() {
        let mat = granite_material();
        let cold = s_wave_at_temperature(&mat, 20.0, 20.0);
        let hot = s_wave_at_temperature(&mat, 500.0, 20.0);
        assert!(hot < cold);
    }

    #[test]
    fn velocity_at_reference_temp_equals_base() {
        let mat = granite_material();
        let vp_base = p_wave_velocity(&mat);
        let vp_ref = p_wave_at_temperature(&mat, 20.0, 20.0);
        assert!((vp_base - vp_ref).abs() < 0.01);
    }

    #[test]
    fn velocity_depth_profile_monotonic_decrease() {
        let mat = granite_material();
        let profile = velocity_depth_profile(&mat, 15.0, 25.0, 30.0, 10);
        assert_eq!(profile.len(), 11);
        for pair in profile.windows(2) {
            assert!(pair[1].1 <= pair[0].1, "Vp should decrease with depth");
            assert!(pair[1].2 <= pair[0].2, "Vs should decrease with depth");
        }
    }

    // --- Weathering-mechanics feedback tests ---

    #[test]
    fn weathered_granite_weaker() {
        let fresh = granite_material();
        let degraded = weathered_material(&fresh, 0.5);
        assert!(degraded.youngs_modulus < fresh.youngs_modulus);
        assert!(degraded.yield_strength < fresh.yield_strength);
    }

    #[test]
    fn fully_weathered_very_weak() {
        let fresh = granite_material();
        let saprolite = weathered_material(&fresh, 1.0);
        assert!(saprolite.youngs_modulus < fresh.youngs_modulus * 0.15);
        assert!(saprolite.yield_strength < fresh.yield_strength * 0.10);
    }

    #[test]
    fn unweathered_unchanged() {
        let fresh = granite_material();
        let same = weathered_material(&fresh, 0.0);
        assert!((same.youngs_modulus - fresh.youngs_modulus).abs() < 1.0);
    }

    #[test]
    fn time_to_failure_finite_at_depth() {
        let mat = granite_material();
        let time = time_to_weathering_failure(&mat, 3000.0, 9.81, 1e-6);
        assert!(time.is_some());
        assert!(time.unwrap() > 0.0);
    }

    #[test]
    fn time_to_failure_none_for_shallow() {
        let mat = granite_material();
        // Shallow — stress much less than yield strength even at full weathering
        let time = time_to_weathering_failure(&mat, 100.0, 9.81, 1e-6);
        assert!(
            time.is_none(),
            "Very shallow rock should never fail under own weight"
        );
    }
}
