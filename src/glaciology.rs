//! Glacier and ice sheet dynamics — Glen's flow law, basal sliding, mass
//! balance, equilibrium line altitude, isostatic adjustment, and
//! depth-integrated ice velocity.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Ice density (kg/m³).
const RHO_ICE: f64 = 917.0;

/// Mantle density (kg/m³).
const RHO_MANTLE: f64 = 3300.0;

/// Gravitational acceleration (m/s²).
const G: f64 = 9.81;

/// Seconds per year (365.25 days).
const SECONDS_PER_YEAR: f64 = 365.25 * 24.0 * 3600.0;

/// Reference Glen flow-law parameter A₀ at −10 °C (Pa⁻³ s⁻¹).
///
/// This is a widely-used reference value for temperate/cold-ice
/// calculations with Glen exponent n = 3.
const A_REF: f64 = 2.4e-24;

/// Reference temperature for `A_REF` (°C).
const T_REF_C: f64 = -10.0;

/// Temperature doubling interval for A (°C).
///
/// A doubles for every 10 °C increase (empirical Arrhenius
/// approximation).
const DOUBLING_INTERVAL: f64 = 10.0;

// ---------------------------------------------------------------------------
// GlacierType
// ---------------------------------------------------------------------------

/// Classification of glacier morphology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GlacierType {
    /// Alpine / valley glacier — confined by topography.
    Alpine,
    /// Continental ice sheet (e.g. Greenland, Antarctica).
    IceSheet,
    /// Ice cap — dome-shaped, smaller than an ice sheet.
    IceCap,
    /// Piedmont glacier — valley glacier that spreads onto a plain.
    Piedmont,
    /// Tidewater glacier — terminates in the ocean with calving.
    TideWater,
}

// ---------------------------------------------------------------------------
// Glen's flow law
// ---------------------------------------------------------------------------

/// Temperature-dependent flow-law parameter *A* (Pa⁻ⁿ s⁻¹).
///
/// Uses an Arrhenius-style doubling rule:
/// A(T) = A₀ × 2^((T − T_ref) / ΔT_doubling)
#[must_use]
fn flow_parameter_a(temperature_c: f64) -> f64 {
    let exponent = (temperature_c - T_REF_C) / DOUBLING_INTERVAL;
    A_REF * f64::exp2(exponent)
}

/// Glen's flow law — compute the strain rate for a given deviatoric
/// stress, temperature, and flow-law exponent.
///
/// ε̇ = A(T) × τⁿ
///
/// # Arguments
///
/// * `stress_pa` — deviatoric stress τ (Pa).
/// * `temperature_c` — ice temperature (°C).
/// * `n` — Glen exponent (commonly 3).
///
/// # Returns
///
/// Strain rate in s⁻¹.
#[must_use]
pub fn glen_flow_law(stress_pa: f64, temperature_c: f64, n: f64) -> f64 {
    let a = flow_parameter_a(temperature_c);
    a * stress_pa.powf(n)
}

// ---------------------------------------------------------------------------
// Basal sliding
// ---------------------------------------------------------------------------

/// Weertman-style basal sliding velocity.
///
/// v_b = k × τ_b^p / N^q
///
/// with p = 3, q = 1, k = 1 × 10⁻¹⁵ (SI).
///
/// # Arguments
///
/// * `basal_shear_stress` — basal shear stress τ_b (Pa).
/// * `effective_pressure` — effective pressure N = overburden − water
///   pressure (Pa). Must be > 0.
///
/// # Returns
///
/// Sliding velocity in m/s.
#[must_use]
pub fn basal_sliding_velocity(basal_shear_stress: f64, effective_pressure: f64) -> f64 {
    const K: f64 = 1e-15;
    const P: f64 = 3.0;
    const Q: f64 = 1.0;

    if effective_pressure <= 0.0 {
        return 0.0;
    }

    K * basal_shear_stress.powf(P) / effective_pressure.powf(Q)
}

