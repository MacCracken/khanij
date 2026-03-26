//! Geothermal modeling — heat flow, thermal gradients, and metamorphic phase
//! transitions powered by the ushma thermodynamics crate.
//!
//! Requires the `thermodynamics` feature.

use serde::{Deserialize, Serialize};
use ushma::state;
use ushma::transfer;

/// Geothermal heat flux via Fourier's law of conduction.
///
/// - `conductivity`: thermal conductivity of rock in W/(m·K)
/// - `area`: cross-sectional area in m²
/// - `t_deep`: deep temperature in K
/// - `t_surface`: surface temperature in K
/// - `depth`: depth in metres
///
/// Returns heat flux in watts.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let q = heat_flux(2.5, 1.0, 373.15, 288.15, 1000.0);
/// assert!(q.unwrap() > 0.0);
/// ```
#[must_use]
pub fn heat_flux(
    conductivity: f64,
    area: f64,
    t_deep: f64,
    t_surface: f64,
    depth: f64,
) -> Option<f64> {
    transfer::conduction(conductivity, area, t_deep, t_surface, depth).ok()
}

/// Temperature at depth given a surface temperature and geothermal gradient.
///
/// - `surface_temp_k`: surface temperature in kelvin
/// - `gradient_k_per_m`: geothermal gradient (typical: ~0.025 K/m = 25°C/km)
/// - `depth_m`: depth in metres
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let t = temperature_at_depth(288.15, 0.025, 1000.0);
/// assert!((t - 313.15).abs() < 0.01);
/// ```
#[must_use]
pub fn temperature_at_depth(surface_temp_k: f64, gradient_k_per_m: f64, depth_m: f64) -> f64 {
    surface_temp_k + gradient_k_per_m * depth_m
}

/// Thermal diffusivity of a rock given its thermal properties.
///
/// - `conductivity`: W/(m·K)
/// - `density`: kg/m³
/// - `specific_heat`: J/(kg·K)
///
/// Returns diffusivity in m²/s.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let alpha = rock_thermal_diffusivity(2.5, 2700.0, 790.0).unwrap();
/// assert!(alpha > 1e-7 && alpha < 1e-5);
/// ```
#[must_use]
pub fn rock_thermal_diffusivity(
    conductivity: f64,
    density: f64,
    specific_heat: f64,
) -> Option<f64> {
    transfer::thermal_diffusivity(conductivity, density, specific_heat).ok()
}

/// Heat stored in a rock body (Q = mcΔT).
///
/// - `mass_kg`: mass of rock body in kg
/// - `specific_heat`: J/(kg·K)
/// - `delta_t`: temperature change in K
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let q = heat_stored(1.0, 790.0, 100.0);
/// assert!((q - 79_000.0).abs() < 1.0);
/// ```
#[must_use]
pub fn heat_stored(mass_kg: f64, specific_heat: f64, delta_t: f64) -> f64 {
    transfer::heat_stored(mass_kg, specific_heat, delta_t)
}

/// Pressure at depth in the lithosphere (lithostatic pressure).
///
/// - `density`: average rock density in kg/m³
/// - `gravity`: gravitational acceleration (9.81 m/s²)
/// - `depth_m`: depth in metres
///
/// Returns pressure in pascals.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let p = lithostatic_pressure(2700.0, 9.81, 1000.0);
/// assert!((p - 26_487_000.0).abs() < 1000.0);
/// ```
#[must_use]
pub fn lithostatic_pressure(density: f64, gravity: f64, depth_m: f64) -> f64 {
    density * gravity * depth_m
}

/// Gibbs free energy to assess metamorphic reaction feasibility.
/// G = H - TS; negative ΔG means the reaction is spontaneous.
///
/// - `enthalpy`: H in joules
/// - `temperature`: T in kelvin
/// - `entropy`: S in J/K
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let g = gibbs_energy(-50_000.0, 298.15, 100.0);
/// assert!(g < 0.0);
/// ```
#[must_use]
pub fn gibbs_energy(enthalpy: f64, temperature: f64, entropy: f64) -> f64 {
    ushma::entropy::gibbs(enthalpy, temperature, entropy)
}

/// Check if a metamorphic reaction is thermodynamically spontaneous.
/// Returns `true` when ΔG < 0.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(is_spontaneous(-50_000.0, 298.15, 100.0));
/// assert!(!is_spontaneous(50_000.0, 298.15, 10.0));
/// ```
#[must_use]
pub fn is_spontaneous(delta_h: f64, temperature: f64, delta_s: f64) -> bool {
    gibbs_energy(delta_h, temperature, delta_s) < 0.0
}

