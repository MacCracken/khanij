//! Hydrology — groundwater flow, sediment transport, and surface water modeling
//! powered by the pravash fluid dynamics crate.
//!
//! Requires the `fluids` feature.

use pravash::buoyancy;
use pravash::{FluidConfig, FluidParticle};

/// Fluid presets for geological contexts.
///
/// # Examples
///
/// ```
/// # use khanij::hydrology::fluids;
/// let gw = fluids::GROUNDWATER;
/// assert!((gw.density - 1000.0).abs() < 1.0);
///
/// let lava = fluids::LAVA;
/// assert!(lava.density > 2500.0);
/// ```
pub mod fluids {
    use pravash::FluidMaterial;

    /// Fresh groundwater at ~15°C.
    pub const GROUNDWATER: FluidMaterial = FluidMaterial::WATER;

    /// Basaltic lava (~1100°C). High density, very high viscosity.
    pub const LAVA: FluidMaterial = FluidMaterial::LAVA;

    /// Create a brine fluid (saline groundwater).
    /// - `salinity_fraction`: salt mass fraction (0.0-0.35)
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::hydrology::fluids;
    /// let seawater = fluids::brine(0.035).unwrap();
    /// assert!(seawater.density > 1000.0);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::hydrology::fluids;
    /// let laden = fluids::sediment_laden(0.3).unwrap();
    /// assert!(laden.density > 1400.0);
    /// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let v = stokes_settling_velocity(2650.0, 1000.0, 0.0005, 0.001, 9.81);
/// assert!(v > 0.1 && v < 1.0);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let re = grain_reynolds_number(1000.0, 0.01, 0.001, 0.001).unwrap();
/// assert!(re > 0.0);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Slow groundwater flow is laminar
/// let regime = flow_regime(1000.0, 0.0001, 0.01, 0.001);
/// assert!(regime.is_some());
/// ```
#[must_use]
pub fn flow_regime(
    fluid_density: f64,
    velocity: f64,
    channel_depth_m: f64,
    fluid_viscosity: f64,
) -> Option<buoyancy::FlowRegime> {
    let re = buoyancy::reynolds_number(fluid_density, velocity, channel_depth_m, fluid_viscosity)
        .ok()?;
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let fb = buoyancy_force(1000.0, 9.81, 1.0);
/// assert!((fb - 9810.0).abs() < 1.0);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let fd = sediment_drag_force(1000.0, 1.0, 0.47, 0.01);
/// assert!(fd > 0.0);
/// ```
#[must_use]
pub fn sediment_drag_force(
    fluid_density: f64,
    velocity: f64,
    drag_coefficient: f64,
    cross_section_area: f64,
) -> f64 {
    buoyancy::drag_force(
        fluid_density,
        velocity,
        drag_coefficient,
        cross_section_area,
    )
}

/// Terminal velocity of a falling rock or grain using pravash.
///
/// - `mass`: kg
/// - `gravity`: m/s²
/// - `fluid_density`: kg/m³
/// - `drag_coefficient`: dimensionless
/// - `area`: cross-sectional area in m²
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let vt = terminal_velocity(0.01, 9.81, 1000.0, 0.47, 0.001).unwrap();
/// assert!(vt > 0.0);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// # use khanij::hydrology::conductivity;
/// let q = darcy_flow(conductivity::SANDSTONE, 0.01, 100.0);
/// assert!((q - 1e-6).abs() < 1e-10);
/// ```
#[must_use]
pub fn darcy_flow(hydraulic_conductivity: f64, hydraulic_gradient: f64, area: f64) -> f64 {
    hydraulic_conductivity * hydraulic_gradient * area
}

/// Aquifer storage coefficient (storativity) — dimensionless.
/// Fraction of water released per unit area per unit decline in head.
///
/// # Examples
///
/// ```
/// # use khanij::hydrology::storativity;
/// assert!(storativity::UNCONFINED_SAND > storativity::CONFINED_TYPICAL);
/// ```
pub mod storativity {
    /// Confined aquifer (typical range 1e-5 to 1e-3).
    pub const CONFINED_TYPICAL: f64 = 1e-4;
    /// Unconfined aquifer (specific yield, typically 0.01 to 0.30).
    pub const UNCONFINED_SAND: f64 = 0.25;
    pub const UNCONFINED_GRAVEL: f64 = 0.22;
    pub const UNCONFINED_SILT: f64 = 0.08;
    pub const UNCONFINED_CLAY: f64 = 0.03;
}

/// Theis well function W(u) — the exponential integral E₁(u).
///
/// W(u) = ∫_u^∞ (e⁻ᵗ / t) dt
///
/// Valid for u > 0. Used in the Theis equation for transient well drawdown.
/// Computed via hisab Gauss-Legendre quadrature.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let w = well_function(0.01);
/// assert!(w > well_function(0.1));
/// ```
#[must_use]
pub fn well_function(u: f64) -> f64 {
    use hisab::calc;
    if u <= 0.0 {
        return f64::INFINITY;
    }
    // Substitution t = e^s transforms ∫_u^∞ e^(-t)/t dt into ∫_{ln(u)}^∞ e^{-e^s} ds,
    // which is smooth and well-behaved for numerical integration.
    let s_lo = u.ln();
    let s_hi = (u + 40.0).ln(); // e^(-40) ≈ 4e-18, tail is negligible
    calc::integral_gauss_legendre(|s| (-s.exp()).exp(), s_lo, s_hi, 200)
        .unwrap_or(0.0)
        .max(0.0)
}

/// Theis equation: drawdown at distance `r` from a pumping well at time `t`.
///
/// s(r,t) = Q / (4π·T) · W(u)  where  u = r²·S / (4·T·t)
///
/// - `pumping_rate`: Q in m³/s
/// - `transmissivity`: T = K·b in m²/s (hydraulic conductivity × aquifer thickness)
/// - `storativity`: S (dimensionless)
/// - `distance_m`: r, distance from well in metres
/// - `time_seconds`: t, time since pumping began
///
/// Returns drawdown in metres.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let s = theis_drawdown(0.01, 0.001, 1e-4, 10.0, 86400.0);
/// assert!(s > 0.0);
/// ```
#[must_use]
pub fn theis_drawdown(
    pumping_rate: f64,
    transmissivity: f64,
    storativity: f64,
    distance_m: f64,
    time_seconds: f64,
) -> f64 {
    if transmissivity <= 0.0 || storativity <= 0.0 || time_seconds <= 0.0 || distance_m <= 0.0 {
        return 0.0;
    }
    let u = distance_m.powi(2) * storativity / (4.0 * transmissivity * time_seconds);
    let w = well_function(u);
    pumping_rate / (4.0 * std::f64::consts::PI * transmissivity) * w
}

/// Cooper-Jacob approximation of drawdown (valid for small u, i.e., long time
/// or small distance).
///
/// s ≈ Q / (4π·T) · [ln(2.25·T·t / (r²·S))]
///
/// Same parameters as `theis_drawdown`.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cj = cooper_jacob_drawdown(0.01, 0.001, 1e-4, 10.0, 864_000.0);
/// assert!(cj > 0.0);
/// ```
#[must_use]
pub fn cooper_jacob_drawdown(
    pumping_rate: f64,
    transmissivity: f64,
    storativity: f64,
    distance_m: f64,
    time_seconds: f64,
) -> f64 {
    if transmissivity <= 0.0 || storativity <= 0.0 || time_seconds <= 0.0 || distance_m <= 0.0 {
        return 0.0;
    }
    let arg = 2.25 * transmissivity * time_seconds / (distance_m.powi(2) * storativity);
    if arg <= 1.0 {
        return 0.0; // approximation not valid
    }
    pumping_rate / (4.0 * std::f64::consts::PI * transmissivity) * arg.ln()
}

/// Radius of influence of a pumping well (where drawdown ≈ 0).
///
/// R = √(2.25 · T · t / S)
///
/// - `transmissivity`: T in m²/s
/// - `storativity`: S (dimensionless)
/// - `time_seconds`: time since pumping began
///
/// Returns radius in metres.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let r = radius_of_influence(0.001, 1e-4, 86400.0);
/// assert!(r > 0.0);
/// ```
#[must_use]
pub fn radius_of_influence(transmissivity: f64, storativity: f64, time_seconds: f64) -> f64 {
    (2.25 * transmissivity * time_seconds / storativity).sqrt()
}

/// Hydraulic conductivity values for common rock types in m/s.
///
/// # Examples
///
/// ```
/// # use khanij::hydrology::conductivity;
/// assert!(conductivity::GRAVEL > conductivity::CLAY);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let config = surface_water_config(100.0, 50.0, 0.5);
/// assert!((config.rest_density - 1000.0).abs() < 1.0);
/// ```
#[must_use]
pub fn surface_water_config(width: f64, height: f64, smoothing_radius: f64) -> FluidConfig {
    use hisab::DVec3;
    let mut config = FluidConfig::water_2d();
    config.smoothing_radius = smoothing_radius;
    config.bounds_max = DVec3::new(width, height, 0.0);
    config
}

/// Create a water particle at the given 2D position for SPH simulation.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let p = water_particle(5.0, 3.0);
/// assert!((p.position.x - 5.0).abs() < f64::EPSILON);
/// ```
#[must_use]
pub fn water_particle(x: f64, y: f64) -> FluidParticle {
    FluidParticle::new_2d(x, y, 1.0)
}

/// Sediment transport regime based on the Hjulström curve.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let regime = transport_regime(0.001, 0.5);
/// assert_eq!(regime, TransportRegime::Erosion);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportRegime {
    /// Velocity too low — sediment deposits on the bed.
    Deposition,
    /// Velocity sufficient to keep sediment in transport but not erode new grains.
    Transport,
    /// Velocity high enough to erode (entrain) grains from the bed.
    Erosion,
}

/// Hjulström curve: critical erosion velocity for a given grain diameter.
///
/// Returns the approximate flow velocity (m/s) needed to erode a grain from
/// the bed. The curve is U-shaped: fine cohesive clays need high velocity,
/// medium sand needs the least, coarse gravel needs high velocity again.
///
/// - `grain_diameter_m`: grain diameter in metres
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let sand = hjulstrom_erosion_velocity(0.0005);
/// let clay = hjulstrom_erosion_velocity(0.00001);
/// assert!(sand < clay);
/// ```
#[must_use]
pub fn hjulstrom_erosion_velocity(grain_diameter_m: f64) -> f64 {
    let d = grain_diameter_m;
    if d < 1e-6 {
        // Colloidal — very high erosion velocity (cohesion dominates)
        5.0
    } else if d < 6.25e-5 {
        // Clay to fine silt — cohesion curve: v ∝ d^(-0.4)
        0.1 * (6.25e-5 / d).powf(0.4)
    } else {
        // Silt to boulders — gravity/inertia curve: v ∝ d^0.5
        4.0 * d.sqrt()
    }
}

/// Hjulström curve: critical deposition (settling) velocity.
///
/// Below this velocity, grains settle out of suspension.
/// Uses an empirical fit to the lower Hjulström curve.
///
/// - `grain_diameter_m`: grain diameter in metres
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let fine = hjulstrom_deposition_velocity(0.0001);
/// let coarse = hjulstrom_deposition_velocity(0.001);
/// assert!(coarse > fine);
/// ```
#[must_use]
pub fn hjulstrom_deposition_velocity(grain_diameter_m: f64) -> f64 {
    let d = grain_diameter_m;
    // Empirical lower Hjulström curve: settling-dominated for coarse grains,
    // with a floor for very fine particles (Brownian keeps them suspended).
    // Calibrated: ~0.01 m/s for fine sand (0.1mm), ~0.05 m/s for 1mm.
    let v = 1.6 * d.powf(0.8);
    v.max(0.001)
}

/// Classify the sediment transport regime for a given grain size and flow velocity.
///
/// - `grain_diameter_m`: grain diameter in metres
/// - `flow_velocity`: water velocity in m/s
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(transport_regime(0.001, 0.001), TransportRegime::Deposition);
/// ```
#[must_use]
pub fn transport_regime(grain_diameter_m: f64, flow_velocity: f64) -> TransportRegime {
    let v_erosion = hjulstrom_erosion_velocity(grain_diameter_m);
    let v_deposition = hjulstrom_deposition_velocity(grain_diameter_m);

    if flow_velocity >= v_erosion {
        TransportRegime::Erosion
    } else if flow_velocity >= v_deposition {
        TransportRegime::Transport
    } else {
        TransportRegime::Deposition
    }
}

/// Shields parameter for incipient motion of a sediment grain.
///
/// θ = τ / ((ρ_s - ρ_f) · g · d)
///
/// - `shear_stress`: bed shear stress in Pa
/// - `grain_density`: grain density in kg/m³ (quartz: 2650)
/// - `fluid_density`: fluid density in kg/m³ (water: 1000)
/// - `grain_diameter_m`: grain diameter in metres
/// - `gravity`: gravitational acceleration (9.81 m/s²)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let theta = shields_parameter(5.0, 2650.0, 1000.0, 0.001, 9.81);
/// assert!(theta > SHIELDS_CRITICAL);
/// ```
#[must_use]
pub fn shields_parameter(
    shear_stress: f64,
    grain_density: f64,
    fluid_density: f64,
    grain_diameter_m: f64,
    gravity: f64,
) -> f64 {
    shear_stress / ((grain_density - fluid_density) * gravity * grain_diameter_m)
}

/// Critical Shields parameter threshold for incipient motion (~0.047 for turbulent flow).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!((SHIELDS_CRITICAL - 0.047).abs() < 1e-10);
/// ```
pub const SHIELDS_CRITICAL: f64 = 0.047;

/// Check if a grain will be mobilised (Shields parameter > critical).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(is_grain_mobile(5.0, 2650.0, 1000.0, 0.001, 9.81));
/// assert!(!is_grain_mobile(0.01, 2650.0, 1000.0, 0.01, 9.81));
/// ```
#[must_use]
pub fn is_grain_mobile(
    shear_stress: f64,
    grain_density: f64,
    fluid_density: f64,
    grain_diameter_m: f64,
    gravity: f64,
) -> bool {
    shields_parameter(
        shear_stress,
        grain_density,
        fluid_density,
        grain_diameter_m,
        gravity,
    ) > SHIELDS_CRITICAL
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
        assert!(
            re < 1.0,
            "Clay grain Re should be <1 for Stokes validity, got {re}"
        );
    }

