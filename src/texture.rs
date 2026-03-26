//! Petrographic texture classification — grain size scales, sorting,
//! roundness, and fabric for describing rock textures.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wentworth/Udden grain size scale
// ---------------------------------------------------------------------------

/// Wentworth grain size class.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let gs = classify_grain_size(0.3);
/// assert_eq!(gs, GrainSize::MediumSand);
/// assert!(GrainSize::Clay < GrainSize::Boulder);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GrainSize {
    Clay,
    Silt,
    VeryFineSand,
    FineSand,
    MediumSand,
    CoarseSand,
    VeryCoarseSand,
    Granule,
    Pebble,
    Cobble,
    Boulder,
}

impl GrainSize {
    /// Size range in mm: (min, max).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let (lo, hi) = GrainSize::MediumSand.range_mm();
    /// assert!((lo - 0.25).abs() < 1e-6);
    /// assert!((hi - 0.5).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn range_mm(&self) -> (f64, f64) {
        match self {
            Self::Clay => (0.0, 0.004),
            Self::Silt => (0.004, 0.0625),
            Self::VeryFineSand => (0.0625, 0.125),
            Self::FineSand => (0.125, 0.25),
            Self::MediumSand => (0.25, 0.5),
            Self::CoarseSand => (0.5, 1.0),
            Self::VeryCoarseSand => (1.0, 2.0),
            Self::Granule => (2.0, 4.0),
            Self::Pebble => (4.0, 64.0),
            Self::Cobble => (64.0, 256.0),
            Self::Boulder => (256.0, f64::INFINITY),
        }
    }

    /// Phi scale value (φ = -log₂(d_mm)).
    /// Returns the phi value at the boundary between this and the next smaller class.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let phi = GrainSize::CoarseSand.phi();
    /// assert!((phi - 0.0).abs() < 1e-6); // 1 mm boundary => phi 0
    /// ```
    #[must_use]
    pub fn phi(&self) -> f64 {
        let (_, max) = self.range_mm();
        if max.is_infinite() {
            -8.0 // boulder
        } else {
            -(max.log2())
        }
    }
}

/// Classify grain diameter (in mm) into Wentworth size class.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_grain_size(0.001), GrainSize::Clay);
/// assert_eq!(classify_grain_size(0.3), GrainSize::MediumSand);
/// assert_eq!(classify_grain_size(500.0), GrainSize::Boulder);
/// ```
#[must_use]
pub fn classify_grain_size(diameter_mm: f64) -> GrainSize {
    if diameter_mm < 0.004 {
        GrainSize::Clay
    } else if diameter_mm < 0.0625 {
        GrainSize::Silt
    } else if diameter_mm < 0.125 {
        GrainSize::VeryFineSand
    } else if diameter_mm < 0.25 {
        GrainSize::FineSand
    } else if diameter_mm < 0.5 {
        GrainSize::MediumSand
    } else if diameter_mm < 1.0 {
        GrainSize::CoarseSand
    } else if diameter_mm < 2.0 {
        GrainSize::VeryCoarseSand
    } else if diameter_mm < 4.0 {
        GrainSize::Granule
    } else if diameter_mm < 64.0 {
        GrainSize::Pebble
    } else if diameter_mm < 256.0 {
        GrainSize::Cobble
    } else {
        GrainSize::Boulder
    }
}

/// Convert phi scale to grain diameter in mm.
///
/// d = 2^(-φ)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let d = phi_to_mm(0.0);
/// assert!((d - 1.0).abs() < 1e-10); // phi 0 => 1 mm
/// ```
#[must_use]
pub fn phi_to_mm(phi: f64) -> f64 {
    2.0_f64.powf(-phi)
}

/// Convert grain diameter in mm to phi scale.
///
/// φ = -log₂(d)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let phi = mm_to_phi(1.0);
/// assert!((phi - 0.0).abs() < 1e-10); // 1 mm => phi 0
/// ```
#[must_use]
pub fn mm_to_phi(diameter_mm: f64) -> f64 {
    -(diameter_mm.log2())
}

// ---------------------------------------------------------------------------
// Sorting
// ---------------------------------------------------------------------------

/// Sorting classification (Folk & Ward, 1957).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let s = classify_sorting(0.2);
/// assert_eq!(s, Sorting::VeryWellSorted);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Sorting {
    VeryWellSorted,
    WellSorted,
    ModeratelyWellSorted,
    ModeratelySorted,
    PoorlySorted,
    VeryPoorlySorted,
    ExtremelyPoorlySorted,
}

/// Classify sorting from standard deviation in phi units.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_sorting(0.4), Sorting::WellSorted);
/// assert_eq!(classify_sorting(1.5), Sorting::PoorlySorted);
/// ```
#[must_use]
pub fn classify_sorting(std_dev_phi: f64) -> Sorting {
    if std_dev_phi < 0.35 {
        Sorting::VeryWellSorted
    } else if std_dev_phi < 0.50 {
        Sorting::WellSorted
    } else if std_dev_phi < 0.71 {
        Sorting::ModeratelyWellSorted
    } else if std_dev_phi < 1.00 {
        Sorting::ModeratelySorted
    } else if std_dev_phi < 2.00 {
        Sorting::PoorlySorted
    } else if std_dev_phi < 4.00 {
        Sorting::VeryPoorlySorted
    } else {
        Sorting::ExtremelyPoorlySorted
    }
}

// ---------------------------------------------------------------------------
// Roundness
// ---------------------------------------------------------------------------

/// Grain roundness (Powers, 1953).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(Roundness::VeryAngular < Roundness::WellRounded);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Roundness {
    VeryAngular,
    Angular,
    SubAngular,
    SubRounded,
    Rounded,
    WellRounded,
}

