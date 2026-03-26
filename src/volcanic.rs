//! Volcanic eruption modeling — explosivity classification, magma properties,
//! eruption column dynamics, pyroclastic flow runout, and lava flow velocity.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Volcanic Explosivity Index (VEI)
// ---------------------------------------------------------------------------

/// Volcanic Explosivity Index (VEI) -- integer scale from 0 to 8 describing
/// the relative explosiveness of a volcanic eruption.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let vei = classify_vei(1e8);
/// assert_eq!(vei, Vei::V4);
/// assert!(Vei::V0 < Vei::V8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Vei {
    /// Non-explosive, effusive lava flows.
    V0,
    /// Gentle — small eruption.
    V1,
    /// Explosive — moderate eruption.
    V2,
    /// Severe — substantial eruption.
    V3,
    /// Cataclysmic — large eruption.
    V4,
    /// Paroxysmal — very large eruption.
    V5,
    /// Colossal — massive eruption (e.g. Pinatubo 1991).
    V6,
    /// Super-colossal — enormous eruption (e.g. Tambora 1815).
    V7,
    /// Mega-colossal — largest known eruptions (e.g. Yellowstone caldera).
    V8,
}

impl Vei {
    /// Returns the typical ejecta volume range in m3 as `(min, max)`.
    ///
    /// Upper bound for VEI 8 is set to 1e13 m3 as a practical cap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let (lo, hi) = Vei::V5.ejecta_volume_m3();
    /// assert!((lo - 1e9).abs() < 1.0);
    /// assert!(lo < hi);
    /// ```
    #[must_use]
    pub fn ejecta_volume_m3(&self) -> (f64, f64) {
        match self {
            Self::V0 => (0.0, 1e4),
            Self::V1 => (1e4, 1e6),
            Self::V2 => (1e6, 1e7),
            Self::V3 => (1e7, 1e8),
            Self::V4 => (1e8, 1e9),
            Self::V5 => (1e9, 1e10),
            Self::V6 => (1e10, 1e11),
            Self::V7 => (1e11, 1e12),
            Self::V8 => (1e12, 1e13),
        }
    }

    /// Short textual description of the eruption severity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert_eq!(Vei::V0.description(), "non-explosive, effusive");
    /// assert!(!Vei::V8.description().is_empty());
    /// ```
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::V0 => "non-explosive, effusive",
            Self::V1 => "gentle",
            Self::V2 => "explosive",
            Self::V3 => "severe",
            Self::V4 => "cataclysmic",
            Self::V5 => "paroxysmal",
            Self::V6 => "colossal",
            Self::V7 => "super-colossal",
            Self::V8 => "mega-colossal",
        }
    }
}

/// Classify an eruption by its total ejecta volume in m3.
///
/// Volumes at or below 0 are clamped to [`Vei::V0`]; volumes exceeding
/// 1e12 m3 are classified as [`Vei::V8`].
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_vei(500.0), Vei::V0);
/// assert_eq!(classify_vei(1e6), Vei::V2);
/// assert_eq!(classify_vei(1e13), Vei::V8);
/// ```
#[must_use]
pub fn classify_vei(ejecta_volume_m3: f64) -> Vei {
    if ejecta_volume_m3 < 1e4 {
        Vei::V0
    } else if ejecta_volume_m3 < 1e6 {
        Vei::V1
    } else if ejecta_volume_m3 < 1e7 {
        Vei::V2
    } else if ejecta_volume_m3 < 1e8 {
        Vei::V3
    } else if ejecta_volume_m3 < 1e9 {
        Vei::V4
    } else if ejecta_volume_m3 < 1e10 {
        Vei::V5
    } else if ejecta_volume_m3 < 1e11 {
        Vei::V6
    } else if ejecta_volume_m3 < 1e12 {
        Vei::V7
    } else {
        Vei::V8
    }
}

// ---------------------------------------------------------------------------
// Magma composition & classification
// ---------------------------------------------------------------------------

/// Weight-percent oxide composition of a magma.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mc = MagmaComposition {
///     sio2: 50.0, al2o3: 15.0, feo: 8.0,
///     mgo: 7.0, cao: 11.0, na2o: 2.5, k2o: 0.5,
/// };
/// assert!((mc.sio2 - 50.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagmaComposition {
    /// SiO₂ weight percent.
    pub sio2: f64,
    /// Al₂O₃ weight percent.
    pub al2o3: f64,
    /// FeO (total iron as FeO) weight percent.
    pub feo: f64,
    /// MgO weight percent.
    pub mgo: f64,
    /// CaO weight percent.
    pub cao: f64,
    /// Na₂O weight percent.
    pub na2o: f64,
    /// K₂O weight percent.
    pub k2o: f64,
}

/// Broad magma classification based on silica content.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_magma(48.0), MagmaType::Mafic);
/// assert_eq!(classify_magma(72.0), MagmaType::Felsic);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MagmaType {
    /// SiO₂ < 45 wt%
    Ultramafic,
    /// 45 ≤ SiO₂ < 52 wt%
    Mafic,
    /// 52 ≤ SiO₂ < 63 wt%
    Intermediate,
    /// SiO₂ ≥ 63 wt%
    Felsic,
}

