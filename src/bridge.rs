//! Cross-crate bridges — convert primitive values from other AGNOS science crates
//! into khanij geology parameters and vice versa.
//!
//! Always available — takes primitive values (f64), no science crate deps.
//!
//! # Architecture
//!
//! ```text
//! dravya (material science) ──┐
//! ushma  (thermodynamics)     ┼──> bridge ──> khanij geology parameters
//! kimiya (chemistry)         ┘
//! ```

// ── Dravya bridges (material science) ──────────────────────────────────────

/// Convert Mohs hardness (1–10) to approximate Vickers hardness (HV).
///
/// Empirical power-law fit: HV ≈ 3 × Mohs^3.3 (good approximation across
/// the Mohs range: talc ~30 HV, quartz ~1000 HV, diamond ~10000 HV).
#[must_use]
#[inline]
pub fn mohs_to_vickers(mohs: f64) -> f64 {
    if mohs <= 0.0 {
        return 0.0;
    }
    3.0 * mohs.clamp(1.0, 10.0).powf(3.3)
}

/// Convert rock porosity (fraction 0–1) to estimated permeability (m²)
/// using the Kozeny-Carman relation.
///
/// k ≈ d² × φ³ / (180 × (1-φ)²), where d is grain diameter.
#[must_use]
pub fn porosity_to_permeability(porosity: f64, grain_diameter_m: f64) -> f64 {
    let phi = porosity.clamp(0.0, 0.99);
    let d2 = grain_diameter_m * grain_diameter_m;
    let one_minus_phi = 1.0 - phi;
    if one_minus_phi <= 0.0 {
        return 0.0;
    }
    d2 * phi.powi(3) / (180.0 * one_minus_phi * one_minus_phi)
}

/// Convert Young's modulus (Pa) and density (kg/m³) to estimated
/// P-wave velocity (m/s) for an isotropic rock.
///
/// Vp ≈ sqrt(E × (1-ν) / (ρ × (1+ν) × (1-2ν))), assuming ν ≈ 0.25.
#[must_use]
#[inline]
pub fn elastic_to_p_wave_velocity(youngs_modulus_pa: f64, density_kg_m3: f64) -> f64 {
    if density_kg_m3 <= 0.0 || youngs_modulus_pa <= 0.0 {
        return 0.0;
    }
    let nu = 0.25;
    let factor = (1.0 - nu) / ((1.0 + nu) * (1.0 - 2.0 * nu));
    (youngs_modulus_pa * factor / density_kg_m3).sqrt()
}

// ── Ushma bridges (thermodynamics) ─────────────────────────────────────────

/// Convert depth (m) and geothermal gradient (°C/km) to subsurface
/// temperature (°C).
///
/// T = T_surface + gradient × depth / 1000.
#[must_use]
#[inline]
pub fn depth_to_temperature(
    surface_temperature_c: f64,
    depth_m: f64,
    gradient_c_per_km: f64,
) -> f64 {
    surface_temperature_c + gradient_c_per_km * depth_m / 1000.0
}

/// Convert mineral thermal conductivity (W/(m·K)) and temperature gradient
/// (°C/m) to heat flow (W/m²).
///
/// q = -k × dT/dz (Fourier's law, positive upward for positive gradient).
#[must_use]
#[inline]
pub fn conductivity_to_heat_flow(conductivity_w_per_m_k: f64, gradient_c_per_m: f64) -> f64 {
    conductivity_w_per_m_k * gradient_c_per_m
}

// ── Kimiya bridges (chemistry) ─────────────────────────────────────────────

/// Convert mineral composition as element weight percentages to a
/// simplified oxide weight percentage (for geochemistry).
///
/// `si_pct`: silicon weight %, `al_pct`: aluminum weight %.
/// Returns `(SiO2_pct, Al2O3_pct)` — the two most important oxide components.
#[must_use]
#[inline]
pub fn element_to_oxide_pct(si_pct: f64, al_pct: f64) -> (f64, f64) {
    // Si → SiO₂: multiply by (28.086 + 2×16) / 28.086 = 60.086/28.086 = 2.139
    // Al → Al₂O₃: multiply by (2×26.982 + 3×16) / (2×26.982) = 101.964/53.964 = 1.889
    let sio2 = si_pct * 2.139;
    let al2o3 = al_pct * 1.889;
    (sio2, al2o3)
}

