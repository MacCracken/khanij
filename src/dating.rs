//! Radiometric dating — age calculations from radioactive decay of isotope
//! systems (U-Pb, K-Ar, Rb-Sr, C-14).

use serde::{Deserialize, Serialize};

/// Radioactive decay: remaining parent fraction after time `t`.
///
/// N(t) = N₀ · e^(-λt)
///
/// - `decay_constant`: λ in yr⁻¹
/// - `time_years`: elapsed time in years
///
/// Returns fraction of parent remaining (0.0-1.0).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let lambda = IsotopeSystem::C14.decay_constant();
/// let frac = parent_remaining(lambda, half_life(lambda));
/// assert!((frac - 0.5).abs() < 0.001);
/// ```
#[must_use]
pub fn parent_remaining(decay_constant: f64, time_years: f64) -> f64 {
    (-decay_constant * time_years).exp()
}

/// Age from parent/daughter ratio.
///
/// t = (1/λ) · ln(1 + D*/P)
///
/// - `decay_constant`: λ in yr⁻¹
/// - `daughter_parent_ratio`: D*/P (radiogenic daughter / remaining parent)
///
/// Returns age in years.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let lambda = decay_constant(5730.0);
/// let age = age_from_ratio(lambda, 1.0).unwrap();
/// assert!((age - 5730.0).abs() < 10.0); // D/P = 1 means one half-life
/// ```
#[must_use]
pub fn age_from_ratio(decay_constant: f64, daughter_parent_ratio: f64) -> Option<f64> {
    if decay_constant <= 0.0 || daughter_parent_ratio < 0.0 {
        return None;
    }
    Some((1.0 + daughter_parent_ratio).ln() / decay_constant)
}

/// Half-life from decay constant: t½ = ln(2) / λ
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let t = half_life(1.2097e-4);
/// assert!((t - 5730.0).abs() < 5.0);
/// ```
#[must_use]
pub fn half_life(decay_constant: f64) -> f64 {
    std::f64::consts::LN_2 / decay_constant
}

/// Decay constant from half-life: λ = ln(2) / t½
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let lambda = decay_constant(5730.0);
/// let recovered = half_life(lambda);
/// assert!((recovered - 5730.0).abs() < 0.01);
/// ```
#[must_use]
pub fn decay_constant(half_life_years: f64) -> f64 {
    std::f64::consts::LN_2 / half_life_years
}

// ---------------------------------------------------------------------------
// Isotope systems
// ---------------------------------------------------------------------------

/// Common radiometric dating isotope system.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let sys = IsotopeSystem::U238Pb206;
/// let t = sys.half_life_years();
/// assert!((t - 4.468e9).abs() < 0.01e9);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IsotopeSystem {
    /// ²³⁸U → ²⁰⁶Pb (t½ = 4.468 Ga)
    U238Pb206,
    /// ²³⁵U → ²⁰⁷Pb (t½ = 0.7038 Ga)
    U235Pb207,
    /// ⁴⁰K → ⁴⁰Ar (t½ = 1.248 Ga)
    K40Ar40,
    /// ⁸⁷Rb → ⁸⁷Sr (t½ = 49.23 Ga)
    Rb87Sr87,
    /// ¹⁴C → ¹⁴N (t½ = 5730 yr)
    C14,
    /// ¹⁴⁷Sm → ¹⁴³Nd (t½ = 106.0 Ga)
    Sm147Nd143,
    /// ¹⁷⁶Lu → ¹⁷⁶Hf (t½ = 37.1 Ga)
    Lu176Hf176,
}

impl IsotopeSystem {
    /// Decay constant λ in yr⁻¹.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let lambda = IsotopeSystem::C14.decay_constant();
    /// assert!((lambda - 1.2097e-4).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn decay_constant(&self) -> f64 {
        match self {
            Self::U238Pb206 => 1.55125e-10,
            Self::U235Pb207 => 9.8485e-10,
            Self::K40Ar40 => 5.554e-10,
            Self::Rb87Sr87 => 1.42e-11,
            Self::C14 => 1.2097e-4,
            Self::Sm147Nd143 => 6.54e-12,
            Self::Lu176Hf176 => 1.867e-11,
        }
    }

    /// Half-life in years.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let t = IsotopeSystem::C14.half_life_years();
    /// assert!((t - 5730.0).abs() < 5.0);
    /// ```
    #[must_use]
    pub fn half_life_years(&self) -> f64 {
        half_life(self.decay_constant())
    }

    /// Useful age range (approximate, in years).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let (min, max) = IsotopeSystem::C14.useful_range();
    /// assert!(min < max);
    /// assert!((max - 50_000.0).abs() < 1.0);
    /// ```
    #[must_use]
    pub fn useful_range(&self) -> (f64, f64) {
        match self {
            Self::C14 => (100.0, 50_000.0),
            Self::K40Ar40 => (100_000.0, 4.6e9),
            Self::U238Pb206 => (1e6, 4.6e9),
            Self::U235Pb207 => (1e6, 4.6e9),
            Self::Rb87Sr87 => (10e6, 4.6e9),
            Self::Sm147Nd143 => (100e6, 4.6e9),
            Self::Lu176Hf176 => (100e6, 4.6e9),
        }
    }

    /// Calculate age from daughter/parent ratio.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let sys = IsotopeSystem::Rb87Sr87;
    /// let age = sys.age(0.01).unwrap();
    /// assert!(age > 0.0);
    /// ```
    #[must_use]
    pub fn age(&self, daughter_parent_ratio: f64) -> Option<f64> {
        age_from_ratio(self.decay_constant(), daughter_parent_ratio)
    }
}