/// Pressure of a volatile phase (e.g., CO₂, H₂O) at depth using the ideal gas
/// law from ushma.
///
/// - `moles`: amount of volatile in mol
/// - `temperature_k`: temperature in kelvin
/// - `volume_m3`: available pore volume in m³
///
/// Returns pressure in pascals.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let p = volatile_pressure(1.0, 473.15, 0.001).unwrap();
/// assert!(p > 3_000_000.0 && p < 5_000_000.0);
/// ```
#[must_use]
pub fn volatile_pressure(moles: f64, temperature_k: f64, volume_m3: f64) -> Option<f64> {
    state::ideal_gas_pressure(moles, temperature_k, volume_m3).ok()
}

/// Metamorphic facies classification based on pressure-temperature conditions.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let facies = classify_facies(350.0, 0.4);
/// assert_eq!(facies, MetamorphicFacies::Greenschist);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MetamorphicFacies {
    /// Low T, low P (< 300°C, < 0.4 GPa). Diagenesis to very low-grade.
    Zeolite,
    /// Low T, moderate P (200-450°C, 0.2-0.8 GPa). Chlorite, epidote stable.
    Greenschist,
    /// Moderate T, moderate P (450-700°C, 0.3-1.0 GPa). Amphibole, garnet.
    Amphibolite,
    /// High T, moderate-high P (> 700°C, 0.3-1.5 GPa). Pyroxene, sillimanite.
    Granulite,
    /// Low T, high P (200-500°C, > 0.6 GPa). Glaucophane, lawsonite.
    Blueschist,
    /// Moderate-high T, very high P (> 450°C, > 1.2 GPa). Omphacite, garnet.
    Eclogite,
    /// Contact metamorphism. High T, low P (> 500°C, < 0.3 GPa). Hornfels.
    ContactHornfels,
}

/// Classify metamorphic facies from temperature and pressure.
///
/// - `temperature_c`: temperature in degrees Celsius
/// - `pressure_gpa`: pressure in gigapascals
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_facies(150.0, 0.1), MetamorphicFacies::Zeolite);
/// assert_eq!(classify_facies(600.0, 1.5), MetamorphicFacies::Eclogite);
/// ```
#[must_use]
pub fn classify_facies(temperature_c: f64, pressure_gpa: f64) -> MetamorphicFacies {
    // Eclogite: high P, moderate-high T
    if pressure_gpa > 1.2 && temperature_c > 450.0 {
        MetamorphicFacies::Eclogite
    }
    // Blueschist: high P, low T
    else if pressure_gpa > 0.6 && temperature_c < 500.0 {
        MetamorphicFacies::Blueschist
    }
    // Contact hornfels: high T, low P
    else if temperature_c > 500.0 && pressure_gpa < 0.3 {
        MetamorphicFacies::ContactHornfels
    }
    // Granulite: high T
    else if temperature_c > 700.0 {
        MetamorphicFacies::Granulite
    }
    // Amphibolite: moderate-high T
    else if temperature_c > 450.0 {
        MetamorphicFacies::Amphibolite
    }
    // Greenschist: moderate T
    else if temperature_c > 200.0 {
        MetamorphicFacies::Greenschist
    }
    // Zeolite: low T
    else {
        MetamorphicFacies::Zeolite
    }
}

/// Classify metamorphic facies at a given depth using typical crustal values.
///
/// - `depth_km`: depth in kilometres
/// - `gradient_c_per_km`: geothermal gradient in °C/km (typical: 25)
/// - `surface_temp_c`: surface temperature in °C
/// - `rock_density`: average rock density in kg/m³ (typical: 2700)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let f = facies_at_depth(5.0, 25.0, 15.0, 2700.0);
/// assert_eq!(f, MetamorphicFacies::Zeolite);
/// ```
#[must_use]
pub fn facies_at_depth(
    depth_km: f64,
    gradient_c_per_km: f64,
    surface_temp_c: f64,
    rock_density: f64,
) -> MetamorphicFacies {
    let temp_c = surface_temp_c + gradient_c_per_km * depth_km;
    let pressure_pa = lithostatic_pressure(rock_density, 9.81, depth_km * 1000.0);
    let pressure_gpa = pressure_pa / 1e9;
    classify_facies(temp_c, pressure_gpa)
}

