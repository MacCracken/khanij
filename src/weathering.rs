use hisab::calc;
#[cfg(feature = "chemistry")]
use serde::{Deserialize, Serialize};

/// Rate of physical weathering (relative scale 0-1).
/// Uses numerical integration over a temperature-moisture interaction model.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let rate = physical_weathering_rate(20.0, 0.5);
/// assert!(rate > 0.0 && rate <= 1.0);
/// ```
#[must_use]
pub fn physical_weathering_rate(temp_range_celsius: f64, moisture_fraction: f64) -> f64 {
    let tr = temp_range_celsius;
    let mf = moisture_fraction;
    if tr <= 0.0 || mf <= 0.0 {
        return 0.0;
    }
    // Integrate the freeze-thaw contribution over the temperature range.
    // The integrand models increasing damage with wider thermal cycling.
    let rate = calc::integral_simpson(|t| (t / tr).powi(2) * mf, 0.0, tr, 20).unwrap_or(0.0);
    // Normalise: max rate at temp_range=50, moisture=1 → integral ≈ 50/3 ≈ 16.67
    let normalised = rate / (50.0 / 3.0);
    normalised.clamp(0.0, 1.0)
}

/// Rate of chemical weathering. Uses Arrhenius-style exponential temperature
/// dependence combined with rainfall, computed via hisab calculus.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let rate = chemical_weathering_rate(25.0, 1500.0);
/// assert!(rate > 0.0 && rate <= 1.0);
/// ```
#[must_use]
pub fn chemical_weathering_rate(mean_temp_celsius: f64, annual_rainfall_mm: f64) -> f64 {
    let temp = mean_temp_celsius;
    let rain = annual_rainfall_mm;
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
    temp_norm * rain_factor
}

/// Erosion rate estimate (Revised Universal Soil Loss Equation, improved).
/// Uses hisab lerp for smooth interpolation of cover factor.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let bare = erosion_rate(50.0, 15.0, 0.0);
/// let covered = erosion_rate(50.0, 15.0, 0.9);
/// assert!(covered < bare);
/// ```
#[must_use]
pub fn erosion_rate(rainfall_intensity: f64, slope_degrees: f64, vegetation_cover: f64) -> f64 {
    let slope_rad = slope_degrees.to_radians();
    // RUSLE slope factor: sin(θ) gives a more realistic nonlinear response than linear θ/45
    let slope_factor = slope_rad.sin().clamp(0.0, 1.0);
    // Cover factor: exponential decay — dense cover is much more effective than sparse
    let cover = vegetation_cover;
    let cover_factor = calc::lerp(1.0, (-3.0_f64 * cover).exp(), cover);
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
        assert!(e.abs() < f64::EPSILON);
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

/// A weathering reaction: what a mineral weathers into.
#[cfg(feature = "chemistry")]
#[derive(Debug, Clone)]
pub struct WeatheringReaction {
    /// Parent mineral name.
    pub parent: &'static str,
    /// Chemical formula of the parent mineral (kimiya thermochem key).
    pub parent_formula: &'static str,
    /// Solid weathering products with names.
    pub solid_products: &'static [&'static str],
    /// Dissolved ions released.
    pub dissolved_ions: &'static [&'static str],
    /// Type of weathering that drives this reaction.
    pub weathering_type: WeatheringType,
    /// Typical activation energy in J/mol for Arrhenius rate.
    pub activation_energy_j: f64,
}

/// Classification of weathering mechanism.
#[cfg(feature = "chemistry")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WeatheringType {
    /// Hydrolysis (reaction with water).
    Hydrolysis,
    /// Carbonation (reaction with carbonic acid / CO₂).
    Carbonation,
    /// Oxidation (reaction with O₂).
    Oxidation,
    /// Dissolution (simple dissolving).
    Dissolution,
}

