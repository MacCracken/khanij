use serde::{Deserialize, Serialize};

/// Mohs hardness scale (1-10).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MohsHardness(f32);

impl MohsHardness {
    #[must_use]
    pub fn new(value: f32) -> Option<Self> {
        if (1.0..=10.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
    /// Can this mineral scratch the other?
    #[must_use]
    #[inline]
    pub fn scratches(&self, other: &Self) -> bool {
        self.0 > other.0
    }
}

/// A mineral with physical properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mineral {
    pub name: String,
    pub formula: String,
    pub hardness: MohsHardness,
    pub density: f32, // g/cm³
    pub crystal_system: super::crystal::CrystalSystem,
    pub luster: Luster,
    pub color: String,
}

/// Mineral luster classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Luster {
    Metallic,
    Vitreous,
    Pearly,
    Silky,
    Resinous,
    Adamantine,
    Waxy,
    Earthy,
    Dull,
}

impl Mineral {
    #[must_use]
    pub fn quartz() -> Self {
        Self {
            name: "Quartz".into(),
            formula: "SiO₂".into(),
            hardness: MohsHardness(7.0),
            density: 2.65,
            crystal_system: super::crystal::CrystalSystem::Hexagonal,
            luster: Luster::Vitreous,
            color: "colorless/white".into(),
        }
    }
    #[must_use]
    pub fn feldspar() -> Self {
        Self {
            name: "Feldspar".into(),
            formula: "KAlSi₃O₈".into(),
            hardness: MohsHardness(6.0),
            density: 2.56,
            crystal_system: super::crystal::CrystalSystem::Monoclinic,
            luster: Luster::Vitreous,
            color: "white/pink".into(),
        }
    }
    #[must_use]
    pub fn calcite() -> Self {
        Self {
            name: "Calcite".into(),
            formula: "CaCO₃".into(),
            hardness: MohsHardness(3.0),
            density: 2.71,
            crystal_system: super::crystal::CrystalSystem::Hexagonal,
            luster: Luster::Vitreous,
            color: "white/colorless".into(),
        }
    }
    #[must_use]
    pub fn diamond() -> Self {
        Self {
            name: "Diamond".into(),
            formula: "C".into(),
            hardness: MohsHardness(10.0),
            density: 3.52,
            crystal_system: super::crystal::CrystalSystem::Cubic,
            luster: Luster::Adamantine,
            color: "colorless".into(),
        }
    }
    #[must_use]
    pub fn talc() -> Self {
        Self {
            name: "Talc".into(),
            formula: "Mg₃Si₄O₁₀(OH)₂".into(),
            hardness: MohsHardness(1.0),
            density: 2.75,
            crystal_system: super::crystal::CrystalSystem::Monoclinic,
            luster: Luster::Pearly,
            color: "white/green".into(),
        }
    }

