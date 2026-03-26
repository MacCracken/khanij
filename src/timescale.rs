//! Geologic timescale — eons, eras, periods, and epochs with absolute age
//! ranges in millions of years ago (Ma).

use serde::{Deserialize, Serialize};

/// A geologic time interval with start and end ages in Ma (millions of years ago).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let interval = Period::Jurassic.interval();
/// assert_eq!(interval.name, "Jurassic");
/// assert!(interval.start_ma > interval.end_ma);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeInterval {
    pub name: &'static str,
    pub start_ma: f64, // older boundary
    pub end_ma: f64,   // younger boundary (0.0 = present)
}

impl TimeInterval {
    /// Duration in millions of years.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let dur = Period::Jurassic.interval().duration_ma();
    /// assert!((dur - 56.4).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn duration_ma(&self) -> f64 {
        self.start_ma - self.end_ma
    }

    /// Check if an age (in Ma) falls within this interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let jurassic = Period::Jurassic.interval();
    /// assert!(jurassic.contains_age(150.0));
    /// assert!(!jurassic.contains_age(50.0));
    /// ```
    #[must_use]
    pub fn contains_age(&self, age_ma: f64) -> bool {
        age_ma >= self.end_ma && age_ma < self.start_ma
    }
}

// ---------------------------------------------------------------------------
// Eons
// ---------------------------------------------------------------------------

/// Geologic eon — the largest division of geologic time.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let interval = Eon::Phanerozoic.interval();
/// assert!(interval.contains_age(100.0));
/// assert_eq!(Eon::ALL.len(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Eon {
    Hadean,
    Archean,
    Proterozoic,
    Phanerozoic,
}

impl Eon {
    /// Returns the time interval for this eon.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let archean = Eon::Archean.interval();
    /// assert!((archean.start_ma - 4000.0).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn interval(&self) -> TimeInterval {
        match self {
            Self::Hadean => TimeInterval {
                name: "Hadean",
                start_ma: 4600.0,
                end_ma: 4000.0,
            },
            Self::Archean => TimeInterval {
                name: "Archean",
                start_ma: 4000.0,
                end_ma: 2500.0,
            },
            Self::Proterozoic => TimeInterval {
                name: "Proterozoic",
                start_ma: 2500.0,
                end_ma: 538.8,
            },
            Self::Phanerozoic => TimeInterval {
                name: "Phanerozoic",
                start_ma: 538.8,
                end_ma: 0.0,
            },
        }
    }

    /// All eons in chronological order (oldest first).
    pub const ALL: &'static [Eon] = &[
        Self::Hadean,
        Self::Archean,
        Self::Proterozoic,
        Self::Phanerozoic,
    ];
}

// ---------------------------------------------------------------------------
// Eras
// ---------------------------------------------------------------------------

/// Geologic era — subdivision of an eon.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mesozoic = Era::Mesozoic.interval();
/// assert!(mesozoic.contains_age(150.0));
/// assert_eq!(Era::ALL.len(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Era {
    // Phanerozoic eras
    Paleozoic,
    Mesozoic,
    Cenozoic,
}

impl Era {
    /// Returns the time interval for this era.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cenozoic = Era::Cenozoic.interval();
    /// assert!((cenozoic.start_ma - 66.0).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn interval(&self) -> TimeInterval {
        match self {
            Self::Paleozoic => TimeInterval {
                name: "Paleozoic",
                start_ma: 538.8,
                end_ma: 251.9,
            },
            Self::Mesozoic => TimeInterval {
                name: "Mesozoic",
                start_ma: 251.9,
                end_ma: 66.0,
            },
            Self::Cenozoic => TimeInterval {
                name: "Cenozoic",
                start_ma: 66.0,
                end_ma: 0.0,
            },
        }
    }

    /// Parent eon.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert_eq!(Era::Mesozoic.eon(), Eon::Phanerozoic);
    /// ```
    #[must_use]
    pub fn eon(&self) -> Eon {
        Eon::Phanerozoic
    }

    /// All eras in chronological order.
    pub const ALL: &'static [Era] = &[Self::Paleozoic, Self::Mesozoic, Self::Cenozoic];
}

// ---------------------------------------------------------------------------
// Periods
// ---------------------------------------------------------------------------

