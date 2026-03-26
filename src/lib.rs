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

pub mod error;
pub mod mineral;
pub mod crystal;
pub mod rock;
pub mod soil;
pub mod weathering;
pub mod ore;

#[cfg(feature = "thermodynamics")]
pub mod geothermal;

#[cfg(feature = "fluids")]
pub mod hydrology;

#[cfg(feature = "logging")]
pub mod logging;

// --- Core re-exports (always available) ---
pub use error::{KhanijError, Result};
pub use mineral::{Luster, Mineral, MohsHardness};
pub use crystal::CrystalSystem;
pub use rock::{GeologicalProcess, Rock, RockType, rock_cycle_next};
pub use soil::{SoilComposition, SoilTexture};
pub use ore::{DepositType, OreDeposit, is_economically_viable};
pub use weathering::{chemical_weathering_rate, erosion_rate, physical_weathering_rate};

// --- Chemistry re-exports (kimiya) ---
#[cfg(feature = "chemistry")]
pub use mineral::{dissolution_rate, ionic_radius, lattice_energy};
#[cfg(feature = "chemistry")]
pub use weathering::{arrhenius_weathering_rate, dissolution_half_life, remaining_mineral_fraction};

// --- Thermodynamics re-exports (ushma) ---
#[cfg(feature = "thermodynamics")]
pub use geothermal::{
    gibbs_energy, heat_flux, heat_stored, is_reaction_spontaneous,
    lithostatic_pressure, rock_thermal_diffusivity, temperature_at_depth,
    volatile_pressure,
};

// --- Fluids re-exports (pravash) ---
#[cfg(feature = "fluids")]
pub use hydrology::{
    buoyancy_force, darcy_flow, flow_regime, grain_reynolds_number,
    sediment_drag_force, stokes_settling_velocity, surface_water_config,
    terminal_velocity, water_particle,
};
