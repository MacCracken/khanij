//! Sequence stratigraphy — systems tracts, sea-level cycles, accommodation
//! space, and facies relationships governed by Walther's Law.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Systems tract classification within a depositional sequence.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemsTract {
    /// Lowstand systems tract — deposited during sea-level lowstand.
    LowstandST,
    /// Transgressive systems tract — deposited during rising sea level.
    TransgressiveST,
    /// Highstand systems tract — deposited during sea-level highstand.
    HighstandST,
    /// Falling-stage systems tract — deposited during forced regression.
    FallingStageST,
}

/// Depositional environment classification.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositionalEnvironment {
    /// River-dominated continental setting.
    Fluvial,
    /// Delta front and delta plain.
    Deltaic,
    /// Wave-dominated nearshore zone above fair-weather wave base.
    Shoreface,
    /// Continental shelf between fair-weather and storm wave base.
    Shelf,
    /// Deep-water setting below storm wave base.
    DeepMarine,
    /// Lake-dominated continental setting.
    Lacustrine,
    /// Wind-dominated continental setting.
    Eolian,
    /// Glacially-dominated setting.
    Glacial,
}

// ---------------------------------------------------------------------------
// SeaLevelCycle
// ---------------------------------------------------------------------------

/// Sinusoidal model of a eustatic sea-level cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeaLevelCycle {
    /// Half the peak-to-trough range of the cycle (metres).
    pub amplitude_m: f64,
    /// Duration of one full cycle (years).
    pub period_years: f64,
}

impl SeaLevelCycle {
    /// Sea-level position at a given time (years) relative to the cycle start.
    ///
    /// Returns the elevation in metres as a sinusoidal function:
    /// `amplitude * sin(2 * PI * time / period)`.
    #[must_use]
    pub fn sea_level_at(&self, time_years: f64) -> f64 {
        self.amplitude_m * (2.0 * PI * time_years / self.period_years).sin()
    }