// ---------------------------------------------------------------------------
// Igneous textures
// ---------------------------------------------------------------------------

/// Igneous rock texture classification.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let t = classify_igneous_texture(3.0, false, false);
/// assert_eq!(t, IgneousTexture::Phaneritic);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IgneousTexture {
    /// Coarse-grained, crystals visible to naked eye (e.g., granite).
    Phaneritic,
    /// Fine-grained, crystals too small to see (e.g., basalt).
    Aphanitic,
    /// Large crystals in a fine-grained matrix (e.g., andesite porphyry).
    Porphyritic,
    /// Glassy, no crystals (e.g., obsidian).
    Glassy,
    /// Contains gas bubble holes (e.g., pumice, scoria).
    Vesicular,
    /// Interlocking grains, from metamorphism of sediment (e.g., quartzite).
    Granoblastic,
    /// Very coarse-grained (>3cm), from pegmatite.
    Pegmatitic,
}

/// Classify igneous texture from cooling rate / grain size.
///
/// - `avg_grain_size_mm`: average crystal size in mm
/// - `has_phenocrysts`: whether large crystals exist in a finer matrix
/// - `is_glassy`: whether the rock is glassy (no crystals)
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_igneous_texture(0.0, false, true), IgneousTexture::Glassy);
/// assert_eq!(classify_igneous_texture(50.0, false, false), IgneousTexture::Pegmatitic);
/// ```
#[must_use]
pub fn classify_igneous_texture(
    avg_grain_size_mm: f64,
    has_phenocrysts: bool,
    is_glassy: bool,
) -> IgneousTexture {
    if is_glassy {
        IgneousTexture::Glassy
    } else if has_phenocrysts {
        IgneousTexture::Porphyritic
    } else if avg_grain_size_mm > 30.0 {
        IgneousTexture::Pegmatitic
    } else if avg_grain_size_mm > 1.0 {
        IgneousTexture::Phaneritic
    } else {
        IgneousTexture::Aphanitic
    }
}

// ---------------------------------------------------------------------------
// Metamorphic fabric
// ---------------------------------------------------------------------------

/// Metamorphic fabric type.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let fabric = MetamorphicFabric::Foliated;
/// assert_eq!(fabric, MetamorphicFabric::Foliated);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MetamorphicFabric {
    /// No preferred orientation.
    Massive,
    /// Planar alignment of platy minerals (slate, phyllite, schist).
    Foliated,
    /// Linear alignment of elongate minerals.
    Lineated,
    /// Alternating light and dark mineral bands (gneiss).
    Banded,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wentworth_classification() {
        assert_eq!(classify_grain_size(0.001), GrainSize::Clay);
        assert_eq!(classify_grain_size(0.03), GrainSize::Silt);
        assert_eq!(classify_grain_size(0.3), GrainSize::MediumSand);
        assert_eq!(classify_grain_size(5.0), GrainSize::Pebble);
        assert_eq!(classify_grain_size(100.0), GrainSize::Cobble);
        assert_eq!(classify_grain_size(500.0), GrainSize::Boulder);
    }

    #[test]
    fn phi_mm_roundtrip() {
        for d in [0.001, 0.0625, 0.25, 1.0, 4.0, 64.0] {
            let phi = mm_to_phi(d);
            let recovered = phi_to_mm(phi);
            assert!((recovered - d).abs() < d * 0.001);
        }
    }

    #[test]
    fn phi_scale_values() {
        // 1mm = 0 phi, 0.5mm = 1 phi, 2mm = -1 phi
        assert!((mm_to_phi(1.0) - 0.0).abs() < 0.001);
        assert!((mm_to_phi(0.5) - 1.0).abs() < 0.001);
        assert!((mm_to_phi(2.0) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn grain_size_ordering() {
        assert!(GrainSize::Clay < GrainSize::Silt);
        assert!(GrainSize::Silt < GrainSize::MediumSand);
        assert!(GrainSize::MediumSand < GrainSize::Pebble);
        assert!(GrainSize::Pebble < GrainSize::Boulder);
    }

    #[test]
    fn sorting_classification() {
        assert_eq!(classify_sorting(0.2), Sorting::VeryWellSorted);
        assert_eq!(classify_sorting(0.4), Sorting::WellSorted);
        assert_eq!(classify_sorting(0.8), Sorting::ModeratelySorted);
        assert_eq!(classify_sorting(1.5), Sorting::PoorlySorted);
        assert_eq!(classify_sorting(3.0), Sorting::VeryPoorlySorted);
        assert_eq!(classify_sorting(5.0), Sorting::ExtremelyPoorlySorted);
    }

    #[test]
    fn roundness_ordering() {
        assert!(Roundness::VeryAngular < Roundness::WellRounded);
        assert!(Roundness::SubAngular < Roundness::Rounded);
    }

    #[test]
    fn igneous_texture_granite() {
        assert_eq!(
            classify_igneous_texture(3.0, false, false),
            IgneousTexture::Phaneritic
        );
    }

    #[test]
    fn igneous_texture_basalt() {
        assert_eq!(
            classify_igneous_texture(0.1, false, false),
            IgneousTexture::Aphanitic
        );
    }

    #[test]
    fn igneous_texture_obsidian() {
        assert_eq!(
            classify_igneous_texture(0.0, false, true),
            IgneousTexture::Glassy
        );
    }

    #[test]
    fn igneous_texture_porphyry() {
        assert_eq!(
            classify_igneous_texture(0.5, true, false),
            IgneousTexture::Porphyritic
        );
    }

    #[test]
    fn igneous_texture_pegmatite() {
        assert_eq!(
            classify_igneous_texture(50.0, false, false),
            IgneousTexture::Pegmatitic
        );
    }
}