/// Convert ore grade (% metal content) and recovery efficiency (0–1) to
/// estimated extraction yield (kg metal per tonne of ore).
///
/// yield = grade/100 × recovery × 1000 kg/t.
#[must_use]
#[inline]
pub fn grade_to_yield_kg_per_tonne(grade_percent: f64, recovery_fraction: f64) -> f64 {
    (grade_percent / 100.0) * recovery_fraction.clamp(0.0, 1.0) * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Dravya ─────────────────────────────────────────────────────────

    #[test]
    fn mohs_to_vickers_quartz() {
        // Quartz (Mohs 7) ≈ HV ~1000
        let hv = mohs_to_vickers(7.0);
        assert!(hv > 500.0 && hv < 2000.0, "quartz HV: {hv}");
    }

    #[test]
    fn mohs_to_vickers_talc() {
        // Talc (Mohs 1) should be soft
        let hv = mohs_to_vickers(1.0);
        assert!(hv < 100.0, "talc HV: {hv}");
    }

    #[test]
    fn mohs_zero() {
        assert_eq!(mohs_to_vickers(0.0), 0.0);
    }

    #[test]
    fn porosity_to_permeability_sandstone() {
        // Sandstone: φ ≈ 0.2, d ≈ 0.5mm → k ≈ 1e-12 m²
        let k = porosity_to_permeability(0.2, 0.0005);
        assert!(k > 1e-14 && k < 1e-10, "sandstone k: {k}");
    }

    #[test]
    fn porosity_zero() {
        assert_eq!(porosity_to_permeability(0.0, 0.001), 0.0);
    }

    #[test]
    fn p_wave_granite() {
        // Granite: E ≈ 50 GPa, ρ ≈ 2700 → Vp ≈ 5000 m/s
        let vp = elastic_to_p_wave_velocity(50e9, 2700.0);
        assert!(vp > 3000.0 && vp < 7000.0, "granite Vp: {vp}");
    }

    #[test]
    fn p_wave_zero_density() {
        assert_eq!(elastic_to_p_wave_velocity(50e9, 0.0), 0.0);
    }

    // ── Ushma ──────────────────────────────────────────────────────────

    #[test]
    fn depth_temp_surface() {
        let t = depth_to_temperature(15.0, 0.0, 30.0);
        assert!((t - 15.0).abs() < 0.01);
    }

    #[test]
    fn depth_temp_1km() {
        // 15°C surface, 30°C/km → 45°C at 1km
        let t = depth_to_temperature(15.0, 1000.0, 30.0);
        assert!((t - 45.0).abs() < 0.01);
    }

    #[test]
    fn heat_flow_basic() {
        // k = 3.0, gradient = 0.03 °C/m → q = 0.09 W/m²
        let q = conductivity_to_heat_flow(3.0, 0.03);
        assert!((q - 0.09).abs() < 0.001);
    }

    // ── Kimiya ─────────────────────────────────────────────────────────

    #[test]
    fn element_to_oxide_silicon() {
        let (sio2, _) = element_to_oxide_pct(46.7, 0.0);
        // Earth's crust Si ≈ 46.7% → SiO₂ ≈ 99.9%
        assert!((sio2 - 99.9).abs() < 1.0);
    }

    #[test]
    fn grade_to_yield_copper() {
        // 1% Cu, 90% recovery → 9 kg/t
        let y = grade_to_yield_kg_per_tonne(1.0, 0.9);
        assert!((y - 9.0).abs() < 0.01);
    }

    #[test]
    fn grade_to_yield_zero_recovery() {
        assert_eq!(grade_to_yield_kg_per_tonne(5.0, 0.0), 0.0);
    }
}
