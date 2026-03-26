//! Hydrothermal ore formation model — combines geothermal cooling, fluid flow,
//! and mineral stability to model where ore deposits form around intrusions.
//!
//! Requires the `thermodynamics`, `fluids`, and `chemistry` features.

/// Temperature-pressure conditions at a point in a hydrothermal system.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cond = HydrothermalConditions {
///     temperature_k: 573.15,
///     pressure_pa: 1e8,
///     distance_m: 50.0,
///     flow_rate: 1e-4,
/// };
/// assert!((cond.temperature_k - 573.15).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct HydrothermalConditions {
    /// Temperature in kelvin.
    pub temperature_k: f64,
    /// Pressure in pascals.
    pub pressure_pa: f64,
    /// Distance from intrusion contact in metres.
    pub distance_m: f64,
    /// Fluid flow rate in m³/s (from Darcy flow).
    pub flow_rate: f64,
}

/// Hydrothermal alteration zone classification.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let zone = classify_alteration(400.0);
/// assert_eq!(zone, AlterationZone::Phyllic);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterationZone {
    /// > 500°C — potassic alteration (K-feldspar, biotite).
    Potassic,
    /// 350-500°C — phyllic alteration (sericite, quartz, pyrite).
    Phyllic,
    /// 250-350°C — argillic alteration (clay minerals, chlorite).
    Argillic,
    /// < 250°C — propylitic alteration (epidote, chlorite, calcite).
    Propylitic,
    /// Too cold for significant alteration.
    Unaltered,
}

/// Classify the hydrothermal alteration zone from temperature.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_alteration(600.0), AlterationZone::Potassic);
/// assert_eq!(classify_alteration(100.0), AlterationZone::Unaltered);
/// ```
#[must_use]
pub fn classify_alteration(temperature_c: f64) -> AlterationZone {
    if temperature_c > 500.0 {
        AlterationZone::Potassic
    } else if temperature_c > 350.0 {
        AlterationZone::Phyllic
    } else if temperature_c > 250.0 {
        AlterationZone::Argillic
    } else if temperature_c > 150.0 {
        AlterationZone::Propylitic
    } else {
        AlterationZone::Unaltered
    }
}

/// Metal solubility as a function of temperature (simplified).
///
/// Most metals are more soluble in hot fluids and precipitate on cooling.
/// This returns a relative solubility factor (0.0-1.0).
///
/// - `temperature_c`: fluid temperature in °C
/// - `precipitation_temp_c`: temperature at which the metal precipitates
///   (e.g., gold ~300°C, copper ~350°C, lead ~200°C)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let hot = metal_solubility(500.0, 300.0);
/// let cold = metal_solubility(200.0, 300.0);
/// assert!(hot > 0.0);
/// assert!(cold.abs() < 1e-10);
/// ```
#[must_use]
pub fn metal_solubility(temperature_c: f64, precipitation_temp_c: f64) -> f64 {
    if temperature_c <= precipitation_temp_c {
        // Below precipitation temp — metal is insoluble (depositing)
        0.0
    } else {
        // Above — solubility increases with temperature
        let excess = temperature_c - precipitation_temp_c;
        (1.0 - (-excess / 200.0).exp()).clamp(0.0, 1.0)
    }
}

/// Rate of metal precipitation from cooling fluid.
///
/// Precipitation occurs in a narrow temperature window around the
/// precipitation temperature. The rate peaks at the precipitation temp
/// and falls off above and below.
///
/// Returns a relative precipitation rate (0.0-1.0).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let at_target = precipitation_rate(300.0, 300.0);
/// assert!((at_target - 1.0).abs() < 0.01);
/// ```
#[must_use]
pub fn precipitation_rate(temperature_c: f64, precipitation_temp_c: f64) -> f64 {
    // Gaussian-like peak at the precipitation temperature, σ ≈ 30°C
    let delta = temperature_c - precipitation_temp_c;
    (-delta.powi(2) / (2.0 * 30.0_f64.powi(2))).exp()
}

