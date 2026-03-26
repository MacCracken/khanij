//! Geochemistry module — major oxide analysis, TAS classification,
//! magmatic differentiation indices, and Rayleigh fractionation.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Molecular weights for molar ratio conversions
// ---------------------------------------------------------------------------
const MW_AL2O3: f64 = 101.961;
const MW_CAO: f64 = 56.077;
const MW_NA2O: f64 = 61.979;
const MW_K2O: f64 = 94.196;
const MW_MGO: f64 = 40.304;
const MW_FEO: f64 = 71.844;

// ---------------------------------------------------------------------------
// MajorOxides
// ---------------------------------------------------------------------------

/// Major-element oxide analysis of a rock sample (weight-percent).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let ox = MajorOxides {
///     sio2: 49.5, tio2: 1.5, al2o3: 15.5, fe2o3: 2.5,
///     feo: 7.5, mno: 0.17, mgo: 8.0, cao: 11.0,
///     na2o: 2.5, k2o: 0.5, p2o5: 0.2, h2o: 0.5,
/// };
/// assert!(ox.is_valid());
/// assert_eq!(ox.tas_classification(), TasClassification::Basalt);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorOxides {
    pub sio2: f64,
    pub tio2: f64,
    pub al2o3: f64,
    pub fe2o3: f64,
    pub feo: f64,
    pub mno: f64,
    pub mgo: f64,
    pub cao: f64,
    pub na2o: f64,
    pub k2o: f64,
    pub p2o5: f64,
    pub h2o: f64,
}

impl MajorOxides {
    /// Sum of all oxide weight-percent values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 49.5, tio2: 1.5, al2o3: 15.5, fe2o3: 2.5,
    ///     feo: 7.5, mno: 0.17, mgo: 8.0, cao: 11.0,
    ///     na2o: 2.5, k2o: 0.5, p2o5: 0.2, h2o: 0.5,
    /// };
    /// assert!((ox.total() - 99.37).abs() < 0.01);
    /// ```
    #[must_use]
    pub fn total(&self) -> f64 {
        self.sio2
            + self.tio2
            + self.al2o3
            + self.fe2o3
            + self.feo
            + self.mno
            + self.mgo
            + self.cao
            + self.na2o
            + self.k2o
            + self.p2o5
            + self.h2o
    }

    /// Returns `true` when the oxide total is within 100 +/- 2 wt%.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 49.5, tio2: 1.5, al2o3: 15.5, fe2o3: 2.5,
    ///     feo: 7.5, mno: 0.17, mgo: 8.0, cao: 11.0,
    ///     na2o: 2.5, k2o: 0.5, p2o5: 0.2, h2o: 0.5,
    /// };
    /// assert!(ox.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let t = self.total();
        (98.0..=102.0).contains(&t)
    }

    /// Total alkali content (Na2O + K2O) for TAS classification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 49.5, tio2: 1.5, al2o3: 15.5, fe2o3: 2.5,
    ///     feo: 7.5, mno: 0.17, mgo: 8.0, cao: 11.0,
    ///     na2o: 2.5, k2o: 0.5, p2o5: 0.2, h2o: 0.5,
    /// };
    /// assert!((ox.total_alkali() - 3.0).abs() < 1e-10);
    /// ```
    #[must_use]
    pub fn total_alkali(&self) -> f64 {
        self.na2o + self.k2o
    }

    /// Classify this analysis on the Total Alkali-Silica diagram.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 73.0, tio2: 0.2, al2o3: 13.5, fe2o3: 0.8,
    ///     feo: 1.0, mno: 0.05, mgo: 0.3, cao: 1.2,
    ///     na2o: 3.5, k2o: 5.0, p2o5: 0.05, h2o: 0.5,
    /// };
    /// assert_eq!(ox.tas_classification(), TasClassification::Rhyolite);
    /// ```
    #[must_use]
    pub fn tas_classification(&self) -> TasClassification {
        classify_tas(self.sio2, self.total_alkali())
    }

    /// Mg-number computed from FeO and MgO of this analysis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 49.5, tio2: 1.5, al2o3: 15.5, fe2o3: 2.5,
    ///     feo: 7.5, mno: 0.17, mgo: 8.0, cao: 11.0,
    ///     na2o: 2.5, k2o: 0.5, p2o5: 0.2, h2o: 0.5,
    /// };
    /// let mg = ox.mg_number();
    /// assert!(mg > 0.5 && mg < 0.8);
    /// ```
    #[must_use]
    pub fn mg_number(&self) -> f64 {
        mg_number(self.feo, self.mgo)
    }

    /// Alumina saturation index for this analysis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let ox = MajorOxides {
    ///     sio2: 73.0, tio2: 0.2, al2o3: 13.5, fe2o3: 0.8,
    ///     feo: 1.0, mno: 0.05, mgo: 0.3, cao: 1.2,
    ///     na2o: 3.5, k2o: 5.0, p2o5: 0.05, h2o: 0.5,
    /// };
    /// let asi = ox.asi();
    /// assert!(asi > 0.0);
    /// ```
    #[must_use]
    pub fn asi(&self) -> f64 {
        alumina_saturation_index(self.al2o3, self.cao, self.na2o, self.k2o)
    }
}