/// Geologic period — subdivision of an era.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let j = Period::Jurassic.interval();
/// assert!(j.contains_age(160.0));
/// assert_eq!(Period::Jurassic.era(), Era::Mesozoic);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Period {
    // Paleozoic
    Cambrian,
    Ordovician,
    Silurian,
    Devonian,
    Carboniferous,
    Permian,
    // Mesozoic
    Triassic,
    Jurassic,
    Cretaceous,
    // Cenozoic
    Paleogene,
    Neogene,
    Quaternary,
}

impl Period {
    /// Returns the time interval for this period.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cretaceous = Period::Cretaceous.interval();
    /// assert!((cretaceous.start_ma - 145.0).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn interval(&self) -> TimeInterval {
        match self {
            Self::Cambrian => TimeInterval {
                name: "Cambrian",
                start_ma: 538.8,
                end_ma: 485.4,
            },
            Self::Ordovician => TimeInterval {
                name: "Ordovician",
                start_ma: 485.4,
                end_ma: 443.8,
            },
            Self::Silurian => TimeInterval {
                name: "Silurian",
                start_ma: 443.8,
                end_ma: 419.2,
            },
            Self::Devonian => TimeInterval {
                name: "Devonian",
                start_ma: 419.2,
                end_ma: 358.9,
            },
            Self::Carboniferous => TimeInterval {
                name: "Carboniferous",
                start_ma: 358.9,
                end_ma: 298.9,
            },
            Self::Permian => TimeInterval {
                name: "Permian",
                start_ma: 298.9,
                end_ma: 251.9,
            },
            Self::Triassic => TimeInterval {
                name: "Triassic",
                start_ma: 251.9,
                end_ma: 201.4,
            },
            Self::Jurassic => TimeInterval {
                name: "Jurassic",
                start_ma: 201.4,
                end_ma: 145.0,
            },
            Self::Cretaceous => TimeInterval {
                name: "Cretaceous",
                start_ma: 145.0,
                end_ma: 66.0,
            },
            Self::Paleogene => TimeInterval {
                name: "Paleogene",
                start_ma: 66.0,
                end_ma: 23.03,
            },
            Self::Neogene => TimeInterval {
                name: "Neogene",
                start_ma: 23.03,
                end_ma: 2.58,
            },
            Self::Quaternary => TimeInterval {
                name: "Quaternary",
                start_ma: 2.58,
                end_ma: 0.0,
            },
        }
    }

    /// Parent era.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert_eq!(Period::Cambrian.era(), Era::Paleozoic);
    /// assert_eq!(Period::Jurassic.era(), Era::Mesozoic);
    /// ```
    #[must_use]
    pub fn era(&self) -> Era {
        match self {
            Self::Cambrian
            | Self::Ordovician
            | Self::Silurian
            | Self::Devonian
            | Self::Carboniferous
            | Self::Permian => Era::Paleozoic,
            Self::Triassic | Self::Jurassic | Self::Cretaceous => Era::Mesozoic,
            Self::Paleogene | Self::Neogene | Self::Quaternary => Era::Cenozoic,
        }
    }

    /// All periods in chronological order.
    pub const ALL: &'static [Period] = &[
        Self::Cambrian,
        Self::Ordovician,
        Self::Silurian,
        Self::Devonian,
        Self::Carboniferous,
        Self::Permian,
        Self::Triassic,
        Self::Jurassic,
        Self::Cretaceous,
        Self::Paleogene,
        Self::Neogene,
        Self::Quaternary,
    ];
}

// ---------------------------------------------------------------------------
// Epochs (Cenozoic detail)
// ---------------------------------------------------------------------------

/// Geologic epoch — subdivision of a period (Cenozoic detail).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let eocene = Epoch::Eocene.interval();
/// assert!(eocene.contains_age(45.0));
/// assert_eq!(Epoch::Eocene.period(), Period::Paleogene);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Epoch {
    // Paleogene
    Paleocene,
    Eocene,
    Oligocene,
    // Neogene
    Miocene,
    Pliocene,
    // Quaternary
    Pleistocene,
    Holocene,
}

