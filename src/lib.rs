//! # Khanij
//!
//! **Khanij** (खनिज — Sanskrit for "mineral, born from mining") — geology and
//! mineralogy engine for the AGNOS ecosystem.
//!
//! Provides mineral properties, crystal systems, Mohs hardness, rock classification,
//! soil composition, weathering/erosion, and ore deposit modeling.

pub mod error;
pub mod mineral;
pub mod crystal;
pub mod rock;
pub mod soil;
pub mod weathering;
pub mod ore;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::{KhanijError, Result};
pub use mineral::{Mineral, MohsHardness};
pub use crystal::CrystalSystem;
pub use rock::{Rock, RockType};