/// Temperature of a cooling magma intrusion at its centre after time `t`.
///
/// Uses the 1D conduction cooling model: T(t) = T_country + (T_magma - T_country) · exp(-π²·α·t / R²)
///
/// - `magma_temp_k`: initial magma temperature in kelvin (e.g., 1473 K for basalt)
/// - `country_temp_k`: country rock temperature in kelvin
/// - `half_width_m`: half-thickness of the intrusion in metres
/// - `diffusivity_m2_s`: thermal diffusivity of the intrusion in m²/s
/// - `time_seconds`: elapsed time since emplacement
///
/// Returns temperature in kelvin at the centre of the intrusion.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let t = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, 0.0);
/// assert!((t - 1473.0).abs() < 0.01);
/// ```
#[must_use]
pub fn intrusion_cooling(
    magma_temp_k: f64,
    country_temp_k: f64,
    half_width_m: f64,
    diffusivity_m2_s: f64,
    time_seconds: f64,
) -> f64 {
    let decay = (-std::f64::consts::PI.powi(2) * diffusivity_m2_s * time_seconds
        / half_width_m.powi(2))
    .exp();
    country_temp_k + (magma_temp_k - country_temp_k) * decay
}

/// Time for a magma intrusion to cool to a target temperature at its centre.
///
/// Inverts the cooling model: t = -R² · ln((T_target - T_country)/(T_magma - T_country)) / (π²·α)
///
/// Returns time in seconds, or `None` if the target is outside the valid range.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let time = intrusion_cooling_time(1473.0, 573.0, 800.0, 50.0, 1e-6);
/// assert!(time.unwrap() > 0.0);
/// ```
#[must_use]
pub fn intrusion_cooling_time(
    magma_temp_k: f64,
    country_temp_k: f64,
    target_temp_k: f64,
    half_width_m: f64,
    diffusivity_m2_s: f64,
) -> Option<f64> {
    if target_temp_k <= country_temp_k || target_temp_k >= magma_temp_k {
        return None;
    }
    let ratio = (target_temp_k - country_temp_k) / (magma_temp_k - country_temp_k);
    let t = -half_width_m.powi(2) * ratio.ln() / (std::f64::consts::PI.powi(2) * diffusivity_m2_s);
    Some(t)
}

/// Contact aureole temperature at distance from an intrusion.
///
/// Simple model: T(x) = T_country + (T_magma - T_country) · exp(-x / half_width)
///
/// - `distance_m`: distance from intrusion contact in metres
/// - `half_width_m`: half-thickness of the intrusion
/// - `magma_temp_k`: magma temperature at contact
/// - `country_temp_k`: far-field country rock temperature
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let t = contact_aureole_temperature(0.0, 50.0, 1473.0, 573.0);
/// assert!((t - 1473.0).abs() < 0.01);
/// ```
#[must_use]
pub fn contact_aureole_temperature(
    distance_m: f64,
    half_width_m: f64,
    magma_temp_k: f64,
    country_temp_k: f64,
) -> f64 {
    country_temp_k + (magma_temp_k - country_temp_k) * (-distance_m / half_width_m).exp()
}

/// Thermal conductivities of common rock types in W/(m·K).
pub mod conductivity {
    pub const GRANITE: f64 = 2.5;
    pub const BASALT: f64 = 1.7;
    pub const SANDSTONE: f64 = 2.3;
    pub const LIMESTONE: f64 = 2.5;
    pub const MARBLE: f64 = 2.9;
    pub const SHALE: f64 = 1.5;
    pub const GNEISS: f64 = 2.7;
    pub const QUARTZITE: f64 = 5.0;
}

/// Specific heat capacities of common rock types in J/(kg·K).
pub mod specific_heat {
    pub const GRANITE: f64 = 790.0;
    pub const BASALT: f64 = 840.0;
    pub const SANDSTONE: f64 = 920.0;
    pub const LIMESTONE: f64 = 840.0;
    pub const MARBLE: f64 = 880.0;
    pub const SHALE: f64 = 760.0;
    pub const GNEISS: f64 = 800.0;
    pub const QUARTZITE: f64 = 740.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature_increases_with_depth() {
        let surface = 288.15; // 15°C
        let gradient = 0.025; // 25°C/km
        let t_1km = temperature_at_depth(surface, gradient, 1000.0);
        let t_5km = temperature_at_depth(surface, gradient, 5000.0);
        assert!(t_1km > surface);
        assert!(t_5km > t_1km);
        assert!((t_1km - 313.15).abs() < 0.01); // 40°C at 1km
    }

    #[test]
    fn lithostatic_pressure_at_depth() {
        // ~26.5 MPa at 1km depth with granite density
        let p = lithostatic_pressure(2700.0, 9.81, 1000.0);
        assert!((p - 26_487_000.0).abs() < 1000.0);
    }

    #[test]
    fn heat_flux_positive_downward() {
        let q = heat_flux(conductivity::GRANITE, 1.0, 373.15, 288.15, 1000.0);
        assert!(q.is_some());
        assert!(q.unwrap() > 0.0);
    }

    #[test]
    fn granite_thermal_diffusivity() {
        let d = rock_thermal_diffusivity(conductivity::GRANITE, 2700.0, specific_heat::GRANITE);
        assert!(d.is_some());
        let alpha = d.unwrap();
        // Expected ~1.17e-6 m²/s
        assert!(alpha > 1e-7 && alpha < 1e-5);
    }