impl Epoch {
    /// Returns the time interval for this epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let holocene = Epoch::Holocene.interval();
    /// assert!((holocene.end_ma).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn interval(&self) -> TimeInterval {
        match self {
            Self::Paleocene => TimeInterval {
                name: "Paleocene",
                start_ma: 66.0,
                end_ma: 56.0,
            },
            Self::Eocene => TimeInterval {
                name: "Eocene",
                start_ma: 56.0,
                end_ma: 33.9,
            },
            Self::Oligocene => TimeInterval {
                name: "Oligocene",
                start_ma: 33.9,
                end_ma: 23.03,
            },
            Self::Miocene => TimeInterval {
                name: "Miocene",
                start_ma: 23.03,
                end_ma: 5.33,
            },
            Self::Pliocene => TimeInterval {
                name: "Pliocene",
                start_ma: 5.33,
                end_ma: 2.58,
            },
            Self::Pleistocene => TimeInterval {
                name: "Pleistocene",
                start_ma: 2.58,
                end_ma: 0.0117,
            },
            Self::Holocene => TimeInterval {
                name: "Holocene",
                start_ma: 0.0117,
                end_ma: 0.0,
            },
        }
    }

    /// Parent period.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert_eq!(Epoch::Miocene.period(), Period::Neogene);
    /// assert_eq!(Epoch::Holocene.period(), Period::Quaternary);
    /// ```
    #[must_use]
    pub fn period(&self) -> Period {
        match self {
            Self::Paleocene | Self::Eocene | Self::Oligocene => Period::Paleogene,
            Self::Miocene | Self::Pliocene => Period::Neogene,
            Self::Pleistocene | Self::Holocene => Period::Quaternary,
        }
    }

    /// All epochs in chronological order.
    pub const ALL: &'static [Epoch] = &[
        Self::Paleocene,
        Self::Eocene,
        Self::Oligocene,
        Self::Miocene,
        Self::Pliocene,
        Self::Pleistocene,
        Self::Holocene,
    ];
}

// ---------------------------------------------------------------------------
// Lookup functions
// ---------------------------------------------------------------------------

/// Look up the geologic period for a given age in Ma.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(period_at_age(70.0), Some(Period::Cretaceous));
/// assert_eq!(period_at_age(3000.0), None);
/// ```
#[must_use]
pub fn period_at_age(age_ma: f64) -> Option<Period> {
    Period::ALL
        .iter()
        .find(|p| p.interval().contains_age(age_ma))
        .copied()
}

/// Look up the geologic era for a given age in Ma.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(era_at_age(100.0), Some(Era::Mesozoic));
/// assert_eq!(era_at_age(3000.0), None);
/// ```
#[must_use]
pub fn era_at_age(age_ma: f64) -> Option<Era> {
    Era::ALL
        .iter()
        .find(|e| e.interval().contains_age(age_ma))
        .copied()
}

/// Look up the geologic eon for a given age in Ma.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(eon_at_age(3000.0), Some(Eon::Archean));
/// assert_eq!(eon_at_age(5000.0), None);
/// ```
#[must_use]
pub fn eon_at_age(age_ma: f64) -> Option<Eon> {
    Eon::ALL
        .iter()
        .find(|e| e.interval().contains_age(age_ma))
        .copied()
}

/// Look up the Cenozoic epoch for a given age in Ma.
/// Returns `None` for ages outside the Cenozoic.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(epoch_at_age(45.0), Some(Epoch::Eocene));
/// assert_eq!(epoch_at_age(100.0), None); // Cretaceous
/// ```
#[must_use]
pub fn epoch_at_age(age_ma: f64) -> Option<Epoch> {
    Epoch::ALL
        .iter()
        .find(|e| e.interval().contains_age(age_ma))
        .copied()
}

/// Full stratigraphic classification for a given age.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let pos = classify_age(150.0);
/// assert_eq!(pos.period, Some(Period::Jurassic));
/// assert_eq!(pos.epoch, None); // pre-Cenozoic
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StratigraphicPosition {
    pub age_ma: f64,
    pub eon: Option<Eon>,
    pub era: Option<Era>,
    pub period: Option<Period>,
    pub epoch: Option<Epoch>,
}