// ---------------------------------------------------------------------------
// Mass balance
// ---------------------------------------------------------------------------

/// Net glacier mass balance.
///
/// B = accumulation − ablation
///
/// # Returns
///
/// Mass balance in m/yr water equivalent.
#[must_use]
pub fn mass_balance(accumulation_m_yr: f64, ablation_m_yr: f64) -> f64 {
    accumulation_m_yr - ablation_m_yr
}

// ---------------------------------------------------------------------------
// Equilibrium line altitude
// ---------------------------------------------------------------------------

/// Simplified equilibrium line altitude (ELA).
///
/// ELA ≈ (summit + terminus) / 2
///
/// A rough first-order approximation assuming symmetric mass-balance
/// gradient.
///
/// # Returns
///
/// Altitude in metres.
#[must_use]
pub fn equilibrium_line_altitude(summit_m: f64, terminus_m: f64) -> f64 {
    (summit_m + terminus_m) / 2.0
}

// ---------------------------------------------------------------------------
// Isostatic depression
// ---------------------------------------------------------------------------

/// Crustal depression under an ice load (isostatic equilibrium).
///
/// d = H_ice × (ρ_ice / ρ_mantle)
///
/// # Returns
///
/// Depression in metres.
#[must_use]
pub fn isostatic_depression(ice_thickness_m: f64) -> f64 {
    ice_thickness_m * (RHO_ICE / RHO_MANTLE)
}

// ---------------------------------------------------------------------------
// Isostatic rebound time
// ---------------------------------------------------------------------------

/// Simplified isostatic rebound relaxation time.
///
/// τ ≈ η / (ρ_mantle × g × 100)
///
/// where 100 m is a characteristic wavelength scaling factor.
///
/// # Arguments
///
/// * `depression_m` — current depression (used only for context; the
///   relaxation time depends on viscosity and mantle properties).
/// * `viscosity_pa_s` — mantle viscosity η (Pa·s), typically ~10²¹.
///
/// # Returns
///
/// Relaxation time in years.
#[must_use]
pub fn isostatic_rebound_time(_depression_m: f64, viscosity_pa_s: f64) -> f64 {
    let tau_seconds = viscosity_pa_s / (RHO_MANTLE * G * 100.0);
    tau_seconds / SECONDS_PER_YEAR
}

// ---------------------------------------------------------------------------
// Depth-integrated ice velocity
// ---------------------------------------------------------------------------

