//! Hydrology — groundwater flow, sediment transport, and surface water modeling
//! powered by the pravash fluid dynamics crate.
//!
//! Requires the `fluids` feature.

use pravash::buoyancy;
use pravash::{FluidConfig, FluidParticle};

/// Fluid presets for geological contexts.
pub mod fluids {
    use pravash::FluidMaterial;

    /// Fresh groundwater at ~15°C.
    pub const GROUNDWATER: FluidMaterial = FluidMaterial::WATER;

    /// Basaltic lava (~1100°C). High density, very high viscosity.
    pub const LAVA: FluidMaterial = FluidMaterial::LAVA;

    /// Create a brine fluid (saline groundwater).
    /// - `salinity_fraction`: salt mass fraction (0.0-0.35)
    #[must_use]
    pub fn brine(salinity_fraction: f64) -> Option<FluidMaterial> {
        if !(0.0..=0.35).contains(&salinity_fraction) {
            return None;
        }
        // Density increases ~0.7 kg/m³ per 1‰ salinity
        let density = 1000.0 + 700.0 * salinity_fraction;
        // Viscosity increases slightly with salinity
        let viscosity = 0.001 * (1.0 + 1.5 * salinity_fraction);
        FluidMaterial::custom(density, viscosity, 0.073, 1500.0).ok()
    }

    /// Create a sediment-laden flow (turbidity current / debris flow).
    /// - `sediment_fraction`: volume fraction of suspended sediment (0.0-0.6)
    #[must_use]
    pub fn sediment_laden(sediment_fraction: f64) -> Option<FluidMaterial> {
        if !(0.0..=0.6).contains(&sediment_fraction) {
            return None;
        }
        // Mixture density: water + sediment (quartz ~2650 kg/m³)
        let density = 1000.0 * (1.0 - sediment_fraction) + 2650.0 * sediment_fraction;
        // Einstein viscosity for dilute suspensions: μ_eff = μ(1 + 2.5φ)
        let viscosity = 0.001 * (1.0 + 2.5 * sediment_fraction);
        FluidMaterial::custom(density, viscosity, 0.073, 1500.0).ok()
    }
}

/// Terminal settling velocity of a sediment grain using Stokes' law
/// (valid for Re < 1, i.e., fine silt and clay).
///
/// - `grain_density`: density of grain in kg/m³ (quartz: 2650)
/// - `fluid_density`: density of fluid in kg/m³ (water: 1000)
/// - `grain_diameter_m`: grain diameter in metres
/// - `fluid_viscosity`: dynamic viscosity in Pa·s (water: 0.001)
/// - `gravity`: gravitational acceleration (9.81 m/s²)
#[must_use]
pub fn stokes_settling_velocity(
    grain_density: f64,
    fluid_density: f64,
    grain_diameter_m: f64,
    fluid_viscosity: f64,
    gravity: f64,
) -> f64 {
    let delta_rho = grain_density - fluid_density;
    (delta_rho * gravity * grain_diameter_m.powi(2)) / (18.0 * fluid_viscosity)
}

/// Reynolds number for flow around a sediment grain.
/// Determines whether Stokes' law is valid (Re < 1).
#[must_use]
pub fn grain_reynolds_number(
    fluid_density: f64,
    velocity: f64,
    grain_diameter_m: f64,
    fluid_viscosity: f64,
) -> Option<f64> {
    buoyancy::reynolds_number(fluid_density, velocity, grain_diameter_m, fluid_viscosity).ok()
}

/// Flow regime classification for sediment transport.
#[must_use]
pub fn flow_regime(
    fluid_density: f64,
    velocity: f64,
    channel_depth_m: f64,
    fluid_viscosity: f64,
) -> Option<buoyancy::FlowRegime> {
    let re = buoyancy::reynolds_number(fluid_density, velocity, channel_depth_m, fluid_viscosity).ok()?;
    if re < 500.0 {
        Some(buoyancy::FlowRegime::Laminar)
    } else if re < 2000.0 {
        Some(buoyancy::FlowRegime::Transitional)
    } else {
        Some(buoyancy::FlowRegime::Turbulent)
    }
}

/// Buoyancy force on a submerged rock or mineral grain.
///
/// - `fluid_density`: kg/m³
/// - `gravity`: m/s²
/// - `displaced_volume`: m³
#[must_use]
pub fn buoyancy_force(fluid_density: f64, gravity: f64, displaced_volume: f64) -> f64 {
    buoyancy::buoyancy_force(fluid_density, gravity, displaced_volume)
}

/// Drag force on a sediment grain moving through fluid.
///
/// - `fluid_density`: kg/m³
/// - `velocity`: m/s
/// - `drag_coefficient`: dimensionless (sphere ≈ 0.47)
/// - `cross_section_area`: m²
#[must_use]
pub fn sediment_drag_force(
    fluid_density: f64,
    velocity: f64,
    drag_coefficient: f64,
    cross_section_area: f64,
) -> f64 {
    buoyancy::drag_force(fluid_density, velocity, drag_coefficient, cross_section_area)
}