    #[must_use]
    pub fn olivine() -> Self {
        Self {
            name: "Olivine".into(),
            formula: "(Mg,Fe)₂SiO₄".into(),
            hardness: MohsHardness(6.5),
            density: 3.30,
            crystal_system: super::crystal::CrystalSystem::Orthorhombic,
            luster: Luster::Vitreous,
            color: "green".into(),
        }
    }
    #[must_use]
    pub fn pyrite() -> Self {
        Self {
            name: "Pyrite".into(),
            formula: "FeS₂".into(),
            hardness: MohsHardness(6.0),
            density: 5.01,
            crystal_system: super::crystal::CrystalSystem::Cubic,
            luster: Luster::Metallic,
            color: "brass-yellow".into(),
        }
    }
    #[must_use]
    pub fn magnetite() -> Self {
        Self {
            name: "Magnetite".into(),
            formula: "Fe₃O₄".into(),
            hardness: MohsHardness(5.5),
            density: 5.17,
            crystal_system: super::crystal::CrystalSystem::Cubic,
            luster: Luster::Metallic,
            color: "black".into(),
        }
    }
    #[must_use]
    pub fn halite() -> Self {
        Self {
            name: "Halite".into(),
            formula: "NaCl".into(),
            hardness: MohsHardness(2.5),
            density: 2.17,
            crystal_system: super::crystal::CrystalSystem::Cubic,
            luster: Luster::Vitreous,
            color: "colorless/white".into(),
        }
    }
    #[must_use]
    pub fn gypsum() -> Self {
        Self {
            name: "Gypsum".into(),
            formula: "CaSO₄·2H₂O".into(),
            hardness: MohsHardness(2.0),
            density: 2.31,
            crystal_system: super::crystal::CrystalSystem::Monoclinic,
            luster: Luster::Vitreous,
            color: "white/colorless".into(),
        }
    }
    #[must_use]
    pub fn muscovite() -> Self {
        Self {
            name: "Muscovite".into(),
            formula: "KAl₂(AlSi₃O₁₀)(OH)₂".into(),
            hardness: MohsHardness(2.5),
            density: 2.82,
            crystal_system: super::crystal::CrystalSystem::Monoclinic,
            luster: Luster::Vitreous,
            color: "colorless/silver".into(),
        }
    }
    #[must_use]
    pub fn fluorite() -> Self {
        Self {
            name: "Fluorite".into(),
            formula: "CaF₂".into(),
            hardness: MohsHardness(4.0),
            density: 3.18,
            crystal_system: super::crystal::CrystalSystem::Cubic,
            luster: Luster::Vitreous,
            color: "purple/green/yellow".into(),
        }
    }
    #[must_use]
    pub fn apatite() -> Self {
        Self {
            name: "Apatite".into(),
            formula: "Ca₅(PO₄)₃(F,Cl,OH)".into(),
            hardness: MohsHardness(5.0),
            density: 3.19,
            crystal_system: super::crystal::CrystalSystem::Hexagonal,
            luster: Luster::Vitreous,
            color: "green/blue".into(),
        }
    }
    #[must_use]
    pub fn corundum() -> Self {
        Self {
            name: "Corundum".into(),
            formula: "Al₂O₃".into(),
            hardness: MohsHardness(9.0),
            density: 4.02,
            crystal_system: super::crystal::CrystalSystem::Hexagonal,
            luster: Luster::Adamantine,
            color: "varies".into(),
        }
    }
    #[must_use]
    pub fn topaz() -> Self {
        Self {
            name: "Topaz".into(),
            formula: "Al₂SiO₄(F,OH)₂".into(),
            hardness: MohsHardness(8.0),
            density: 3.53,
            crystal_system: super::crystal::CrystalSystem::Orthorhombic,
            luster: Luster::Vitreous,
            color: "colorless/yellow/blue".into(),
        }
    }

    /// Parse this mineral's formula string into a [`super::formula::Formula`].
    #[must_use]
    pub fn parsed_formula(&self) -> Option<super::formula::Formula> {
        super::formula::Formula::parse(&self.formula)
    }

    /// Build a kimiya `Molecule` from this mineral's formula string.
    ///
    /// Uses the formula parser — works for any mineral, not just presets.
    /// Requires the `chemistry` feature.
    #[cfg(feature = "chemistry")]
    #[must_use]
    pub fn molecule(&self) -> Option<kimiya::Molecule> {
        self.parsed_formula().and_then(|f| f.to_molecule())
    }

    /// Molecular weight in g/mol via kimiya.
    ///
    /// Uses the formula parser — works for any mineral, not just presets.
    /// Requires the `chemistry` feature.
    #[cfg(feature = "chemistry")]
    #[must_use]
    pub fn molecular_weight(&self) -> Option<f64> {
        self.parsed_formula().and_then(|f| f.molecular_weight())
    }
}

// ---------------------------------------------------------------------------
// Chemistry-gated helpers (kimiya)
// ---------------------------------------------------------------------------

/// Lattice energy of an ionic mineral using the Born-Landé equation.
/// Requires the `chemistry` feature.
///
/// - `madelung`: Madelung constant for the crystal structure
/// - `z_cation` / `z_anion`: ion charges
/// - `r0_m`: equilibrium inter-ionic distance in metres
/// - `born_exponent`: Born exponent (5-12, depends on electron config)
#[cfg(feature = "chemistry")]
#[must_use]
pub fn lattice_energy(
    madelung: f64,
    z_cation: u32,
    z_anion: u32,
    r0_m: f64,
    born_exponent: f64,
) -> Option<f64> {
    kimiya::inorganic::born_lande_lattice_energy(madelung, z_cation, z_anion, r0_m, born_exponent)
        .ok()
}