// ---------------------------------------------------------------------------
// Carbon-14 specific
// ---------------------------------------------------------------------------

/// Carbon-14 age from fraction of modern carbon remaining.
///
/// t = -t½/ln(2) × ln(F)  where F = C14_sample / C14_modern
///
/// - `fraction_modern`: ratio of sample ¹⁴C to modern ¹⁴C (0.0-1.0)
///
/// Returns age in years before present, or `None` if fraction is invalid.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let age = c14_age(0.5).unwrap();
/// assert!((age - 5730.0).abs() < 10.0); // half modern = one half-life
/// assert!(c14_age(0.0).is_none());
/// ```
#[must_use]
pub fn c14_age(fraction_modern: f64) -> Option<f64> {
    if fraction_modern <= 0.0 || fraction_modern > 1.0 {
        return None;
    }
    let lambda = IsotopeSystem::C14.decay_constant();
    Some(-fraction_modern.ln() / lambda)
}

/// Fraction of modern ¹⁴C remaining at a given age.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let frac = c14_fraction_remaining(5730.0);
/// assert!((frac - 0.5).abs() < 0.01);
/// ```
#[must_use]
pub fn c14_fraction_remaining(age_years: f64) -> f64 {
    parent_remaining(IsotopeSystem::C14.decay_constant(), age_years)
}

// ---------------------------------------------------------------------------
// Isochron dating
// ---------------------------------------------------------------------------

/// A single data point for isochron plotting.
///
/// For Rb-Sr: x = ⁸⁷Rb/⁸⁶Sr, y = ⁸⁷Sr/⁸⁶Sr
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let pt = IsochronPoint { x: 0.5, y: 0.710 };
/// assert!((pt.x - 0.5).abs() < f64::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IsochronPoint {
    pub x: f64, // parent/stable ratio
    pub y: f64, // daughter/stable ratio
}