// ---------------------------------------------------------------------------
// TAS Classification
// ---------------------------------------------------------------------------

/// Rock classification on the Total Alkali-Silica (TAS) diagram
/// (Le Bas et al., 1986).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cls = classify_tas(50.0, 3.0);
/// assert_eq!(cls, TasClassification::Basalt);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TasClassification {
    Picrite,
    Picrobasalt,
    Basalt,
    BasalticAndesite,
    Andesite,
    Dacite,
    Rhyolite,
    Trachybasalt,
    BasalticTrachyandesite,
    Trachyandesite,
    Trachyte,
    Phonolite,
    Tephrite,
    Phonotephrite,
    Tephriphonolite,
    Foidite,
}

/// Classify a volcanic rock on the Total Alkali-Silica diagram using
/// simplified field boundaries after Le Bas et al. (1986).
///
/// * `sio2` -- SiO2 in weight-percent
/// * `total_alkali` -- Na2O + K2O in weight-percent
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_tas(50.0, 3.0), TasClassification::Basalt);
/// assert_eq!(classify_tas(73.0, 8.5), TasClassification::Rhyolite);
/// assert_eq!(classify_tas(58.0, 4.0), TasClassification::Andesite);
/// ```
#[must_use]
pub fn classify_tas(sio2: f64, total_alkali: f64) -> TasClassification {
    // Ultra-alkaline / foidite field
    if total_alkali > sio2 - 25.0 && sio2 < 41.0 {
        return TasClassification::Foidite;
    }

    // Highly alkaline, low-silica
    if sio2 < 41.0 {
        if total_alkali >= 3.0 {
            return TasClassification::Foidite;
        }
        return TasClassification::Picrite;
    }

    // Main classification grid — approximate polygon boundaries.
    // The alkaline/sub-alkaline divide runs roughly along
    //   total_alkali = 0.37 * sio2 - 14.43  (Irvine & Baragar, 1971)
    let alkaline_boundary = 0.37 * sio2 - 14.43;
    let is_alkaline = total_alkali > alkaline_boundary;

    match sio2 {
        s if s < 45.0 => {
            if is_alkaline {
                if total_alkali >= 9.0 {
                    TasClassification::Phonotephrite
                } else {
                    TasClassification::Tephrite
                }
            } else if total_alkali < 2.0 {
                TasClassification::Picrite
            } else {
                TasClassification::Picrobasalt
            }
        }
        s if s < 52.0 => {
            if is_alkaline {
                if total_alkali >= 9.5 {
                    TasClassification::Tephriphonolite
                } else if total_alkali >= 5.0 {
                    TasClassification::Trachybasalt
                } else {
                    TasClassification::Basalt
                }
            } else {
                TasClassification::Basalt
            }
        }
        s if s < 57.0 => {
            if is_alkaline {
                if total_alkali >= 11.0 {
                    TasClassification::Phonolite
                } else if total_alkali >= 7.0 {
                    TasClassification::BasalticTrachyandesite
                } else {
                    TasClassification::BasalticAndesite
                }
            } else {
                TasClassification::BasalticAndesite
            }
        }
        s if s < 63.0 => {
            if is_alkaline {
                if total_alkali >= 11.5 {
                    TasClassification::Phonolite
                } else if total_alkali >= 7.5 {
                    TasClassification::Trachyandesite
                } else {
                    TasClassification::Andesite
                }
            } else {
                TasClassification::Andesite
            }
        }
        s if s < 69.0 => {
            if total_alkali >= 11.0 {
                TasClassification::Trachyte
            } else {
                TasClassification::Dacite
            }
        }
        _ => {
            if total_alkali >= 12.0 {
                TasClassification::Trachyte
            } else {
                TasClassification::Rhyolite
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Mg-number
// ---------------------------------------------------------------------------

/// Mg-number (Mg#) -- molar MgO / (MgO + FeO).
///
/// A high Mg# (~0.7-0.8) indicates primitive, undifferentiated magma;
/// lower values indicate more evolved compositions.
///
/// Returns 0.0 when both `feo` and `mgo` are zero.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mg = mg_number(7.5, 8.0);
/// assert!(mg > 0.5 && mg < 0.8);
/// assert_eq!(mg_number(0.0, 0.0), 0.0);
/// ```
#[must_use]
pub fn mg_number(feo: f64, mgo: f64) -> f64 {
    let mgo_mol = mgo / MW_MGO;
    let feo_mol = feo / MW_FEO;
    let denom = mgo_mol + feo_mol;
    if denom <= 0.0 {
        return 0.0;
    }
    mgo_mol / denom
}

// ---------------------------------------------------------------------------
// Alumina Saturation Index
// ---------------------------------------------------------------------------

/// Alumina Saturation Index (ASI) = Al2O3 / (CaO + Na2O + K2O) in molar
/// proportions (Shand, 1943).
///
/// * ASI > 1 -- peraluminous
/// * ASI < 1 -- metaluminous (or peralkaline when Na+K > Al)
///
/// Returns `f64::INFINITY` when the denominator is zero.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let asi = alumina_saturation_index(16.0, 1.0, 3.0, 4.0);
/// assert!(asi > 1.0); // peraluminous
/// ```
#[must_use]
pub fn alumina_saturation_index(al2o3: f64, cao: f64, na2o: f64, k2o: f64) -> f64 {
    let al_mol = al2o3 / MW_AL2O3;
    let denom = (cao / MW_CAO) + (na2o / MW_NA2O) + (k2o / MW_K2O);
    if denom <= 0.0 {
        return f64::INFINITY;
    }
    al_mol / denom
}

// ---------------------------------------------------------------------------
// ASI Classification
// ---------------------------------------------------------------------------

/// Alumina saturation classification (Shand, 1943).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_asi(1.5), AsiClassification::Peraluminous);
/// assert_eq!(classify_asi(0.8), AsiClassification::Metaluminous);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AsiClassification {
    Peraluminous,
    Metaluminous,
    Peralkaline,
}

/// Classify a rock by its Alumina Saturation Index.
///
/// * ASI > 1.0 -- `Peraluminous`
/// * 0.5 <= ASI <= 1.0 -- `Metaluminous`
/// * ASI < 0.5 -- `Peralkaline`
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_asi(1.5), AsiClassification::Peraluminous);
/// assert_eq!(classify_asi(0.8), AsiClassification::Metaluminous);
/// assert_eq!(classify_asi(0.3), AsiClassification::Peralkaline);
/// ```
#[must_use]
pub fn classify_asi(asi: f64) -> AsiClassification {
    if asi > 1.0 {
        AsiClassification::Peraluminous
    } else if asi >= 0.5 {
        AsiClassification::Metaluminous
    } else {
        AsiClassification::Peralkaline
    }
}

// ---------------------------------------------------------------------------
// Rayleigh Fractionation
// ---------------------------------------------------------------------------

/// Rayleigh fractional crystallization: C_l = C_0 * F^(D - 1).
///
/// * `c0` -- initial concentration of element in the melt
/// * `f_remaining` -- fraction of melt remaining (0 < F <= 1)
/// * `partition_coeff` -- bulk partition coefficient D
///
/// Returns the concentration of the element in the remaining liquid.
///
/// Returns `None` if `f_remaining` is not in (0, 1].
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Incompatible element (D < 1) enriches in melt
/// let c = fractional_crystallization(100.0, 0.5, 0.1).unwrap();
/// assert!(c > 100.0);
/// // No crystallization => unchanged
/// let c2 = fractional_crystallization(42.0, 1.0, 3.0).unwrap();
/// assert!((c2 - 42.0).abs() < 1e-10);
/// // Invalid f_remaining returns None
/// assert!(fractional_crystallization(100.0, 0.0, 1.0).is_none());
/// ```
#[must_use]
pub fn fractional_crystallization(
    c0: f64,
    f_remaining: f64,
    partition_coeff: f64,
) -> Option<f64> {
    if f_remaining <= 0.0 || f_remaining > 1.0 {
        return None;
    }
    Some(c0 * f_remaining.powf(partition_coeff - 1.0))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    /// Helper: a "typical basalt" oxide analysis.
    fn basalt_oxides() -> MajorOxides {
        MajorOxides {
            sio2: 49.5,
            tio2: 1.5,
            al2o3: 15.5,
            fe2o3: 2.5,
            feo: 7.5,
            mno: 0.17,
            mgo: 8.0,
            cao: 11.0,
            na2o: 2.5,
            k2o: 0.5,
            p2o5: 0.2,
            h2o: 0.5,
        }
    }

    /// Helper: a "typical rhyolite" oxide analysis.
    fn rhyolite_oxides() -> MajorOxides {
        MajorOxides {
            sio2: 73.0,
            tio2: 0.2,
            al2o3: 13.5,
            fe2o3: 0.8,
            feo: 1.0,
            mno: 0.05,
            mgo: 0.3,
            cao: 1.2,
            na2o: 3.5,
            k2o: 5.0,
            p2o5: 0.05,
            h2o: 0.5,
        }
    }

    // -- Oxide total & validity --

    #[test]
    fn oxide_total_basalt() {
        let ox = basalt_oxides();
        let t = ox.total();
        assert!(approx_eq(t, 99.37, 0.01), "total = {t}");
    }

    #[test]
    fn oxide_is_valid_basalt() {
        assert!(basalt_oxides().is_valid());
    }

    #[test]
    fn oxide_invalid_low_total() {
        let mut ox = basalt_oxides();
        ox.sio2 = 30.0; // artificially low
        assert!(!ox.is_valid());
    }

    #[test]
    fn oxide_invalid_high_total() {
        let mut ox = basalt_oxides();
        ox.sio2 = 80.0; // way too high
        assert!(!ox.is_valid());
    }

    // -- TAS classification --

    #[test]
    fn tas_basalt() {
        // ~50% SiO2, ~3% total alkali => basalt
        assert_eq!(classify_tas(50.0, 3.0), TasClassification::Basalt);
    }

    #[test]
    fn tas_basalt_from_struct() {
        assert_eq!(
            basalt_oxides().tas_classification(),
            TasClassification::Basalt
        );
    }

    #[test]
    fn tas_rhyolite() {
        // ~73% SiO2, ~8.5% total alkali => rhyolite
        assert_eq!(classify_tas(73.0, 8.5), TasClassification::Rhyolite);
    }

    #[test]
    fn tas_rhyolite_from_struct() {
        assert_eq!(
            rhyolite_oxides().tas_classification(),
            TasClassification::Rhyolite
        );
    }

    #[test]
    fn tas_andesite() {
        assert_eq!(classify_tas(58.0, 4.0), TasClassification::Andesite);
    }

    #[test]
    fn tas_dacite() {
        assert_eq!(classify_tas(66.0, 5.0), TasClassification::Dacite);
    }

    // -- Mg-number --

    #[test]
    fn mg_number_primitive_basalt() {
        // Primitive mantle-derived basalt: ~8% MgO, ~7.5% FeO => Mg# ~0.66
        let mg = mg_number(7.5, 8.0);
        assert!(mg > 0.5 && mg < 0.8, "Mg# = {mg}");
    }

    #[test]
    fn mg_number_zero_inputs() {
        assert_eq!(mg_number(0.0, 0.0), 0.0);
    }

    // -- ASI --

    #[test]
    fn asi_peraluminous_granite() {
        // S-type granite: high Al2O3 relative to CaO+Na2O+K2O
        let asi = alumina_saturation_index(16.0, 1.0, 3.0, 4.0);
        assert!(asi > 1.0, "ASI = {asi}");
        assert_eq!(classify_asi(asi), AsiClassification::Peraluminous);
    }

    #[test]
    fn asi_metaluminous() {
        let asi = alumina_saturation_index(14.0, 5.0, 3.5, 4.5);
        assert!((0.5..1.0).contains(&asi), "ASI = {asi}");
        assert_eq!(classify_asi(asi), AsiClassification::Metaluminous);
    }

    // -- Rayleigh fractionation --

    #[test]
    fn rayleigh_compatible_element() {
        let c = fractional_crystallization(100.0, 0.5, 2.0).unwrap();
        assert!(c < 100.0, "C_l = {c}");
    }

    #[test]
    fn rayleigh_incompatible_element() {
        let c = fractional_crystallization(100.0, 0.5, 0.1).unwrap();
        assert!(c > 100.0, "C_l = {c}");
    }

    #[test]
    fn rayleigh_no_crystallization() {
        let c = fractional_crystallization(42.0, 1.0, 3.0).unwrap();
        assert!(approx_eq(c, 42.0, 1e-10));
    }

    #[test]
    fn rayleigh_d_equals_one() {
        let c = fractional_crystallization(100.0, 0.3, 1.0).unwrap();
        assert!(approx_eq(c, 100.0, 1e-10));
    }

    #[test]
    fn rayleigh_zero_f_returns_none() {
        assert!(fractional_crystallization(100.0, 0.0, 1.0).is_none());
    }

    // -- Serde round-trip --

    #[test]
    fn serde_major_oxides_round_trip() {
        let ox = basalt_oxides();
        let json = serde_json::to_string(&ox).unwrap();
        let deser: MajorOxides = serde_json::from_str(&json).unwrap();
        assert!(approx_eq(ox.total(), deser.total(), 1e-10));
    }

    #[test]
    fn serde_tas_round_trip() {
        let cls = TasClassification::Basalt;
        let json = serde_json::to_string(&cls).unwrap();
        let deser: TasClassification = serde_json::from_str(&json).unwrap();
        assert_eq!(cls, deser);
    }
}