/// Look up Shannon ionic radius for an element in a mineral.
/// Returns radius in picometres.
/// Requires the `chemistry` feature.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn ionic_radius(symbol: &str, charge: i8) -> Option<f64> {
    kimiya::inorganic::lookup_ionic_radius(symbol, charge).map(|r| r.radius_pm)
}

/// Dissolution rate constant at a given temperature using Arrhenius kinetics.
/// - `pre_exponential`: frequency factor (s⁻¹)
/// - `activation_energy_j`: activation energy in joules per mole
/// - `temperature_k`: temperature in kelvin
///
/// Requires the `chemistry` feature.
#[cfg(feature = "chemistry")]
#[must_use]
pub fn dissolution_rate(
    pre_exponential: f64,
    activation_energy_j: f64,
    temperature_k: f64,
) -> Option<f64> {
    kimiya::arrhenius_rate(pre_exponential, activation_energy_j, temperature_k).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mohs_range() {
        assert!(MohsHardness::new(7.0).is_some());
        assert!(MohsHardness::new(0.5).is_none());
        assert!(MohsHardness::new(11.0).is_none());
    }

    #[test]
    fn diamond_scratches_everything() {
        let diamond = MohsHardness::new(10.0).unwrap();
        let quartz = MohsHardness::new(7.0).unwrap();
        assert!(diamond.scratches(&quartz));
        assert!(!quartz.scratches(&diamond));
    }

    #[test]
    fn mineral_presets() {
        assert!((Mineral::quartz().hardness.value() - 7.0).abs() < f32::EPSILON);
        assert!((Mineral::diamond().hardness.value() - 10.0).abs() < f32::EPSILON);
        assert!((Mineral::talc().hardness.value() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn density_positive() {
        let minerals = [
            Mineral::quartz(),
            Mineral::feldspar(),
            Mineral::calcite(),
            Mineral::diamond(),
            Mineral::talc(),
        ];
        for m in &minerals {
            assert!(m.density > 0.0, "{} density should be positive", m.name);
        }
    }

    #[test]
    fn hardness_type_prevents_invalid() {
        assert!(MohsHardness::new(0.0).is_none());
        assert!(MohsHardness::new(10.5).is_none());
        assert!(MohsHardness::new(1.0).is_some());
        assert!(MohsHardness::new(10.0).is_some());
    }
}

#[cfg(all(test, feature = "chemistry"))]
mod chemistry_tests {
    use super::*;

    #[test]
    fn quartz_molecular_weight() {
        let mw = Mineral::quartz().molecular_weight().unwrap();
        assert!(
            (mw - 60.08).abs() < 0.1,
            "SiO₂ should be ~60.08 g/mol, got {mw}"
        );
    }

    #[test]
    fn calcite_molecular_weight() {
        let mw = Mineral::calcite().molecular_weight().unwrap();
        assert!(
            (mw - 100.09).abs() < 0.1,
            "CaCO₃ should be ~100.09 g/mol, got {mw}"
        );
    }

    #[test]
    fn diamond_molecule() {
        let mol = Mineral::diamond().molecule().unwrap();
        assert_eq!(mol.total_atoms(), 1);
    }

    #[test]
    fn dissolution_rate_increases_with_temperature() {
        let cold = dissolution_rate(1e10, 60_000.0, 283.15).unwrap(); // 10°C
        let hot = dissolution_rate(1e10, 60_000.0, 313.15).unwrap(); // 40°C
        assert!(hot > cold);
    }

    #[test]
    fn nacl_lattice_energy() {
        // NaCl: Madelung 1.7476, z+=1, z-=1, r0≈2.81e-10m, Born exp ≈ 8
        let energy = lattice_energy(kimiya::inorganic::MADELUNG_NACL, 1, 1, 2.81e-10, 8.0);
        assert!(energy.is_some());
        let e = energy.unwrap();
        // Expected ~750 kJ/mol (negative convention varies)
        assert!(
            e.abs() > 600_000.0,
            "NaCl lattice energy should be >600 kJ/mol, got {e}"
        );
    }

    #[test]
    fn ionic_radius_lookup() {
        let r = ionic_radius("Na", 1);
        assert!(r.is_some());
        let pm = r.unwrap();
        assert!(
            pm > 90.0 && pm < 120.0,
            "Na+ radius should be ~102 pm, got {pm}"
        );
    }
}