/// Compute an isochron age from a set of data points using linear regression.
///
/// The slope of the isochron line = e^(λt) - 1, so t = ln(slope + 1) / λ.
///
/// Returns `(age_years, initial_ratio)` or `None` if insufficient data.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let sys = IsotopeSystem::Rb87Sr87;
/// let pts = vec![
///     IsochronPoint { x: 0.1, y: 0.705 },
///     IsochronPoint { x: 1.0, y: 0.718 },
/// ];
/// let (age, init) = isochron_age(sys, &pts).unwrap();
/// assert!(age > 0.0);
/// assert!(init > 0.0);
/// ```
#[must_use]
pub fn isochron_age(system: IsotopeSystem, points: &[IsochronPoint]) -> Option<(f64, f64)> {
    if points.len() < 2 {
        return None;
    }
    let n = points.len() as f64;
    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let sum_xy: f64 = points.iter().map(|p| p.x * p.y).sum();
    let sum_x2: f64 = points.iter().map(|p| p.x * p.x).sum();

    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < f64::EPSILON {
        return None;
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;

    if slope < 0.0 {
        return None;
    }

    let age = (1.0 + slope).ln() / system.decay_constant();
    Some((age, intercept))
}

// ---------------------------------------------------------------------------
// Closure temperature
// ---------------------------------------------------------------------------

/// Approximate closure temperature for common mineral-system pairs (°C).
///
/// Closure temperature is the temperature below which the system becomes
/// closed to diffusive loss of daughter isotopes.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let tc = closure_temperature(IsotopeSystem::U238Pb206, "zircon");
/// assert_eq!(tc, Some(900.0));
/// assert!(closure_temperature(IsotopeSystem::C14, "quartz").is_none());
/// ```
#[must_use]
pub fn closure_temperature(system: IsotopeSystem, mineral: &str) -> Option<f64> {
    match (system, mineral.to_lowercase().as_str()) {
        (IsotopeSystem::K40Ar40, "hornblende") => Some(530.0),
        (IsotopeSystem::K40Ar40, "muscovite") => Some(350.0),
        (IsotopeSystem::K40Ar40, "biotite") => Some(310.0),
        (IsotopeSystem::K40Ar40, "feldspar") => Some(150.0),
        (IsotopeSystem::U238Pb206, "zircon") => Some(900.0),
        (IsotopeSystem::U238Pb206, "monazite") => Some(700.0),
        (IsotopeSystem::U238Pb206, "titanite") => Some(600.0),
        (IsotopeSystem::U238Pb206, "apatite") => Some(450.0),
        (IsotopeSystem::U235Pb207, "zircon") => Some(900.0),
        (IsotopeSystem::Rb87Sr87, "muscovite") => Some(500.0),
        (IsotopeSystem::Rb87Sr87, "biotite") => Some(310.0),
        (IsotopeSystem::Rb87Sr87, "feldspar") => Some(200.0),
        (IsotopeSystem::Sm147Nd143, "garnet") => Some(700.0),
        (IsotopeSystem::Lu176Hf176, "garnet") => Some(700.0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c14_age_of_half_modern() {
        // At 50% modern, age should be one half-life (~5730 years)
        let age = c14_age(0.5).unwrap();
        assert!(
            (age - 5730.0).abs() < 10.0,
            "Half modern should give ~5730 yr, got {age}"
        );
    }

    #[test]
    fn c14_modern_sample_is_zero_age() {
        let age = c14_age(1.0).unwrap();
        assert!(age.abs() < 1.0);
    }

    #[test]
    fn c14_invalid_fraction() {
        assert!(c14_age(0.0).is_none());
        assert!(c14_age(-0.5).is_none());
        assert!(c14_age(1.5).is_none());
    }

    #[test]
    fn c14_roundtrip() {
        let age = 10_000.0;
        let fraction = c14_fraction_remaining(age);
        let recovered = c14_age(fraction).unwrap();
        assert!((recovered - age).abs() < 1.0);
    }

    #[test]
    fn u238_half_life() {
        let t = IsotopeSystem::U238Pb206.half_life_years();
        // Expected: ~4.468 Ga
        assert!(
            (t - 4.468e9).abs() < 0.01e9,
            "U-238 half-life should be ~4.468 Ga, got {t}"
        );
    }

    #[test]
    fn age_from_ratio_basic() {
        let system = IsotopeSystem::U238Pb206;
        // If D*/P = e^(λt) - 1 and t = 1 Ga:
        let t = 1.0e9;
        let ratio = (system.decay_constant() * t).exp() - 1.0;
        let recovered = system.age(ratio).unwrap();
        assert!(
            (recovered - t).abs() < 1e6,
            "Should recover 1 Ga, got {recovered}"
        );
    }

    #[test]
    fn parent_remaining_at_one_half_life() {
        let lambda = IsotopeSystem::C14.decay_constant();
        let t = half_life(lambda);
        let remaining = parent_remaining(lambda, t);
        assert!((remaining - 0.5).abs() < 0.001);
    }

    #[test]
    fn decay_constant_half_life_roundtrip() {
        let t_half = 5730.0;
        let lambda = decay_constant(t_half);
        let recovered = half_life(lambda);
        assert!((recovered - t_half).abs() < 0.01);
    }

    #[test]
    fn isochron_two_point() {
        let system = IsotopeSystem::Rb87Sr87;
        let t = 1.0e9;
        let slope = (system.decay_constant() * t).exp() - 1.0;
        let initial = 0.704; // initial ⁸⁷Sr/⁸⁶Sr

        let points = vec![
            IsochronPoint {
                x: 0.1,
                y: initial + slope * 0.1,
            },
            IsochronPoint {
                x: 1.0,
                y: initial + slope * 1.0,
            },
            IsochronPoint {
                x: 5.0,
                y: initial + slope * 5.0,
            },
        ];

        let (age, init) = isochron_age(system, &points).unwrap();
        assert!(
            (age - t).abs() < 1e7,
            "Isochron age should be ~1 Ga, got {age}"
        );
        assert!(
            (init - initial).abs() < 0.001,
            "Initial ratio should be ~0.704, got {init}"
        );
    }

    #[test]
    fn isochron_insufficient_data() {
        assert!(isochron_age(IsotopeSystem::Rb87Sr87, &[]).is_none());
        assert!(
            isochron_age(
                IsotopeSystem::Rb87Sr87,
                &[IsochronPoint { x: 1.0, y: 0.71 }]
            )
            .is_none()
        );
    }

    #[test]
    fn closure_temperature_zircon_u_pb() {
        let tc = closure_temperature(IsotopeSystem::U238Pb206, "zircon").unwrap();
        assert!((tc - 900.0).abs() < 1.0);
    }

    #[test]
    fn closure_temperature_biotite_k_ar() {
        let tc = closure_temperature(IsotopeSystem::K40Ar40, "biotite").unwrap();
        assert!((tc - 310.0).abs() < 1.0);
    }

    #[test]
    fn closure_temperature_unknown() {
        assert!(closure_temperature(IsotopeSystem::C14, "quartz").is_none());
    }

    #[test]
    fn all_systems_have_positive_half_life() {
        for sys in [
            IsotopeSystem::U238Pb206,
            IsotopeSystem::U235Pb207,
            IsotopeSystem::K40Ar40,
            IsotopeSystem::Rb87Sr87,
            IsotopeSystem::C14,
            IsotopeSystem::Sm147Nd143,
            IsotopeSystem::Lu176Hf176,
        ] {
            assert!(sys.half_life_years() > 0.0);
        }
    }

    #[test]
    fn useful_range_ordered() {
        for sys in [
            IsotopeSystem::U238Pb206,
            IsotopeSystem::C14,
            IsotopeSystem::K40Ar40,
        ] {
            let (min, max) = sys.useful_range();
            assert!(max > min);
        }
    }
}
