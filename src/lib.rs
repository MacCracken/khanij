//! # Khanij
//!
//! **Khanij** (खनिज — Sanskrit for "mineral, born from mining") — geology and
//! mineralogy engine for the AGNOS ecosystem.
//!
//! Provides mineral properties, crystal systems, Mohs hardness, rock classification,
//! soil composition, weathering/erosion, and ore deposit modeling.
//!
//! ## Optional features
//!
//! - **`chemistry`** — mineral composition, dissolution kinetics, and lattice energy
//!   via [kimiya](https://crates.io/crates/kimiya).
//! - **`thermodynamics`** — geothermal gradients, heat flow, and metamorphic phase
//!   equilibria via [ushma](https://crates.io/crates/ushma).
//! - **`fluids`** — groundwater flow, sediment transport, and surface hydrology
//!   via [pravash](https://crates.io/crates/pravash).
//! - **`weather`** — climate-driven weathering and erosion from atmospheric state
//!   via [badal](https://crates.io/crates/badal).

/// Cross-crate bridges — primitive-value conversions from other AGNOS science crates.
pub mod bridge;
pub mod crystal;
pub mod crystallography;
pub mod dating;
pub mod error;
pub mod formula;
pub mod geochemistry;
pub mod glaciology;
pub mod grid;
pub mod hydrothermal;
/// Integration APIs for downstream consumers (soorat rendering).
pub mod integration;
pub mod mineral;
pub mod ore;
pub mod rock;
pub mod sediment;
pub mod soil;
pub mod stratigraphy;
pub mod tectonics;
pub mod texture;
pub mod timescale;
pub mod volcanic;
pub mod weathering;

#[cfg(feature = "chemistry")]
pub mod stability;

#[cfg(feature = "thermodynamics")]
pub mod geothermal;

#[cfg(feature = "fluids")]
pub mod hydrology;

#[cfg(feature = "mechanics")]
pub mod rock_mechanics;

#[cfg(feature = "logging")]
pub mod logging;

// --- Core re-exports (always available) ---
pub use crystal::CrystalSystem;
pub use crystallography::{MillerIndex, UnitCell, bragg_angle, bragg_wavelength, d_spacing};
pub use dating::{
    IsochronPoint, IsotopeSystem, age_from_ratio, c14_age, c14_fraction_remaining,
    closure_temperature, decay_constant, half_life, isochron_age, parent_remaining,
};
pub use error::{KhanijError, Result};
pub use formula::Formula;
pub use geochemistry::{
    AsiClassification, MajorOxides, TasClassification, alumina_saturation_index, classify_asi,
    classify_tas, fractional_crystallization, mg_number,
};
pub use glaciology::{
    GlacierType, basal_sliding_velocity, equilibrium_line_altitude, glen_flow_law,
    ice_velocity_depth_integrated, isostatic_depression, isostatic_rebound_time, mass_balance,
};
pub use grid::{GeologicGrid, GeologicLayer, GeologicUnit, StratigraphicColumn, StrikeDip};
pub use hydrothermal::{
    AlterationZone, HydrothermalConditions, classify_alteration, estimated_ore_grade,
    metal_solubility, precipitation_rate,
};
pub use mineral::{Luster, Mineral, MohsHardness};
pub use ore::{
    DepositType, OreDeposit, ResourceCategory, TonnageGradePoint, cutoff_grade,
    is_economically_viable, net_present_value, tonnage_grade_curve,
};
pub use rock::{
    GeologicalProcess, Rock, RockType, bulk_density, bulk_density_from_minerals,
    porosity_from_density, rock_cycle_next,
};
pub use sediment::{
    BudgetResult, GRAIN_CLASSES, GRAIN_DIAMETERS, SedimentSink, SedimentSource, compute_budget,
    denudation_rate, sediment_delivery_ratio, sediment_production, transport_capacity,
};
pub use soil::{
    HorizonType, SoilComposition, SoilFertility, SoilHorizon, SoilOrder, SoilPhClass, SoilProfile,
    SoilTexture, available_water_capacity, cation_exchange_capacity, classify_ph,
    hydraulic_conductivity_mm_hr,
};
pub use stratigraphy::{
    DepositionalEnvironment, ParasequenceBoundary, SeaLevelCycle, SystemsTract, WalthersLaw,
    accommodation_space, sediment_supply_ratio,
};
pub use tectonics::{
    BoundaryType, EulerPole, RidgeType, SubductionZone, classify_ridge, full_spreading_rate,
    lithosphere_thickness, ocean_depth_m, ocean_floor_age,
};
pub use texture::{
    GrainSize, IgneousTexture, MetamorphicFabric, Roundness, Sorting, classify_grain_size,
    classify_igneous_texture, classify_sorting, mm_to_phi, phi_to_mm,
};
pub use timescale::{
    Eon, Epoch, Era, Period, StratigraphicPosition, TimeInterval, classify_age, eon_at_age,
    epoch_at_age, era_at_age, period_at_age,
};
pub use volcanic::{
    MagmaComposition, MagmaType, Vei, classify_magma, classify_vei, eruption_column_height,
    lava_flow_velocity, magma_viscosity, pyroclastic_flow_runout,
};
pub use weathering::{chemical_weathering_rate, erosion_rate, physical_weathering_rate};

