//! Geothermal modeling — heat flow, thermal gradients, and metamorphic phase
//! transitions powered by the ushma thermodynamics crate.
//!
//! Requires the `thermodynamics` feature.

use ushma::transfer;
use ushma::state;

/// Geothermal heat flux via Fourier's law of conduction.
///
/// - `conductivity`: thermal conductivity of rock in W/(m·K)
/// - `area`: cross-sectional area in m²
/// - `t_deep`: deep temperature in K
/// - `t_surface`: surface temperature in K
/// - `depth`: depth in metres
///
/// Returns heat flux in watts.
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
#[must_use]
pub fn gibbs_energy(enthalpy: f64, temperature: f64, entropy: f64) -> f64 {
    ushma::entropy::gibbs(enthalpy, temperature, entropy)
}

/// Check if a metamorphic reaction is thermodynamically spontaneous.
/// Returns `true` when ΔG < 0.
#[must_use]
pub fn is_reaction_spontaneous(delta_h: f64, temperature: f64, delta_s: f64) -> bool {
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
#[must_use]
pub fn volatile_pressure(moles: f64, temperature_k: f64, volume_m3: f64) -> Option<f64> {
    state::ideal_gas_pressure(moles, temperature_k, volume_m3).ok()
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
        assert!(is_reaction_spontaneous(-50_000.0, 298.15, 100.0));
        // Endothermic with small entropy gain at low temp → not spontaneous
        assert!(!is_reaction_spontaneous(50_000.0, 298.15, 10.0));
    }

    #[test]
    fn volatile_pressure_in_pore() {
        let p = volatile_pressure(1.0, 473.15, 0.001); // 1 mol CO₂, 200°C, 1 litre
        assert!(p.is_some());
        // PV=nRT → P ≈ 3.93 MPa
        let pa = p.unwrap();
        assert!(pa > 3_000_000.0 && pa < 5_000_000.0);
    }
}
