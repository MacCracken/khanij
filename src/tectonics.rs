//! Plate tectonics — Euler poles, plate velocities, boundary types, and
//! subduction geometry.

use serde::{Deserialize, Serialize};

/// Plate boundary type.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let bt = BoundaryType::Divergent;
/// assert_eq!(bt, BoundaryType::Divergent);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BoundaryType {
    /// Divergent — plates move apart (mid-ocean ridges, rift valleys).
    Divergent,
    /// Convergent — plates collide (subduction zones, collision belts).
    Convergent,
    /// Transform — plates slide past each other (strike-slip faults).
    Transform,
}

/// An Euler pole describing plate rotation on a sphere.
///
/// All plate motions on a sphere can be described as rotations about an
/// Euler pole at (latitude, longitude) with an angular velocity ω.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let pole = EulerPole {
///     latitude_deg: 0.0,
///     longitude_deg: 0.0,
///     omega_deg_per_myr: 1.0,
/// };
/// let v = pole.velocity_mm_yr(90.0);
/// assert!(v > 100.0 && v < 120.0);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EulerPole {
    /// Latitude of the pole in degrees (-90 to 90).
    pub latitude_deg: f64,
    /// Longitude of the pole in degrees (-180 to 180).
    pub longitude_deg: f64,
    /// Angular velocity in degrees per million years.
    pub omega_deg_per_myr: f64,
}

impl EulerPole {
    /// Linear plate velocity at a given angular distance from the Euler pole.
    ///
    /// v = ω × R × sin(Δ)
    ///
    /// - `angular_distance_deg`: angle between the Euler pole and the point
    ///
    /// Returns velocity in mm/yr.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let pole = EulerPole {
    ///     latitude_deg: 0.0,
    ///     longitude_deg: 0.0,
    ///     omega_deg_per_myr: 1.0,
    /// };
    /// let v = pole.velocity_mm_yr(0.0);
    /// assert!(v.abs() < 0.01);
    /// ```
    #[must_use]
    pub fn velocity_mm_yr(&self, angular_distance_deg: f64) -> f64 {
        let earth_radius_km = 6371.0;
        let omega_rad_per_yr = self.omega_deg_per_myr.to_radians() / 1e6;
        let delta_rad = angular_distance_deg.to_radians();
        // v = ω × R × sin(Δ), convert km/yr to mm/yr
        omega_rad_per_yr * earth_radius_km * delta_rad.sin() * 1e6
    }

    /// Maximum velocity (at 90° from the pole).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let pole = EulerPole {
    ///     latitude_deg: 0.0,
    ///     longitude_deg: 0.0,
    ///     omega_deg_per_myr: 1.0,
    /// };
    /// assert!(pole.max_velocity_mm_yr() > 100.0);
    /// ```
    #[must_use]
    pub fn max_velocity_mm_yr(&self) -> f64 {
        self.velocity_mm_yr(90.0)
    }
}

/// Spreading rate at a mid-ocean ridge.
///
/// - `half_rate_mm_yr`: half-spreading rate in mm/yr
///
/// Returns full spreading rate in mm/yr.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let full = full_spreading_rate(12.0);
/// assert!((full - 24.0).abs() < 1e-10);
/// ```
#[must_use]
pub fn full_spreading_rate(half_rate_mm_yr: f64) -> f64 {
    2.0 * half_rate_mm_yr
}

/// Ridge classification by spreading rate.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let rt = classify_ridge(30.0);
/// assert_eq!(rt, RidgeType::Slow);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RidgeType {
    /// < 20 mm/yr full rate (e.g., Arctic ridges).
    UltraSlow,
    /// 20-55 mm/yr (e.g., Mid-Atlantic Ridge).
    Slow,
    /// 55-75 mm/yr (e.g., intermediate Pacific).
    Intermediate,
    /// 75-150 mm/yr (e.g., East Pacific Rise).
    Fast,
    /// > 150 mm/yr.
    UltraFast,
}