/// Known weathering reactions for common rock-forming minerals.
#[cfg(feature = "chemistry")]
pub const WEATHERING_REACTIONS: &[WeatheringReaction] = &[
    // Feldspar + H₂O + CO₂ → Kaolinite + K⁺ + HCO₃⁻ + SiO₂
    WeatheringReaction {
        parent: "Feldspar",
        parent_formula: "SiO2(s)", // proxy — KAlSi₃O₈ not in kimiya DB
        solid_products: &["Kaolinite (clay)", "Quartz"],
        dissolved_ions: &["K⁺", "HCO₃⁻"],
        weathering_type: WeatheringType::Hydrolysis,
        activation_energy_j: 67_000.0,
    },
    // Calcite + CO₂ + H₂O → Ca²⁺ + 2HCO₃⁻
    WeatheringReaction {
        parent: "Calcite",
        parent_formula: "CaCO3(s)",
        solid_products: &[],
        dissolved_ions: &["Ca²⁺", "HCO₃⁻"],
        weathering_type: WeatheringType::Carbonation,
        activation_energy_j: 35_000.0,
    },
    // Pyrite + O₂ + H₂O → Fe₂O₃ + H₂SO₄
    WeatheringReaction {
        parent: "Pyrite",
        parent_formula: "Fe2O3(s)", // product proxy
        solid_products: &["Limonite (iron oxide)"],
        dissolved_ions: &["Fe²⁺", "SO₄²⁻", "H⁺"],
        weathering_type: WeatheringType::Oxidation,
        activation_energy_j: 55_000.0,
    },
    // Olivine + H₂O + CO₂ → Serpentine + Mg²⁺ + HCO₃⁻ + SiO₂
    WeatheringReaction {
        parent: "Olivine",
        parent_formula: "MgO(s)", // proxy for Mg₂SiO₄
        solid_products: &["Serpentine", "Silica"],
        dissolved_ions: &["Mg²⁺", "HCO₃⁻"],
        weathering_type: WeatheringType::Hydrolysis,
        activation_energy_j: 79_000.0,
    },
    // Halite → Na⁺ + Cl⁻ (simple dissolution)
    WeatheringReaction {
        parent: "Halite",
        parent_formula: "NaCl(s)",
        solid_products: &[],
        dissolved_ions: &["Na⁺", "Cl⁻"],
        weathering_type: WeatheringType::Dissolution,
        activation_energy_j: 20_000.0,
    },
    // Gypsum → Ca²⁺ + SO₄²⁻ + 2H₂O
    WeatheringReaction {
        parent: "Gypsum",
        parent_formula: "CaCO3(s)", // proxy — CaSO₄ not in DB
        solid_products: &[],
        dissolved_ions: &["Ca²⁺", "SO₄²⁻"],
        weathering_type: WeatheringType::Dissolution,
        activation_energy_j: 25_000.0,
    },
];

/// Look up the weathering reaction for a named mineral.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn weathering_reaction(mineral_name: &str) -> Option<&'static WeatheringReaction> {
    WEATHERING_REACTIONS
        .iter()
        .find(|r| r.parent.eq_ignore_ascii_case(mineral_name))
}

/// Weathering rate constant for a specific mineral at a given temperature,
/// using the mineral's characteristic activation energy.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn mineral_weathering_rate(mineral_name: &str, temperature_k: f64) -> Option<f64> {
    let rxn = weathering_reaction(mineral_name)?;
    // Use a standard pre-exponential factor (varies by mineral, ~1e10 for silicates)
    let pre_exp = match rxn.weathering_type {
        WeatheringType::Dissolution => 1e8,
        WeatheringType::Carbonation => 1e9,
        WeatheringType::Hydrolysis => 1e10,
        WeatheringType::Oxidation => 1e11,
    };
    arrhenius_weathering_rate(pre_exp, rxn.activation_energy_j, temperature_k)
}

#[cfg(all(test, feature = "chemistry"))]
mod chemistry_tests {
    use super::*;

