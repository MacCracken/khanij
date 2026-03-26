use serde::{Deserialize, Serialize};

/// A crystallographic unit cell defined by lattice parameters.
///
/// Lengths `a`, `b`, `c` are in angstroms (Å).
/// Angles `alpha`, `beta`, `gamma` are in degrees.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cell = UnitCell::cubic(5.64);
/// assert!(cell.is_cubic());
/// assert!((cell.volume() - 5.64_f64.powi(3)).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UnitCell {
    /// Lattice parameter a (Å).
    pub a: f64,
    /// Lattice parameter b (Å).
    pub b: f64,
    /// Lattice parameter c (Å).
    pub c: f64,
    /// Angle α between b and c axes (degrees).
    pub alpha: f64,
    /// Angle β between a and c axes (degrees).
    pub beta: f64,
    /// Angle γ between a and b axes (degrees).
    pub gamma: f64,
}

impl UnitCell {
    /// Create a new unit cell from lattice parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::new(4.0, 5.0, 6.0, 90.0, 90.0, 90.0);
    /// assert!(cell.is_orthorhombic());
    /// ```
    #[must_use]
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }

    /// Create a cubic unit cell (a = b = c, α = β = γ = 90°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::cubic(3.0);
    /// assert!(cell.is_cubic());
    /// assert!((cell.volume() - 27.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn cubic(a: f64) -> Self {
        Self::new(a, a, a, 90.0, 90.0, 90.0)
    }

    /// Create a hexagonal unit cell (a = b ≠ c, α = β = 90°, γ = 120°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::hexagonal(4.913, 5.405);
    /// assert!(cell.is_hexagonal());
    /// ```
    #[must_use]
    pub fn hexagonal(a: f64, c: f64) -> Self {
        Self::new(a, a, c, 90.0, 90.0, 120.0)
    }

    /// Create a rhombohedral unit cell (a = b ≠ c, α = β = 90°, γ = 120°)
    /// using the hexagonal setting commonly used in crystallography.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::rhombohedral(4.989, 17.062);
    /// assert!(cell.is_hexagonal());
    /// ```
    #[must_use]
    pub fn rhombohedral(a: f64, c: f64) -> Self {
        Self::hexagonal(a, c)
    }

    /// Create an orthorhombic unit cell (a ≠ b ≠ c, α = β = γ = 90°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::orthorhombic(3.0, 4.0, 5.0);
    /// assert!((cell.volume() - 60.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn orthorhombic(a: f64, b: f64, c: f64) -> Self {
        Self::new(a, b, c, 90.0, 90.0, 90.0)
    }

    // ── Preset minerals ──────────────────────────────────────────────

    /// Halite (NaCl) — cubic, a = 5.64 Å.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::halite();
    /// assert!(cell.is_cubic());
    /// assert!((cell.a - 5.64).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn halite() -> Self {
        Self::cubic(5.64)
    }

    /// Quartz (SiO₂) — hexagonal, a = 4.913 Å, c = 5.405 Å.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::quartz();
    /// assert!(cell.is_hexagonal());
    /// ```
    #[must_use]
    pub fn quartz() -> Self {
        Self::hexagonal(4.913, 5.405)
    }

    /// Calcite (CaCO₃) — rhombohedral (hex setting), a = 4.989 Å, c = 17.062 Å.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::calcite();
    /// assert!((cell.a - 4.989).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn calcite() -> Self {
        Self::rhombohedral(4.989, 17.062)
    }

    /// Diamond (C) — cubic, a = 3.567 Å.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::diamond();
    /// assert!(cell.is_cubic());
    /// assert!((cell.a - 3.567).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn diamond() -> Self {
        Self::cubic(3.567)
    }

    // ── Geometry helpers ─────────────────────────────────────────────

    /// Unit cell volume using the general triclinic formula (ų).
    ///
    /// V = abc √(1 − cos²α − cos²β − cos²γ + 2·cosα·cosβ·cosγ)
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let vol = UnitCell::cubic(5.0).volume();
    /// assert!((vol - 125.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn volume(&self) -> f64 {
        let ca = self.alpha.to_radians().cos();
        let cb = self.beta.to_radians().cos();
        let cg = self.gamma.to_radians().cos();
        let factor = 1.0 - ca * ca - cb * cb - cg * cg + 2.0 * ca * cb * cg;
        self.a * self.b * self.c * factor.sqrt()
    }

    /// Returns `true` if the cell is cubic (a ≈ b ≈ c, all angles ≈ 90°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert!(UnitCell::halite().is_cubic());
    /// assert!(!UnitCell::quartz().is_cubic());
    /// ```
    #[must_use]
    pub fn is_cubic(&self) -> bool {
        let tol = 1e-6;
        (self.a - self.b).abs() < tol
            && (self.b - self.c).abs() < tol
            && (self.alpha - 90.0).abs() < tol
            && (self.beta - 90.0).abs() < tol
            && (self.gamma - 90.0).abs() < tol
    }

    /// Returns `true` if the cell is hexagonal (a ≈ b, α ≈ β ≈ 90°, γ ≈ 120°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert!(UnitCell::quartz().is_hexagonal());
    /// assert!(!UnitCell::halite().is_hexagonal());
    /// ```
    #[must_use]
    pub fn is_hexagonal(&self) -> bool {
        let tol = 1e-6;
        (self.a - self.b).abs() < tol
            && (self.alpha - 90.0).abs() < tol
            && (self.beta - 90.0).abs() < tol
            && (self.gamma - 120.0).abs() < tol
    }

    /// Returns `true` if the cell is tetragonal (a ≈ b ≠ c, all angles ≈ 90°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::new(4.0, 4.0, 6.0, 90.0, 90.0, 90.0);
    /// assert!(cell.is_tetragonal());
    /// ```
    #[must_use]
    pub fn is_tetragonal(&self) -> bool {
        let tol = 1e-6;
        (self.a - self.b).abs() < tol
            && (self.a - self.c).abs() >= tol
            && (self.alpha - 90.0).abs() < tol
            && (self.beta - 90.0).abs() < tol
            && (self.gamma - 90.0).abs() < tol
    }

    /// Returns `true` if the cell is orthorhombic (a ≠ b ≠ c, all angles ≈ 90°).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let cell = UnitCell::orthorhombic(3.0, 4.0, 5.0);
    /// assert!(cell.is_orthorhombic());
    /// ```
    #[must_use]
    pub fn is_orthorhombic(&self) -> bool {
        let tol = 1e-6;
        (self.alpha - 90.0).abs() < tol
            && (self.beta - 90.0).abs() < tol
            && (self.gamma - 90.0).abs() < tol
            && (self.a - self.b).abs() >= tol
            && (self.b - self.c).abs() >= tol
    }
}