    // --- Hjulström curve tests ---

    #[test]
    fn hjulstrom_sand_easiest_to_erode() {
        // Medium sand (~0.5mm) should require less velocity than clay or gravel
        let sand = hjulstrom_erosion_velocity(0.0005);
        let clay = hjulstrom_erosion_velocity(0.00001);
        let gravel = hjulstrom_erosion_velocity(0.01);
        assert!(sand < clay, "Sand should erode easier than clay");
        assert!(sand < gravel, "Sand should erode easier than gravel");
    }

    #[test]
    fn hjulstrom_deposition_increases_with_size() {
        let fine = hjulstrom_deposition_velocity(0.0001);
        let coarse = hjulstrom_deposition_velocity(0.001);
        assert!(coarse > fine);
    }

    #[test]
    fn transport_regime_classification() {
        let d = 0.001; // 1mm sand
        let v_erosion = hjulstrom_erosion_velocity(d);
        let v_deposition = hjulstrom_deposition_velocity(d);

        assert_eq!(
            transport_regime(d, v_erosion + 0.1),
            TransportRegime::Erosion
        );
        // Midpoint between deposition and erosion velocity → transport
        let v_transport = (v_deposition + v_erosion) / 2.0;
        assert_eq!(transport_regime(d, v_transport), TransportRegime::Transport);
        assert_eq!(
            transport_regime(d, v_deposition * 0.5),
            TransportRegime::Deposition
        );
    }