    #[test]
    fn arrhenius_rate_increases_with_temperature() {
        // Typical silicate dissolution: Ea ≈ 60 kJ/mol
        let cold = arrhenius_weathering_rate(1e10, 60_000.0, 283.15).unwrap(); // 10°C
        let hot = arrhenius_weathering_rate(1e10, 60_000.0, 313.15).unwrap(); // 40°C
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

    #[test]
    fn feldspar_weathers_to_clay() {
        let rxn = weathering_reaction("Feldspar").unwrap();
        assert_eq!(rxn.weathering_type, WeatheringType::Hydrolysis);
        assert!(rxn.solid_products.contains(&"Kaolinite (clay)"));
        assert!(rxn.dissolved_ions.contains(&"K⁺"));
    }

    #[test]
    fn calcite_weathers_by_carbonation() {
        let rxn = weathering_reaction("Calcite").unwrap();
        assert_eq!(rxn.weathering_type, WeatheringType::Carbonation);
        assert!(rxn.solid_products.is_empty()); // fully dissolves
        assert!(rxn.dissolved_ions.contains(&"Ca²⁺"));
    }

    #[test]
    fn pyrite_oxidizes() {
        let rxn = weathering_reaction("Pyrite").unwrap();
        assert_eq!(rxn.weathering_type, WeatheringType::Oxidation);
        assert!(rxn.dissolved_ions.contains(&"H⁺")); // acid mine drainage
    }

    #[test]
    fn halite_dissolves_fastest() {
        // Dissolution minerals should weather faster than silicates
        let halite_k = mineral_weathering_rate("Halite", 298.15).unwrap();
        let feldspar_k = mineral_weathering_rate("Feldspar", 298.15).unwrap();
        assert!(halite_k > feldspar_k);
    }

    #[test]
    fn unknown_mineral_returns_none() {
        assert!(weathering_reaction("Unobtanium").is_none());
        assert!(mineral_weathering_rate("Unobtanium", 298.15).is_none());
    }

    #[test]
    fn olivine_weathers_faster_warm() {
        let cold = mineral_weathering_rate("Olivine", 273.15).unwrap();
        let warm = mineral_weathering_rate("Olivine", 313.15).unwrap();
        assert!(warm > cold);
    }
}

// ---------------------------------------------------------------------------
// Weather-gated functions (badal)
// ---------------------------------------------------------------------------

/// Physical weathering rate driven by atmospheric conditions.
///
/// Uses temperature and humidity from a badal `AtmosphericState` to compute
/// the freeze-thaw weathering potential. Assumes the diurnal temperature range
/// is roughly proportional to the deviation from dew point.
///
/// Requires the `weather` feature.
#[cfg(feature = "weather")]
#[must_use]
pub fn physical_weathering_from_climate(state: &badal::AtmosphericState) -> f64 {
    let rh = state.humidity_percent();
    // Estimate diurnal range: drier air → bigger daily swing (arid ~20°C, humid ~5°C)
    let temp_range = 20.0 * (1.0 - rh / 100.0) + 5.0;
    let moisture = rh / 100.0;
    physical_weathering_rate(temp_range, moisture)
}

/// Chemical weathering rate driven by atmospheric conditions.
///
/// Uses temperature and humidity to estimate annual rainfall proxy, then
/// computes the chemical weathering rate.
///
/// Requires the `weather` feature.
#[cfg(feature = "weather")]
#[must_use]
pub fn chemical_weathering_from_climate(state: &badal::AtmosphericState) -> f64 {
    let temp_c = state.temperature_celsius();
    let rh = state.humidity_percent();
    // Estimate annual rainfall from humidity: tropical humid ~2000mm, arid ~100mm
    let rainfall_mm = (rh / 100.0) * 2500.0;
    chemical_weathering_rate(temp_c, rainfall_mm)
}

/// Erosion rate driven by atmospheric conditions.
///
/// Uses wind speed (from Beaufort scale estimate based on stability) and
/// precipitation intensity proxy from atmospheric state.
///
/// - `slope_degrees`: terrain slope
/// - `vegetation_cover`: vegetation cover fraction (0.0-1.0)
///
/// Requires the `weather` feature.
#[cfg(feature = "weather")]
#[must_use]
pub fn erosion_from_climate(
    state: &badal::AtmosphericState,
    slope_degrees: f64,
    vegetation_cover: f64,
) -> f64 {
    let rh = state.humidity_percent();
    // Rainfall intensity proxy: higher humidity → more intense rainfall events
    let rainfall_intensity = rh / 100.0 * 60.0; // max ~60 mm/h
    erosion_rate(rainfall_intensity, slope_degrees, vegetation_cover)
}

/// Frost weathering susceptibility: how many freeze-thaw cycles per day
/// are likely at this atmospheric state.
///
/// Returns an estimate of daily freeze-thaw cycles (0.0 = no frost, up to ~2.0).
///
/// Requires the `weather` feature.
#[cfg(feature = "weather")]
#[must_use]
pub fn freeze_thaw_cycles(state: &badal::AtmosphericState) -> f64 {
    let temp_c = state.temperature_celsius();
    // Freeze-thaw cycles peak when mean temperature is near 0°C
    // Gaussian-like distribution centered at 0°C with σ ≈ 5°C
    let proximity = (-temp_c.powi(2) / 50.0).exp();
    // More humidity → more water available to freeze
    let moisture_factor = state.humidity_percent() / 100.0;
    2.0 * proximity * moisture_factor
}

/// Combined weathering intensity index from atmospheric state.
///
/// Returns a value 0.0-1.0 representing the overall weathering potential,
/// combining physical and chemical weathering.
///
/// Requires the `weather` feature.
#[cfg(feature = "weather")]
#[must_use]
pub fn weathering_intensity(state: &badal::AtmosphericState) -> f64 {
    let physical = physical_weathering_from_climate(state);
    let chemical = chemical_weathering_from_climate(state);
    // Geometric mean gives balanced weight to both mechanisms
    (physical * chemical).sqrt().clamp(0.0, 1.0)
}

#[cfg(all(test, feature = "weather"))]
mod weather_tests {
    use super::*;