/// Classify magma by its SiO2 weight percent.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_magma(40.0), MagmaType::Ultramafic);
/// assert_eq!(classify_magma(57.0), MagmaType::Intermediate);
/// ```
#[must_use]
pub fn classify_magma(sio2: f64) -> MagmaType {
    if sio2 < 45.0 {
        MagmaType::Ultramafic
    } else if sio2 < 52.0 {
        MagmaType::Mafic
    } else if sio2 < 63.0 {
        MagmaType::Intermediate
    } else {
        MagmaType::Felsic
    }
}

// ---------------------------------------------------------------------------
// Magma viscosity
// ---------------------------------------------------------------------------

/// Estimate magma dynamic viscosity in Pa-s from SiO2 content and temperature.
///
/// Viscosity increases exponentially with SiO2 and decreases with temperature.
/// Uses a simplified Arrhenius-style model:
///
/// ```text
/// ln(mu) = -6.4 + 0.05 * SiO2(%) + 26_000 / T(K)
/// ```
///
/// where T(K) = temperature_c + 273.15.
///
/// Returns `f64::INFINITY` if the temperature is at or below absolute zero.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let v_basalt = magma_viscosity(50.0, 1200.0);
/// let v_rhyolite = magma_viscosity(75.0, 1200.0);
/// assert!(v_rhyolite > v_basalt);
/// ```
#[must_use]
pub fn magma_viscosity(sio2_percent: f64, temperature_c: f64) -> f64 {
    let t_k = temperature_c + 273.15;
    if t_k <= 0.0 {
        return f64::INFINITY;
    }
    // Simplified Arrhenius-type relation
    let ln_mu = -6.4 + 0.05 * sio2_percent + 26_000.0 / t_k;
    ln_mu.exp()
}

// ---------------------------------------------------------------------------
// Eruption column height — Sparks (1986)
// ---------------------------------------------------------------------------

/// Eruption column height using the Sparks (1986) scaling relation.
///
/// ```text
/// H = 0.236 * Q^0.25   (km)
/// ```
///
/// where `Q` is the mass eruption rate in kg/s.
///
/// Returns the height in **kilometres**. Returns 0.0 for non-positive flux.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let h = eruption_column_height(1e8);
/// assert!((h - 23.6).abs() < 0.1);
/// assert_eq!(eruption_column_height(0.0), 0.0);
/// ```
#[must_use]
pub fn eruption_column_height(mass_flux_kg_s: f64) -> f64 {
    if mass_flux_kg_s <= 0.0 {
        return 0.0;
    }
    0.236 * mass_flux_kg_s.powf(0.25)
}

// ---------------------------------------------------------------------------
// Pyroclastic flow runout
// ---------------------------------------------------------------------------

/// Simplified pyroclastic flow runout distance in km.
///
/// Uses an energy-cone model:
///
/// ```text
/// R = H / tan(alpha)
/// ```
///
/// where `H` is the column collapse height in km and `alpha` is the terrain
/// slope in degrees (clamped to [0.5, 89] to avoid singularities).
///
/// A minimum slope of 0.5 degrees is enforced to prevent unrealistic infinite runout.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let r = pyroclastic_flow_runout(10.0, 10.0);
/// assert!(r > 0.0);
/// assert_eq!(pyroclastic_flow_runout(0.0, 10.0), 0.0);
/// ```
#[must_use]
pub fn pyroclastic_flow_runout(column_height_km: f64, slope_degrees: f64) -> f64 {
    if column_height_km <= 0.0 {
        return 0.0;
    }
    let slope = slope_degrees.clamp(0.5, 89.0);
    let tan_slope = slope.to_radians().tan();
    column_height_km / tan_slope
}

// ---------------------------------------------------------------------------
// Lava flow velocity — Jeffreys equation
// ---------------------------------------------------------------------------