/// Miller indices (hkl) denoting a crystallographic plane.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let hkl = MillerIndex::new(1, 1, 1);
/// assert_eq!(format!("{hkl}"), "(1 1 1)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MillerIndex {
    /// h index.
    pub h: i32,
    /// k index.
    pub k: i32,
    /// l index.
    pub l: i32,
}

impl MillerIndex {
    /// Create a new Miller index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let hkl = MillerIndex::new(2, 0, 0);
    /// assert_eq!(hkl.h, 2);
    /// ```
    #[must_use]
    pub fn new(h: i32, k: i32, l: i32) -> Self {
        Self { h, k, l }
    }
}

impl core::fmt::Display for MillerIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({} {} {})", self.h, self.k, self.l)
    }
}

/// Interplanar d-spacing for a given unit cell and Miller index.
///
/// - **Cubic** (a = b = c, 90° angles): d = a / √(h² + k² + l²)
/// - **Orthorhombic** (90° angles, a ≠ b ≠ c): 1/d² = h²/a² + k²/b² + l²/c²
/// - Falls back to orthorhombic formula for any cell with all-90° angles.
///
/// # Panics
///
/// Panics if the denominator under the square root is zero (i.e. h = k = l = 0).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cell = UnitCell::halite();
/// let hkl = MillerIndex::new(2, 0, 0);
/// let d = d_spacing(&cell, &hkl);
/// assert!((d - 2.82).abs() < 0.01);
/// ```
#[must_use]
pub fn d_spacing(cell: &UnitCell, hkl: &MillerIndex) -> f64 {
    let h2 = f64::from(hkl.h * hkl.h);
    let k2 = f64::from(hkl.k * hkl.k);
    let l2 = f64::from(hkl.l * hkl.l);

    if cell.is_cubic() {
        // d = a / √(h² + k² + l²)
        cell.a / (h2 + k2 + l2).sqrt()
    } else {
        // General orthorhombic: 1/d² = h²/a² + k²/b² + l²/c²
        let inv_d2 = h2 / (cell.a * cell.a) + k2 / (cell.b * cell.b) + l2 / (cell.c * cell.c);
        1.0 / inv_d2.sqrt()
    }
}

/// Bragg angle θ (in degrees) from Bragg's law: nλ = 2d sin θ.
///
/// Uses first-order diffraction (n = 1). Returns `None` when λ / (2d) > 1
/// (no diffraction possible).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let angle = bragg_angle(3.256, 1.5406).unwrap();
/// assert!((angle - 13.68).abs() < 0.05);
/// assert!(bragg_angle(0.5, 1.5406).is_none());
/// ```
#[must_use]
pub fn bragg_angle(d_spacing: f64, wavelength: f64) -> Option<f64> {
    let sin_theta = wavelength / (2.0 * d_spacing);
    if sin_theta.abs() > 1.0 {
        None
    } else {
        Some(sin_theta.asin().to_degrees())
    }
}