/// Classify an age into its full stratigraphic position.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let pos = classify_age(10.0);
/// assert_eq!(pos.eon, Some(Eon::Phanerozoic));
/// assert_eq!(pos.era, Some(Era::Cenozoic));
/// assert_eq!(pos.epoch, Some(Epoch::Miocene));
/// ```
#[must_use]
pub fn classify_age(age_ma: f64) -> StratigraphicPosition {
    StratigraphicPosition {
        age_ma,
        eon: eon_at_age(age_ma),
        era: era_at_age(age_ma),
        period: period_at_age(age_ma),
        epoch: epoch_at_age(age_ma),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_day_is_quaternary() {
        assert_eq!(period_at_age(0.001), Some(Period::Quaternary));
    }

    #[test]
    fn present_is_holocene() {
        assert_eq!(epoch_at_age(0.005), Some(Epoch::Holocene));
    }

    #[test]
    fn dinosaur_age_is_cretaceous() {
        assert_eq!(period_at_age(70.0), Some(Period::Cretaceous));
    }

    #[test]
    fn jurassic_boundaries() {
        let j = Period::Jurassic.interval();
        assert!(j.contains_age(150.0));
        assert!(!j.contains_age(210.0)); // Triassic
        assert!(!j.contains_age(140.0)); // Cretaceous
    }

    #[test]
    fn cambrian_explosion() {
        assert_eq!(period_at_age(520.0), Some(Period::Cambrian));
        assert_eq!(era_at_age(520.0), Some(Era::Paleozoic));
        assert_eq!(eon_at_age(520.0), Some(Eon::Phanerozoic));
    }

    #[test]
    fn precambrian() {
        assert_eq!(eon_at_age(3000.0), Some(Eon::Archean));
        assert_eq!(era_at_age(3000.0), None); // eras only defined for Phanerozoic
        assert_eq!(period_at_age(3000.0), None);
    }

    #[test]
    fn hadean() {
        assert_eq!(eon_at_age(4500.0), Some(Eon::Hadean));
    }

    #[test]
    fn proterozoic() {
        assert_eq!(eon_at_age(1000.0), Some(Eon::Proterozoic));
    }

    #[test]
    fn beyond_earth_returns_none() {
        assert_eq!(eon_at_age(5000.0), None);
    }

    #[test]
    fn epoch_outside_cenozoic_is_none() {
        assert_eq!(epoch_at_age(100.0), None); // Cretaceous — no epoch detail
    }

    #[test]
    fn eocene_epoch() {
        assert_eq!(epoch_at_age(45.0), Some(Epoch::Eocene));
        assert_eq!(epoch_at_age(45.0).unwrap().period(), Period::Paleogene);
    }

    #[test]
    fn pleistocene_ice_age() {
        assert_eq!(epoch_at_age(1.0), Some(Epoch::Pleistocene));
    }

    #[test]
    fn period_era_relationship() {
        assert_eq!(Period::Jurassic.era(), Era::Mesozoic);
        assert_eq!(Period::Cambrian.era(), Era::Paleozoic);
        assert_eq!(Period::Quaternary.era(), Era::Cenozoic);
    }

    #[test]
    fn all_periods_cover_phanerozoic() {
        // Ensure periods span continuously from 538.8 to 0.0
        let first = Period::ALL.first().unwrap().interval();
        let last = Period::ALL.last().unwrap().interval();
        assert!((first.start_ma - 538.8).abs() < 0.1);
        assert!(last.end_ma.abs() < 0.01);
    }

    #[test]
    fn periods_are_contiguous() {
        for pair in Period::ALL.windows(2) {
            let older = pair[0].interval();
            let younger = pair[1].interval();
            assert!(
                (older.end_ma - younger.start_ma).abs() < 0.01,
                "{} end ({}) should equal {} start ({})",
                older.name,
                older.end_ma,
                younger.name,
                younger.start_ma
            );
        }
    }

    #[test]
    fn duration_positive() {
        for p in Period::ALL {
            assert!(
                p.interval().duration_ma() > 0.0,
                "{} should have positive duration",
                p.interval().name
            );
        }
    }

    #[test]
    fn classify_age_full() {
        let pos = classify_age(150.0);
        assert_eq!(pos.eon, Some(Eon::Phanerozoic));
        assert_eq!(pos.era, Some(Era::Mesozoic));
        assert_eq!(pos.period, Some(Period::Jurassic));
        assert_eq!(pos.epoch, None); // pre-Cenozoic
    }

    #[test]
    fn classify_age_cenozoic() {
        let pos = classify_age(10.0);
        assert_eq!(pos.eon, Some(Eon::Phanerozoic));
        assert_eq!(pos.era, Some(Era::Cenozoic));
        assert_eq!(pos.period, Some(Period::Neogene));
        assert_eq!(pos.epoch, Some(Epoch::Miocene));
    }

    #[test]
    fn twelve_periods() {
        assert_eq!(Period::ALL.len(), 12);
    }

    #[test]
    fn four_eons() {
        assert_eq!(Eon::ALL.len(), 4);
    }

    #[test]
    fn seven_epochs() {
        assert_eq!(Epoch::ALL.len(), 7);
    }
}