/// Terminal velocity of a falling rock or grain using pravash.
///
/// - `mass`: kg
/// - `gravity`: m/s²
/// - `fluid_density`: kg/m³
/// - `drag_coefficient`: dimensionless
/// - `area`: cross-sectional area in m²
#[must_use]
pub fn terminal_velocity(
    mass: f64,
    gravity: f64,
    fluid_density: f64,
    drag_coefficient: f64,
    area: f64,
) -> Option<f64> {
    buoyancy::terminal_velocity(mass, gravity, fluid_density, drag_coefficient, area).ok()
}

/// Darcy's law for groundwater flow through porous rock.
///
/// - `hydraulic_conductivity`: K in m/s (depends on rock type)
/// - `hydraulic_gradient`: dimensionless (Δh/Δl)
/// - `area`: cross-sectional area in m²
///
/// Returns volumetric flow rate in m³/s.
#[must_use]
pub fn darcy_flow(hydraulic_conductivity: f64, hydraulic_gradient: f64, area: f64) -> f64 {
    hydraulic_conductivity * hydraulic_gradient * area
}

/// Hydraulic conductivity values for common rock types in m/s.
pub mod conductivity {
    pub const GRAVEL: f64 = 1e-2;
    pub const COARSE_SAND: f64 = 1e-3;
    pub const FINE_SAND: f64 = 1e-5;
    pub const SILT: f64 = 1e-7;
    pub const CLAY: f64 = 1e-9;
    pub const SANDSTONE: f64 = 1e-6;
    pub const FRACTURED_GRANITE: f64 = 1e-6;
    pub const UNFRACTURED_GRANITE: f64 = 1e-12;
    pub const LIMESTONE_KARST: f64 = 1e-2;
    pub const LIMESTONE_INTACT: f64 = 1e-8;
}

/// Create a pravash `FluidConfig` for a 2D surface water simulation domain.
///
/// - `width`: domain width in metres
/// - `height`: domain height in metres
/// - `smoothing_radius`: SPH kernel radius (typical: 0.1-1.0 m)
#[must_use]
pub fn surface_water_config(width: f64, height: f64, smoothing_radius: f64) -> FluidConfig {
    use hisab::DVec3;
    let mut config = FluidConfig::water_2d();
    config.smoothing_radius = smoothing_radius;
    config.bounds_max = DVec3::new(width, height, 0.0);
    config
}

/// Create a water particle at the given 2D position for SPH simulation.
#[must_use]
pub fn water_particle(x: f64, y: f64) -> FluidParticle {
    FluidParticle::new_2d(x, y, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stokes_settling_sand_grain() {
        // Medium sand grain: 0.5mm diameter, quartz density
        let v = stokes_settling_velocity(2650.0, 1000.0, 0.0005, 0.001, 9.81);
        assert!(v > 0.0, "sand grain should settle");
        // Expected ~0.22 m/s for 0.5mm grain (Stokes overestimates for larger grains)
        assert!(v > 0.1 && v < 1.0);
    }

    #[test]
    fn clay_settles_slower_than_sand() {
        let sand = stokes_settling_velocity(2650.0, 1000.0, 0.001, 0.001, 9.81);
        let clay = stokes_settling_velocity(2650.0, 1000.0, 0.00001, 0.001, 9.81);
        assert!(sand > clay);
    }

    #[test]
    fn darcy_flow_through_sandstone() {
        let q = darcy_flow(conductivity::SANDSTONE, 0.01, 100.0);
        assert!(q > 0.0);
        // K=1e-6, i=0.01, A=100 → Q=1e-6 m³/s
        assert!((q - 1e-6).abs() < 1e-10);
    }

    #[test]
    fn unfractured_granite_nearly_impermeable() {
        let q_granite = darcy_flow(conductivity::UNFRACTURED_GRANITE, 0.01, 100.0);
        let q_gravel = darcy_flow(conductivity::GRAVEL, 0.01, 100.0);
        assert!(q_gravel > q_granite * 1e6);
    }

    #[test]
    fn brine_denser_than_freshwater() {
        let brine = fluids::brine(0.035).unwrap(); // seawater salinity
        assert!(brine.density > 1000.0);
    }

    #[test]
    fn sediment_laden_flow_denser() {
        let clear = fluids::GROUNDWATER;
        let laden = fluids::sediment_laden(0.3).unwrap();
        assert!(laden.density > clear.density);
    }

    #[test]
    fn buoyancy_on_submerged_rock() {
        // 1 m³ of rock submerged in water
        let fb = buoyancy_force(1000.0, 9.81, 1.0);
        assert!((fb - 9810.0).abs() < 1.0);
    }

    #[test]
    fn surface_water_config_valid() {
        let config = surface_water_config(100.0, 50.0, 0.5);
        assert!((config.rest_density - 1000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn flow_regime_classification() {
        // Slow groundwater → laminar
        let regime = flow_regime(1000.0, 0.0001, 0.01, 0.001);
        assert_eq!(regime, Some(buoyancy::FlowRegime::Laminar));
    }

    #[test]
    fn grain_reynolds_small_for_clay() {
        let v = stokes_settling_velocity(2650.0, 1000.0, 0.00001, 0.001, 9.81);
        let re = grain_reynolds_number(1000.0, v, 0.00001, 0.001).unwrap();
        assert!(re < 1.0, "Clay grain Re should be <1 for Stokes validity, got {re}");
    }
}