/// Lava flow velocity from Jeffreys' equation for a viscous sheet on a slope.
///
/// ```text
/// v = rho * g * h^2 * sin(alpha) / (3 * mu)
/// ```
///
/// - `slope_degrees`: terrain slope angle
/// - `viscosity`: dynamic viscosity in Pa-s
/// - `thickness_m`: flow thickness in metres
/// - `density`: lava density in kg/m3 (typically 2200-2700)
///
/// Returns velocity in **m/s**. Returns 0.0 for non-positive or degenerate
/// inputs.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let v = lava_flow_velocity(10.0, 100.0, 2.0, 2500.0);
/// assert!(v > 0.0);
/// assert_eq!(lava_flow_velocity(10.0, 0.0, 2.0, 2500.0), 0.0);
/// ```
#[must_use]
pub fn lava_flow_velocity(
    slope_degrees: f64,
    viscosity: f64,
    thickness_m: f64,
    density: f64,
) -> f64 {
    if viscosity <= 0.0 || thickness_m <= 0.0 || density <= 0.0 || slope_degrees <= 0.0 {
        return 0.0;
    }
    let g = 9.80665; // m/s²
    let sin_alpha = slope_degrees.to_radians().sin();
    density * g * thickness_m * thickness_m * sin_alpha / (3.0 * viscosity)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- VEI classification --------------------------------------------------

    #[test]
    fn classify_vei_zero() {
        assert_eq!(classify_vei(500.0), Vei::V0);
    }

    #[test]
    fn classify_vei_boundaries() {
        assert_eq!(classify_vei(1e4), Vei::V1);
        assert_eq!(classify_vei(1e6), Vei::V2);
        assert_eq!(classify_vei(1e7), Vei::V3);
        assert_eq!(classify_vei(5e8), Vei::V4);
        assert_eq!(classify_vei(5e9), Vei::V5);
        assert_eq!(classify_vei(5e10), Vei::V6);
        assert_eq!(classify_vei(5e11), Vei::V7);
        assert_eq!(classify_vei(1e13), Vei::V8);
    }

    #[test]
    fn vei_ejecta_volume_range() {
        let (lo, hi) = Vei::V5.ejecta_volume_m3();
        assert!(lo < hi);
        assert!((lo - 1e9).abs() < 1.0);
    }

    #[test]
    fn vei_description_non_empty() {
        assert!(!Vei::V0.description().is_empty());
        assert!(!Vei::V8.description().is_empty());
    }

    // -- Magma viscosity -----------------------------------------------------

    #[test]
    fn viscosity_increases_with_sio2() {
        let v_basalt = magma_viscosity(50.0, 1200.0);
        let v_rhyolite = magma_viscosity(75.0, 1200.0);
        assert!(
            v_rhyolite > v_basalt,
            "rhyolite ({v_rhyolite}) should be more viscous than basalt ({v_basalt})"
        );
    }

    #[test]
    fn viscosity_decreases_with_temperature() {
        let v_cool = magma_viscosity(65.0, 800.0);
        let v_hot = magma_viscosity(65.0, 1200.0);
        assert!(
            v_cool > v_hot,
            "cooler magma ({v_cool}) should be more viscous than hotter ({v_hot})"
        );
    }

    #[test]
    fn viscosity_at_absolute_zero() {
        assert!(magma_viscosity(50.0, -274.0).is_infinite());
    }

    // -- Eruption column height ----------------------------------------------

    #[test]
    fn column_height_plinian() {
        // Plinian eruption ~1e8 kg/s → H ≈ 0.236 × 100 = 23.6 km
        let h = eruption_column_height(1e8);
        assert!((h - 23.6).abs() < 0.1, "expected ~23.6 km, got {h}");
    }

    #[test]
    fn column_height_zero_flux() {
        assert_eq!(eruption_column_height(0.0), 0.0);
    }

    // -- Pyroclastic flow runout ---------------------------------------------

    #[test]
    fn runout_increases_with_height() {
        let r1 = pyroclastic_flow_runout(5.0, 10.0);
        let r2 = pyroclastic_flow_runout(20.0, 10.0);
        assert!(r2 > r1);
    }

    #[test]
    fn runout_zero_height() {
        assert_eq!(pyroclastic_flow_runout(0.0, 10.0), 0.0);
    }

    // -- Lava flow velocity --------------------------------------------------

    #[test]
    fn lava_flow_basic() {
        // Basaltic lava on a 10° slope, ~2 m thick, density 2500, viscosity 100 Pa·s
        let v = lava_flow_velocity(10.0, 100.0, 2.0, 2500.0);
        // ρgh²sin(α)/(3μ) = 2500 * 9.80665 * 4 * sin(10°) / 300
        let expected = 2500.0 * 9.80665 * 4.0 * (10.0_f64).to_radians().sin() / 300.0;
        assert!((v - expected).abs() < 1e-6, "expected {expected}, got {v}");
    }

    #[test]
    fn lava_flow_zero_viscosity() {
        assert_eq!(lava_flow_velocity(10.0, 0.0, 2.0, 2500.0), 0.0);
    }

    // -- Magma classification ------------------------------------------------

    #[test]
    fn magma_classification() {
        assert_eq!(classify_magma(40.0), MagmaType::Ultramafic);
        assert_eq!(classify_magma(48.0), MagmaType::Mafic);
        assert_eq!(classify_magma(57.0), MagmaType::Intermediate);
        assert_eq!(classify_magma(72.0), MagmaType::Felsic);
    }

    #[test]
    fn magma_classification_boundaries() {
        assert_eq!(classify_magma(44.9), MagmaType::Ultramafic);
        assert_eq!(classify_magma(45.0), MagmaType::Mafic);
        assert_eq!(classify_magma(52.0), MagmaType::Intermediate);
        assert_eq!(classify_magma(63.0), MagmaType::Felsic);
    }
}