/// Classify a mid-ocean ridge by its full spreading rate.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_ridge(10.0), RidgeType::UltraSlow);
/// assert_eq!(classify_ridge(100.0), RidgeType::Fast);
/// ```
#[must_use]
pub fn classify_ridge(full_rate_mm_yr: f64) -> RidgeType {
    if full_rate_mm_yr < 20.0 {
        RidgeType::UltraSlow
    } else if full_rate_mm_yr < 55.0 {
        RidgeType::Slow
    } else if full_rate_mm_yr < 75.0 {
        RidgeType::Intermediate
    } else if full_rate_mm_yr < 150.0 {
        RidgeType::Fast
    } else {
        RidgeType::UltraFast
    }
}

/// Subduction zone geometry.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let sz = SubductionZone {
///     dip_deg: 45.0,
///     convergence_rate_mm_yr: 80.0,
///     plate_age_ma: 100.0,
/// };
/// let depth = sz.slab_depth_km(100.0);
/// assert!((depth - 100.0).abs() < 0.1);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SubductionZone {
    /// Dip angle of the slab in degrees.
    pub dip_deg: f64,
    /// Convergence rate in mm/yr.
    pub convergence_rate_mm_yr: f64,
    /// Age of the subducting plate in Ma.
    pub plate_age_ma: f64,
}

impl SubductionZone {
    /// Depth to the slab at a given horizontal distance from the trench.
    ///
    /// depth = distance × tan(dip)
    ///
    /// Returns depth in km.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let sz = SubductionZone {
    ///     dip_deg: 45.0,
    ///     convergence_rate_mm_yr: 80.0,
    ///     plate_age_ma: 100.0,
    /// };
    /// let deep = sz.slab_depth_km(200.0);
    /// assert!(deep > sz.slab_depth_km(50.0));
    /// ```
    #[must_use]
    pub fn slab_depth_km(&self, distance_from_trench_km: f64) -> f64 {
        distance_from_trench_km * self.dip_deg.to_radians().tan()
    }

    /// Estimated slab dip from plate age (older plates dip steeper).
    ///
    /// Empirical: dip ≈ 30° + 0.3 × age_Ma (capped at 80°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let dip = SubductionZone::estimated_dip_from_age(100.0);
    /// assert!((dip - 60.0).abs() < 0.01);
    /// ```
    #[must_use]
    pub fn estimated_dip_from_age(plate_age_ma: f64) -> f64 {
        (30.0 + 0.3 * plate_age_ma).min(80.0)
    }
}

/// Estimate ocean floor age from distance to the ridge axis.
///
/// age = distance / (half_spreading_rate)
///
/// - `distance_km`: distance from ridge axis in km
/// - `half_rate_mm_yr`: half-spreading rate in mm/yr
///
/// Returns age in Ma.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let age = ocean_floor_age(500.0, 25.0);
/// assert!((age - 20.0).abs() < 0.1);
/// ```
#[must_use]
pub fn ocean_floor_age(distance_km: f64, half_rate_mm_yr: f64) -> f64 {
    if half_rate_mm_yr <= 0.0 {
        return 0.0;
    }
    // km / (mm/yr) → need consistent units
    // distance_km × 1e6 mm/km / (half_rate mm/yr) = age in years / 1e6 = Ma
    distance_km * 1e6 / half_rate_mm_yr / 1e6
}

/// Lithosphere thickness from plate age (cooling half-space model).
///
/// h ≈ 10 × √(age_Ma) km (Parsons & Sclater, 1977)
///
/// Returns thickness in km.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let h = lithosphere_thickness(100.0);
/// assert!((h - 100.0).abs() < 1.0);
/// ```
#[must_use]
pub fn lithosphere_thickness(age_ma: f64) -> f64 {
    if age_ma <= 0.0 {
        return 0.0;
    }
    10.0 * age_ma.sqrt()
}