/// Wavelength λ (Å) from Bragg's law: λ = 2d sin θ.
///
/// `angle_deg` is the Bragg angle θ in degrees (first-order, n = 1).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let d = 3.256;
/// let angle = bragg_angle(d, 1.5406).unwrap();
/// let lambda = bragg_wavelength(d, angle);
/// assert!((lambda - 1.5406).abs() < 1e-4);
/// ```
#[must_use]
pub fn bragg_wavelength(d_spacing: f64, angle_deg: f64) -> f64 {
    2.0 * d_spacing * angle_deg.to_radians().sin()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-6;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    // ── Volume tests ─────────────────────────────────────────────────

    #[test]
    fn cubic_volume_is_a_cubed() {
        let cell = UnitCell::cubic(5.0);
        assert!(approx(cell.volume(), 125.0, EPS));
    }

    #[test]
    fn halite_volume() {
        let cell = UnitCell::halite();
        let expected = 5.64_f64.powi(3); // 179.406…
        assert!(approx(cell.volume(), expected, 1e-3));
    }

    #[test]
    fn orthorhombic_volume() {
        let cell = UnitCell::orthorhombic(3.0, 4.0, 5.0);
        assert!(approx(cell.volume(), 60.0, EPS));
    }

    // ── System identification ────────────────────────────────────────

    #[test]
    fn halite_is_cubic() {
        assert!(UnitCell::halite().is_cubic());
        assert!(!UnitCell::halite().is_hexagonal());
    }

    #[test]
    fn quartz_is_hexagonal() {
        assert!(UnitCell::quartz().is_hexagonal());
        assert!(!UnitCell::quartz().is_cubic());
    }

    // ── d-spacing ────────────────────────────────────────────────────

    #[test]
    fn nacl_111_d_spacing() {
        // Halite (111): d = 5.64 / √3 ≈ 3.2563
        let cell = UnitCell::halite();
        let hkl = MillerIndex::new(1, 1, 1);
        let d = d_spacing(&cell, &hkl);
        let expected = 5.64 / 3.0_f64.sqrt();
        assert!(approx(d, expected, 1e-4));
    }

    #[test]
    fn nacl_200_d_spacing() {
        // Halite (200): d = 5.64 / √4 = 2.82
        let cell = UnitCell::halite();
        let hkl = MillerIndex::new(2, 0, 0);
        let d = d_spacing(&cell, &hkl);
        assert!(approx(d, 2.82, 1e-4));
    }

    // ── Bragg angle ──────────────────────────────────────────────────

    #[test]
    fn bragg_angle_cu_ka_nacl_111() {
        // Cu Kα λ = 1.5406 Å, NaCl (111) d ≈ 3.2563 Å
        // sin θ = 1.5406 / (2 × 3.2563) ≈ 0.23654 → θ ≈ 13.68°
        let d = 5.64 / 3.0_f64.sqrt();
        let angle = bragg_angle(d, 1.5406).expect("valid angle");
        assert!(approx(angle, 13.68, 0.05));
    }

    #[test]
    fn bragg_angle_returns_none_when_impossible() {
        // Very small d, large wavelength → sin θ > 1
        assert!(bragg_angle(0.5, 1.5406).is_none());
    }

    // ── Bragg wavelength (roundtrip) ─────────────────────────────────

    #[test]
    fn bragg_roundtrip() {
        let d = 3.256;
        let wavelength = 1.5406;
        let angle = bragg_angle(d, wavelength).unwrap();
        let recovered = bragg_wavelength(d, angle);
        assert!(approx(recovered, wavelength, 1e-6));
    }

    // ── Presets ──────────────────────────────────────────────────────

    #[test]
    fn preset_diamond() {
        let d = UnitCell::diamond();
        assert!(d.is_cubic());
        assert!(approx(d.a, 3.567, EPS));
    }

    #[test]
    fn preset_calcite() {
        let c = UnitCell::calcite();
        assert!(c.is_hexagonal());
        assert!(approx(c.a, 4.989, EPS));
        assert!(approx(c.c, 17.062, EPS));
    }

    // ── Serde roundtrip ──────────────────────────────────────────────

    #[test]
    fn serde_roundtrip_unit_cell() {
        let cell = UnitCell::quartz();
        let json = serde_json::to_string(&cell).unwrap();
        let back: UnitCell = serde_json::from_str(&json).unwrap();
        assert_eq!(cell, back);
    }

    #[test]
    fn serde_roundtrip_miller() {
        let hkl = MillerIndex::new(1, -1, 2);
        let json = serde_json::to_string(&hkl).unwrap();
        let back: MillerIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(hkl, back);
    }

    // ── Display ──────────────────────────────────────────────────────

    #[test]
    fn miller_display() {
        let hkl = MillerIndex::new(1, 1, 1);
        assert_eq!(format!("{hkl}"), "(1 1 1)");
    }
}