    #[test]
    fn heat_storage() {
        // 1 kg of granite heated 100K → ~79 kJ
        let q = heat_stored(1.0, specific_heat::GRANITE, 100.0);
        assert!((q - 79_000.0).abs() < 1000.0);
    }

    #[test]
    fn gibbs_spontaneity() {
        // Exothermic reaction with positive entropy change → always spontaneous
        assert!(is_spontaneous(-50_000.0, 298.15, 100.0));
        // Endothermic with small entropy gain at low temp → not spontaneous
        assert!(!is_spontaneous(50_000.0, 298.15, 10.0));
    }

    #[test]
    fn volatile_pressure_in_pore() {
        let p = volatile_pressure(1.0, 473.15, 0.001); // 1 mol CO₂, 200°C, 1 litre
        assert!(p.is_some());
        // PV=nRT → P ≈ 3.93 MPa
        let pa = p.unwrap();
        assert!(pa > 3_000_000.0 && pa < 5_000_000.0);
    }

    #[test]
    fn facies_zeolite_shallow() {
        assert_eq!(classify_facies(150.0, 0.1), MetamorphicFacies::Zeolite);
    }

    #[test]
    fn facies_greenschist() {
        assert_eq!(classify_facies(350.0, 0.4), MetamorphicFacies::Greenschist);
    }

    #[test]
    fn facies_amphibolite() {
        assert_eq!(classify_facies(550.0, 0.6), MetamorphicFacies::Amphibolite);
    }

    #[test]
    fn facies_granulite() {
        assert_eq!(classify_facies(800.0, 0.8), MetamorphicFacies::Granulite);
    }

    #[test]
    fn facies_blueschist_high_p_low_t() {
        assert_eq!(classify_facies(300.0, 1.0), MetamorphicFacies::Blueschist);
    }

    #[test]
    fn facies_eclogite_high_p_high_t() {
        assert_eq!(classify_facies(600.0, 1.5), MetamorphicFacies::Eclogite);
    }

    #[test]
    fn facies_contact_hornfels_high_t_low_p() {
        assert_eq!(
            classify_facies(600.0, 0.2),
            MetamorphicFacies::ContactHornfels
        );
    }

    #[test]
    fn facies_at_depth_shallow_is_zeolite() {
        // 5km, 25°C/km gradient, 15°C surface, granite
        let f = facies_at_depth(5.0, 25.0, 15.0, 2700.0);
        assert_eq!(f, MetamorphicFacies::Zeolite);
    }

    #[test]
    fn facies_at_depth_deep_is_higher_grade() {
        let shallow = facies_at_depth(5.0, 25.0, 15.0, 2700.0);
        let deep = facies_at_depth(20.0, 25.0, 15.0, 2700.0);
        // Deeper should be higher grade (not zeolite)
        assert_ne!(deep, shallow);
    }

    #[test]
    fn intrusion_starts_at_magma_temp() {
        let t = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, 0.0);
        assert!((t - 1473.0).abs() < 0.01);
    }

    #[test]
    fn intrusion_cools_over_time() {
        let early = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, 1_000_000.0);
        let late = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, 100_000_000.0);
        assert!(late < early);
        assert!(late >= 573.0); // never below country rock
    }

    #[test]
    fn intrusion_approaches_country_rock() {
        // After very long time, should approach country rock temp
        let t = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, 1e12);
        assert!((t - 573.0).abs() < 1.0);
    }

    #[test]
    fn cooling_time_roundtrip() {
        let target = 800.0;
        let time = intrusion_cooling_time(1473.0, 573.0, target, 50.0, 1e-6).unwrap();
        let recovered = intrusion_cooling(1473.0, 573.0, 50.0, 1e-6, time);
        assert!((recovered - target).abs() < 0.1);
    }

    #[test]
    fn cooling_time_invalid_target() {
        // Target below country rock
        assert!(intrusion_cooling_time(1473.0, 573.0, 500.0, 50.0, 1e-6).is_none());
        // Target above magma
        assert!(intrusion_cooling_time(1473.0, 573.0, 1500.0, 50.0, 1e-6).is_none());
    }

    #[test]
    fn contact_aureole_at_contact() {
        let t = contact_aureole_temperature(0.0, 50.0, 1473.0, 573.0);
        assert!((t - 1473.0).abs() < 0.01);
    }

    #[test]
    fn contact_aureole_decays_with_distance() {
        let near = contact_aureole_temperature(10.0, 50.0, 1473.0, 573.0);
        let far = contact_aureole_temperature(100.0, 50.0, 1473.0, 573.0);
        assert!(near > far);
        assert!(far > 573.0);
    }
}