// --- Chemistry re-exports (kimiya) ---
#[cfg(feature = "chemistry")]
pub use mineral::{dissolution_rate, ionic_radius, lattice_energy};
#[cfg(feature = "chemistry")]
pub use stability::{
    equilibrium_temperature, gibbs_at_temperature, gibbs_formation, is_reaction_spontaneous,
    reaction_enthalpy, reaction_entropy, reaction_gibbs, stable_polymorph,
};
#[cfg(feature = "chemistry")]
pub use weathering::{
    WEATHERING_REACTIONS, WeatheringReaction, WeatheringType, arrhenius_weathering_rate,
    dissolution_half_life, mineral_weathering_rate, remaining_mineral_fraction,
    weathering_reaction,
};

// --- Thermodynamics re-exports (ushma) ---
#[cfg(feature = "thermodynamics")]
pub use geothermal::{
    MetamorphicFacies, classify_facies, contact_aureole_temperature, facies_at_depth, gibbs_energy,
    heat_flux, heat_stored, intrusion_cooling, intrusion_cooling_time, is_spontaneous,
    lithostatic_pressure, rock_thermal_diffusivity, temperature_at_depth, volatile_pressure,
};

// --- Fluids re-exports (pravash) ---
#[cfg(feature = "fluids")]
pub use hydrology::{
    SHIELDS_CRITICAL, TransportRegime, buoyancy_force, cooper_jacob_drawdown, darcy_flow,
    flow_regime, grain_reynolds_number, hjulstrom_deposition_velocity, hjulstrom_erosion_velocity,
    is_grain_mobile, radius_of_influence, sediment_drag_force, shields_parameter,
    stokes_settling_velocity, surface_water_config, terminal_velocity, theis_drawdown,
    transport_regime, water_particle, well_function,
};

// --- Mechanics re-exports (dravya) ---
#[cfg(feature = "mechanics")]
pub use rock_mechanics::{
    FailureMode, basalt_material, brittle_ductile_transition_depth, classify_failure_mode,
    gneiss_material, granite_material, infinite_slope_safety_factor, limestone_material,
    marble_material, mohr_coulomb_failure, mohr_coulomb_safety_factor, mohr_coulomb_strength,
    mohr_coulomb_to_drucker_prager, p_wave_at_temperature, p_wave_velocity,
    poisson_from_velocities, quartzite_material, s_wave_at_temperature, s_wave_velocity,
    sandstone_material, shale_material, time_to_weathering_failure, velocity_depth_profile,
    vp_vs_ratio, weathered_material,
};

// --- Weather re-exports (badal) ---
#[cfg(feature = "weather")]
pub use weathering::{
    chemical_weathering_from_climate, erosion_from_climate, freeze_thaw_cycles,
    physical_weathering_from_climate, weathering_intensity,
};