/// Estimate ore grade enhancement from hydrothermal fluid focusing.
///
/// Ore grades are higher where:
/// 1. Fluid flux is high (more metal delivered)
/// 2. Temperature is near precipitation temperature (metal drops out)
/// 3. There's a structural trap (porosity/permeability contrast)
///
/// - `fluid_flux`: Darcy flux in m/s
/// - `temperature_c`: fluid temperature in °C
/// - `precipitation_temp_c`: metal's precipitation temperature
/// - `porosity`: host rock porosity (0.0-1.0)
/// - `background_grade`: regional background grade (fraction)
///
/// Returns estimated ore grade (fraction).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let grade = estimated_ore_grade(1e-6, 300.0, 300.0, 0.1, 0.001);
/// assert!(grade > 0.001);
/// ```
#[must_use]
pub fn estimated_ore_grade(
    fluid_flux: f64,
    temperature_c: f64,
    precipitation_temp_c: f64,
    porosity: f64,
    background_grade: f64,
) -> f64 {
    let precip = precipitation_rate(temperature_c, precipitation_temp_c);
    let flux_factor = (fluid_flux / 1e-6).min(10.0); // normalise to typical Darcy flux
    let trap_factor = porosity * 5.0; // more porous = more trapping
    let enhancement = precip * flux_factor * trap_factor;
    (background_grade * (1.0 + enhancement)).min(1.0)
}

/// Typical precipitation temperatures for common ore metals (°C).
pub mod precipitation_temps {
    pub const GOLD: f64 = 300.0;
    pub const COPPER: f64 = 350.0;
    pub const SILVER: f64 = 250.0;
    pub const LEAD: f64 = 200.0;
    pub const ZINC: f64 = 250.0;
    pub const TIN: f64 = 400.0;
    pub const MOLYBDENUM: f64 = 500.0;
    pub const TUNGSTEN: f64 = 450.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alteration_zones_temperature_order() {
        assert_eq!(classify_alteration(600.0), AlterationZone::Potassic);
        assert_eq!(classify_alteration(400.0), AlterationZone::Phyllic);
        assert_eq!(classify_alteration(300.0), AlterationZone::Argillic);
        assert_eq!(classify_alteration(200.0), AlterationZone::Propylitic);
        assert_eq!(classify_alteration(100.0), AlterationZone::Unaltered);
    }

    #[test]
    fn metal_soluble_when_hot() {
        let hot = metal_solubility(500.0, 300.0);
        let cold = metal_solubility(200.0, 300.0);
        assert!(hot > 0.0);
        assert!(cold.abs() < f64::EPSILON);
    }

    #[test]
    fn precipitation_peaks_at_target_temp() {
        let at_target = precipitation_rate(300.0, 300.0);
        let above = precipitation_rate(400.0, 300.0);
        let below = precipitation_rate(200.0, 300.0);
        assert!(at_target > above);
        assert!(at_target > below);
        assert!((at_target - 1.0).abs() < 0.01); // peak ≈ 1.0
    }

    #[test]
    fn ore_grade_enhanced_at_precipitation_temp() {
        let bg = 0.001;
        let enhanced = estimated_ore_grade(1e-6, 300.0, 300.0, 0.1, bg);
        assert!(
            enhanced > bg,
            "Grade should be enhanced at precipitation temp"
        );
    }

    #[test]
    fn ore_grade_not_enhanced_far_from_precip_temp() {
        let bg = 0.001;
        let far = estimated_ore_grade(1e-6, 100.0, 300.0, 0.1, bg);
        assert!(
            (far - bg).abs() < bg * 0.1,
            "Grade should be near background far from precip temp"
        );
    }

    #[test]
    fn ore_grade_bounded() {
        let grade = estimated_ore_grade(1e-3, 300.0, 300.0, 0.5, 0.5);
        assert!(grade <= 1.0);
    }

    #[test]
    fn higher_flux_higher_grade() {
        let low = estimated_ore_grade(1e-8, 300.0, 300.0, 0.1, 0.001);
        let high = estimated_ore_grade(1e-5, 300.0, 300.0, 0.1, 0.001);
        assert!(high > low);
    }

    #[test]
    fn gold_precipitates_at_300c() {
        assert!((precipitation_temps::GOLD - 300.0).abs() < f64::EPSILON);
    }
}