    #[test]
    fn shields_mobile_at_high_stress() {
        // High shear stress should mobilise sand
        assert!(is_grain_mobile(5.0, 2650.0, 1000.0, 0.001, 9.81));
    }

    #[test]
    fn shields_immobile_at_low_stress() {
        // Very low shear stress should not mobilise gravel
        assert!(!is_grain_mobile(0.01, 2650.0, 1000.0, 0.01, 9.81));
    }

    // --- Groundwater tests ---

    #[test]
    fn theis_drawdown_near_well() {
        // Pumping 0.01 m³/s from confined aquifer (T=0.001, S=1e-4)
        // At 10m after 1 day (86400s)
        let s = theis_drawdown(0.01, 0.001, 1e-4, 10.0, 86400.0);
        assert!(s > 0.0, "Should have measurable drawdown near well");
    }

    #[test]
    fn theis_drawdown_decreases_with_distance() {
        let near = theis_drawdown(0.01, 0.001, 1e-4, 10.0, 86400.0);
        let far = theis_drawdown(0.01, 0.001, 1e-4, 100.0, 86400.0);
        assert!(near > far);
    }

    #[test]
    fn theis_drawdown_increases_with_time() {
        let early = theis_drawdown(0.01, 0.001, 1e-4, 50.0, 3600.0);
        let late = theis_drawdown(0.01, 0.001, 1e-4, 50.0, 86400.0);
        assert!(late > early);
    }

    #[test]
    fn cooper_jacob_agrees_with_theis_at_late_time() {
        // At late time / small u, Cooper-Jacob ≈ Theis
        let theis = theis_drawdown(0.01, 0.001, 1e-4, 10.0, 864_000.0);
        let cj = cooper_jacob_drawdown(0.01, 0.001, 1e-4, 10.0, 864_000.0);
        let rel_err = (theis - cj).abs() / theis;
        assert!(
            rel_err < 0.05,
            "Cooper-Jacob should agree within 5% at late time, got {:.1}%",
            rel_err * 100.0
        );
    }

    #[test]
    fn radius_of_influence_grows_with_time() {
        let r1 = radius_of_influence(0.001, 1e-4, 3600.0);
        let r2 = radius_of_influence(0.001, 1e-4, 86400.0);
        assert!(r2 > r1);
    }

    #[test]
    fn well_function_decreasing() {
        assert!(well_function(0.01) > well_function(0.1));
        assert!(well_function(0.1) > well_function(1.0));
    }
}