/// Depth-integrated velocity from Glen's flow law (shallow-ice
/// approximation).
///
/// v = (2 A / (n + 1)) × (ρ g sin α)ⁿ × H^(n+1)
///
/// Uses n = 3 and temperature-dependent A.
///
/// # Arguments
///
/// * `surface_slope` — surface slope angle α (radians).
/// * `thickness_m` — ice thickness H (m).
/// * `temperature_c` — ice temperature (°C).
///
/// # Returns
///
/// Depth-integrated surface velocity in m/yr.
#[must_use]
pub fn ice_velocity_depth_integrated(
    surface_slope: f64,
    thickness_m: f64,
    temperature_c: f64,
) -> f64 {
    let n: f64 = 3.0;
    let a = flow_parameter_a(temperature_c);

    let driving_stress = RHO_ICE * G * surface_slope.sin();
    let velocity_m_s = (2.0 * a / (n + 1.0)) * driving_stress.powf(n) * thickness_m.powf(n + 1.0);

    velocity_m_s * SECONDS_PER_YEAR
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    // -- Glen's flow law --

    #[test]
    fn glen_flow_law_increases_with_stress() {
        let low = glen_flow_law(1e5, -10.0, 3.0);
        let high = glen_flow_law(2e5, -10.0, 3.0);
        assert!(high > low, "strain rate should increase with stress");
    }

    #[test]
    fn glen_flow_law_increases_with_temperature() {
        let cold = glen_flow_law(1e5, -20.0, 3.0);
        let warm = glen_flow_law(1e5, -10.0, 3.0);
        assert!(warm > cold, "strain rate should increase with temperature");
    }

    #[test]
    fn glen_flow_law_reference_value() {
        // At T = -10 °C and n = 3, A should be exactly A_REF.
        let rate = glen_flow_law(1.0, -10.0, 3.0);
        assert!(
            (rate - A_REF).abs() < 1e-40,
            "A(T_ref) × 1^3 should equal A_REF"
        );
    }

    #[test]
    fn glen_flow_law_zero_stress() {
        let rate = glen_flow_law(0.0, -10.0, 3.0);
        assert!(
            rate.abs() < EPS,
            "zero stress should yield zero strain rate"
        );
    }

    // -- Basal sliding --

    #[test]
    fn basal_sliding_positive() {
        let v = basal_sliding_velocity(1e5, 1e6);
        assert!(v > 0.0, "sliding velocity should be positive");
    }

    #[test]
    fn basal_sliding_increases_with_stress() {
        let v_low = basal_sliding_velocity(1e5, 1e6);
        let v_high = basal_sliding_velocity(2e5, 1e6);
        assert!(
            v_high > v_low,
            "velocity should increase with basal shear stress"
        );
    }

    #[test]
    fn basal_sliding_zero_effective_pressure() {
        let v = basal_sliding_velocity(1e5, 0.0);
        assert!(v.abs() < EPS, "zero effective pressure should return 0");
    }

    // -- Mass balance --

    #[test]
    fn mass_balance_positive_when_accumulation_exceeds_ablation() {
        let b = mass_balance(2.0, 1.5);
        assert!((b - 0.5).abs() < EPS);
    }

    #[test]
    fn mass_balance_negative_when_ablation_exceeds() {
        let b = mass_balance(1.0, 3.0);
        assert!(b < 0.0);
    }

    // -- ELA --

    #[test]
    fn ela_midpoint() {
        let ela = equilibrium_line_altitude(4000.0, 2000.0);
        assert!((ela - 3000.0).abs() < EPS);
    }

    // -- Isostatic depression --

    #[test]
    fn isostatic_depression_proportional() {
        let d = isostatic_depression(3000.0);
        let expected = 3000.0 * (917.0 / 3300.0);
        assert!((d - expected).abs() < 1e-6);
    }

    #[test]
    fn isostatic_depression_zero_ice() {
        let d = isostatic_depression(0.0);
        assert!(d.abs() < EPS);
    }

    // -- Isostatic rebound time --

    #[test]
    fn rebound_time_positive() {
        let t = isostatic_rebound_time(100.0, 1e21);
        assert!(t > 0.0, "rebound time should be positive");
    }

    #[test]
    fn rebound_time_increases_with_viscosity() {
        let t_low = isostatic_rebound_time(100.0, 1e20);
        let t_high = isostatic_rebound_time(100.0, 1e21);
        assert!(
            t_high > t_low,
            "higher viscosity should yield longer rebound time"
        );
    }

    // -- Ice velocity --

    #[test]
    fn velocity_positive_for_downslope() {
        let v = ice_velocity_depth_integrated(0.05, 500.0, -10.0);
        assert!(v > 0.0, "downslope flow should yield positive velocity");
    }

    #[test]
    fn velocity_increases_with_thickness() {
        let v_thin = ice_velocity_depth_integrated(0.05, 200.0, -10.0);
        let v_thick = ice_velocity_depth_integrated(0.05, 500.0, -10.0);
        assert!(v_thick > v_thin, "thicker ice should flow faster");
    }

    // -- GlacierType --

    #[test]
    fn glacier_type_roundtrip_serde() {
        let gt = GlacierType::TideWater;
        let json = serde_json::to_string(&gt).unwrap();
        let back: GlacierType = serde_json::from_str(&json).unwrap();
        assert_eq!(gt, back);
    }
}