/// Ocean depth from plate age (cooling half-space model).
///
/// d ≈ 2500 + 350 × √(age_Ma) metres (Parsons & Sclater, 1977)
///
/// Returns depth in metres.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let depth = ocean_depth_m(0.0);
/// assert!((depth - 2500.0).abs() < 1.0);
/// ```
#[must_use]
pub fn ocean_depth_m(age_ma: f64) -> f64 {
    if age_ma <= 0.0 {
        return 2500.0; // ridge crest depth
    }
    2500.0 + 350.0 * age_ma.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_pole_velocity_at_equator() {
        let pole = EulerPole {
            latitude_deg: 0.0,
            longitude_deg: 0.0,
            omega_deg_per_myr: 1.0,
        };
        let v = pole.velocity_mm_yr(90.0);
        // v = ω × R = (1°/Myr) × 6371 km ≈ 111 mm/yr
        assert!(v > 100.0 && v < 120.0, "Expected ~111 mm/yr, got {v}");
    }

    #[test]
    fn euler_pole_zero_at_pole() {
        let pole = EulerPole {
            latitude_deg: 90.0,
            longitude_deg: 0.0,
            omega_deg_per_myr: 1.0,
        };
        let v = pole.velocity_mm_yr(0.0);
        assert!(v.abs() < 0.01);
    }

    #[test]
    fn spreading_rate_classification() {
        assert_eq!(classify_ridge(10.0), RidgeType::UltraSlow);
        assert_eq!(classify_ridge(30.0), RidgeType::Slow);
        assert_eq!(classify_ridge(60.0), RidgeType::Intermediate);
        assert_eq!(classify_ridge(100.0), RidgeType::Fast);
        assert_eq!(classify_ridge(160.0), RidgeType::UltraFast);
    }

    #[test]
    fn mid_atlantic_ridge_is_slow() {
        // MAR half-rate ~12 mm/yr → full rate ~24 mm/yr
        let full = full_spreading_rate(12.0);
        assert_eq!(classify_ridge(full), RidgeType::Slow);
    }

    #[test]
    fn slab_depth_increases_with_distance() {
        let sz = SubductionZone {
            dip_deg: 45.0,
            convergence_rate_mm_yr: 80.0,
            plate_age_ma: 100.0,
        };
        let shallow = sz.slab_depth_km(50.0);
        let deep = sz.slab_depth_km(200.0);
        assert!(deep > shallow);
    }

    #[test]
    fn slab_depth_45_degrees() {
        let sz = SubductionZone {
            dip_deg: 45.0,
            convergence_rate_mm_yr: 80.0,
            plate_age_ma: 100.0,
        };
        let depth = sz.slab_depth_km(100.0);
        assert!(
            (depth - 100.0).abs() < 0.1,
            "45° dip at 100km → ~100km depth, got {depth}"
        );
    }

    #[test]
    fn older_plates_dip_steeper() {
        let young = SubductionZone::estimated_dip_from_age(10.0);
        let old = SubductionZone::estimated_dip_from_age(100.0);
        assert!(old > young);
    }

    #[test]
    fn dip_capped_at_80() {
        let very_old = SubductionZone::estimated_dip_from_age(500.0);
        assert!((very_old - 80.0).abs() < 0.01);
    }

    #[test]
    fn ocean_floor_age_basic() {
        // 500 km from ridge at 25 mm/yr half-rate = 20 Ma
        let age = ocean_floor_age(500.0, 25.0);
        assert!((age - 20.0).abs() < 0.1);
    }

    #[test]
    fn lithosphere_thickens_with_age() {
        let young = lithosphere_thickness(10.0);
        let old = lithosphere_thickness(100.0);
        assert!(old > young);
        // 100 Ma → ~100 km
        assert!((old - 100.0).abs() < 1.0);
    }

    #[test]
    fn ocean_deepens_with_age() {
        let ridge = ocean_depth_m(0.0);
        let old = ocean_depth_m(100.0);
        assert!(old > ridge);
        assert!((ridge - 2500.0).abs() < 1.0);
    }
}