    /// Classify the systems tract for a given phase (0.0 – 1.0) within the
    /// cycle.
    ///
    /// | Phase range | Systems tract |
    /// |-------------|---------------|
    /// | 0.00 – 0.25 | Lowstand      |
    /// | 0.25 – 0.50 | Transgressive |
    /// | 0.50 – 0.75 | Highstand     |
    /// | 0.75 – 1.00 | Falling stage |
    #[must_use]
    pub fn classify_systems_tract(phase: f64) -> SystemsTract {
        match phase {
            p if p < 0.25 => SystemsTract::LowstandST,
            p if p < 0.50 => SystemsTract::TransgressiveST,
            p if p < 0.75 => SystemsTract::HighstandST,
            _ => SystemsTract::FallingStageST,
        }
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Total accommodation space (metres) from sea-level change and tectonic
/// subsidence.
///
/// `accommodation = sea_level_change + subsidence`
#[must_use]
pub fn accommodation_space(sea_level_change_m: f64, subsidence_m: f64) -> f64 {
    sea_level_change_m + subsidence_m
}

/// Accommodation-to-sediment-supply ratio (A/S).
///
/// - A/S > 1 → basin is underfilled (accommodation exceeds supply).
/// - A/S < 1 → basin is overfilled (supply exceeds accommodation).
/// - A/S = 1 → balanced fill.
///
/// # Panics
///
/// Panics if `sediment_supply` is zero.
#[must_use]
pub fn sediment_supply_ratio(accommodation: f64, sediment_supply: f64) -> f64 {
    assert!(sediment_supply != 0.0, "sediment_supply must be non-zero");
    accommodation / sediment_supply
}

// ---------------------------------------------------------------------------
// Walther's Law
// ---------------------------------------------------------------------------

/// Walther's Law helper: facies that occur in conformable vertical succession
/// also occur in laterally adjacent environments.
pub struct WalthersLaw;

impl WalthersLaw {
    /// Return the set of depositional environments laterally adjacent to the
    /// given environment (including the environment itself).
    #[must_use]
    pub fn lateral_equivalent(
        environment: DepositionalEnvironment,
    ) -> Vec<DepositionalEnvironment> {
        use DepositionalEnvironment::*;
        match environment {
            Fluvial => vec![Fluvial, Deltaic, Eolian, Lacustrine],
            Deltaic => vec![Fluvial, Deltaic, Shoreface, Lacustrine],
            Shoreface => vec![Deltaic, Shoreface, Shelf],
            Shelf => vec![Shoreface, Shelf, DeepMarine],
            DeepMarine => vec![Shelf, DeepMarine],
            Lacustrine => vec![Fluvial, Deltaic, Lacustrine],
            Eolian => vec![Fluvial, Eolian],
            Glacial => vec![Glacial, Fluvial, Lacustrine],
        }
    }
}

// ---------------------------------------------------------------------------
// ParasequenceBoundary
// ---------------------------------------------------------------------------

/// A parasequence boundary marking a marine flooding surface or equivalent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParasequenceBoundary {
    /// Depth of the boundary below a reference datum (metres).
    pub depth_m: f64,
    /// Age of the boundary (millions of years before present).
    pub age_ma: f64,
    /// Whether this boundary is a marine flooding surface.
    pub flooding_surface: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- SeaLevelCycle ------------------------------------------------------

    #[test]
    fn sea_level_at_zero_time_is_zero() {
        let cycle = SeaLevelCycle {
            amplitude_m: 50.0,
            period_years: 100_000.0,
        };
        let level = cycle.sea_level_at(0.0);
        assert!(level.abs() < 1e-10, "expected ~0, got {level}");
    }

    #[test]
    fn sea_level_peak_at_quarter_period() {
        let cycle = SeaLevelCycle {
            amplitude_m: 50.0,
            period_years: 100_000.0,
        };
        let level = cycle.sea_level_at(25_000.0);
        assert!((level - 50.0).abs() < 1e-10, "expected 50, got {level}");
    }

    #[test]
    fn sea_level_trough_at_three_quarter_period() {
        let cycle = SeaLevelCycle {
            amplitude_m: 50.0,
            period_years: 100_000.0,
        };
        let level = cycle.sea_level_at(75_000.0);
        assert!((level + 50.0).abs() < 1e-10, "expected -50, got {level}");
    }

    #[test]
    fn sea_level_full_period_returns_to_zero() {
        let cycle = SeaLevelCycle {
            amplitude_m: 30.0,
            period_years: 40_000.0,
        };
        let level = cycle.sea_level_at(40_000.0);
        assert!(level.abs() < 1e-10, "expected ~0, got {level}");
    }

    // -- Systems tract classification --------------------------------------

    #[test]
    fn classify_lowstand() {
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.0),
            SystemsTract::LowstandST
        );
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.1),
            SystemsTract::LowstandST
        );
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.24),
            SystemsTract::LowstandST
        );
    }

    #[test]
    fn classify_transgressive() {
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.25),
            SystemsTract::TransgressiveST
        );
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.49),
            SystemsTract::TransgressiveST
        );
    }

    #[test]
    fn classify_highstand() {
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.50),
            SystemsTract::HighstandST
        );
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.74),
            SystemsTract::HighstandST
        );
    }

    #[test]
    fn classify_falling_stage() {
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.75),
            SystemsTract::FallingStageST
        );
        assert_eq!(
            SeaLevelCycle::classify_systems_tract(0.99),
            SystemsTract::FallingStageST
        );
    }

    // -- Accommodation space -----------------------------------------------

    #[test]
    fn accommodation_positive_values() {
        let acc = accommodation_space(10.0, 5.0);
        assert!((acc - 15.0).abs() < 1e-10);
    }

    #[test]
    fn accommodation_negative_sea_level() {
        let acc = accommodation_space(-3.0, 8.0);
        assert!((acc - 5.0).abs() < 1e-10);
    }

    // -- A/S ratio ---------------------------------------------------------

    #[test]
    fn as_ratio_underfilled() {
        let ratio = sediment_supply_ratio(100.0, 50.0);
        assert!((ratio - 2.0).abs() < 1e-10, "expected 2.0, got {ratio}");
    }

    #[test]
    fn as_ratio_overfilled() {
        let ratio = sediment_supply_ratio(30.0, 60.0);
        assert!((ratio - 0.5).abs() < 1e-10, "expected 0.5, got {ratio}");
    }

    #[test]
    #[should_panic(expected = "sediment_supply must be non-zero")]
    fn as_ratio_zero_supply_panics() {
        let _ = sediment_supply_ratio(10.0, 0.0);
    }

    // -- Walther's Law -----------------------------------------------------

    #[test]
    fn walthers_law_shoreface_adjacency() {
        let adj = WalthersLaw::lateral_equivalent(DepositionalEnvironment::Shoreface);
        assert!(adj.contains(&DepositionalEnvironment::Deltaic));
        assert!(adj.contains(&DepositionalEnvironment::Shoreface));
        assert!(adj.contains(&DepositionalEnvironment::Shelf));
        assert!(!adj.contains(&DepositionalEnvironment::DeepMarine));
    }

    #[test]
    fn walthers_law_deep_marine_adjacency() {
        let adj = WalthersLaw::lateral_equivalent(DepositionalEnvironment::DeepMarine);
        assert!(adj.contains(&DepositionalEnvironment::Shelf));
        assert!(adj.contains(&DepositionalEnvironment::DeepMarine));
        assert_eq!(adj.len(), 2);
    }

    #[test]
    fn walthers_law_glacial_adjacency() {
        let adj = WalthersLaw::lateral_equivalent(DepositionalEnvironment::Glacial);
        assert!(adj.contains(&DepositionalEnvironment::Glacial));
        assert!(adj.contains(&DepositionalEnvironment::Fluvial));
        assert!(adj.contains(&DepositionalEnvironment::Lacustrine));
    }

    // -- Serialization round-trip ------------------------------------------

    #[test]
    fn parasequence_boundary_serde_roundtrip() {
        let boundary = ParasequenceBoundary {
            depth_m: 123.4,
            age_ma: 65.5,
            flooding_surface: true,
        };
        let json = serde_json::to_string(&boundary).unwrap();
        let back: ParasequenceBoundary = serde_json::from_str(&json).unwrap();
        assert!((back.depth_m - 123.4).abs() < 1e-10);
        assert!((back.age_ma - 65.5).abs() < 1e-10);
        assert!(back.flooding_surface);
    }
}