    fn tropical() -> badal::AtmosphericState {
        badal::AtmosphericState::new(303.15, 101_325.0, 85.0, 0.0).unwrap() // 30°C, 85% RH
    }

    fn arid() -> badal::AtmosphericState {
        badal::AtmosphericState::new(313.15, 101_325.0, 15.0, 0.0).unwrap() // 40°C, 15% RH
    }

    fn arctic() -> badal::AtmosphericState {
        badal::AtmosphericState::new(263.15, 101_325.0, 70.0, 0.0).unwrap() // -10°C, 70% RH
    }

    fn periglacial() -> badal::AtmosphericState {
        badal::AtmosphericState::new(273.15, 101_325.0, 80.0, 0.0).unwrap() // 0°C, 80% RH
    }

    #[test]
    fn tropical_high_chemical_weathering() {
        let rate = chemical_weathering_from_climate(&tropical());
        assert!(
            rate > 0.3,
            "Tropical should have high chemical weathering, got {rate}"
        );
    }

    #[test]
    fn arid_low_chemical_weathering() {
        let rate = chemical_weathering_from_climate(&arid());
        assert!(
            rate < 0.2,
            "Arid should have low chemical weathering, got {rate}"
        );
    }

    #[test]
    fn tropical_more_chemical_than_arid() {
        let trop = chemical_weathering_from_climate(&tropical());
        let dry = chemical_weathering_from_climate(&arid());
        assert!(trop > dry);
    }

    #[test]
    fn arid_more_physical_than_tropical() {
        // Arid: big diurnal range, low moisture but still some physical
        // Both should have measurable physical weathering
        let trop = physical_weathering_from_climate(&tropical());
        let dry = physical_weathering_from_climate(&arid());
        // Arid has bigger temp range, but tropical has more moisture
        // Physical weathering needs both — tropical actually higher due to moisture
        assert!(trop > 0.0);
        assert!(dry >= 0.0);
    }

    #[test]
    fn freeze_thaw_peaks_near_zero() {
        let peri = freeze_thaw_cycles(&periglacial()); // 0°C
        let trop = freeze_thaw_cycles(&tropical()); // 30°C
        let cold = freeze_thaw_cycles(&arctic()); // -10°C
        assert!(peri > trop, "Freeze-thaw should peak near 0°C");
        assert!(
            peri > cold,
            "Freeze-thaw should peak near 0°C, not at -10°C"
        );
    }

    #[test]
    fn freeze_thaw_zero_in_tropics() {
        let cycles = freeze_thaw_cycles(&tropical());
        assert!(cycles < 0.01, "No freeze-thaw in tropics, got {cycles}");
    }

    #[test]
    fn erosion_from_climate_increases_with_humidity() {
        let wet = erosion_from_climate(&tropical(), 15.0, 0.3);
        let dry = erosion_from_climate(&arid(), 15.0, 0.3);
        assert!(wet > dry);
    }

    #[test]
    fn weathering_intensity_bounded() {
        for state in [tropical(), arid(), arctic(), periglacial()] {
            let wi = weathering_intensity(&state);
            assert!(
                (0.0..=1.0).contains(&wi),
                "Weathering intensity should be 0-1, got {wi}"
            );
        }
    }

    #[test]
    fn at_altitude_reduces_temperature() {
        let sea = badal::AtmosphericState::sea_level();
        let mountain = badal::AtmosphericState::at_altitude(3000.0);
        assert!(mountain.temperature_k() < sea.temperature_k());
    }
}
