use serde::{Deserialize, Serialize};

/// Mohs hardness scale (1-10).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MohsHardness(f32);

impl MohsHardness {
    #[must_use]
    pub fn new(value: f32) -> Option<Self> {
        if (1.0..=10.0).contains(&value) { Some(Self(value)) } else { None }
    }
    #[must_use] #[inline]
    pub fn value(&self) -> f32 { self.0 }
    /// Can this mineral scratch the other?
    #[must_use] #[inline]
    pub fn scratches(&self, other: &Self) -> bool { self.0 > other.0 }
}

/// A mineral with physical properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mineral {
    pub name: String,
    pub formula: String,
    pub hardness: f32,
    pub density: f32,          // g/cm³
    pub crystal_system: super::crystal::CrystalSystem,
    pub luster: Luster,
    pub color: String,
}

/// Mineral luster classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Luster {
    Metallic, Vitreous, Pearly, Silky, Resinous, Adamantine, Waxy, Earthy, Dull,
}

impl Mineral {
    #[must_use] pub fn quartz() -> Self {
        Self { name: "Quartz".into(), formula: "SiO₂".into(), hardness: 7.0, density: 2.65, crystal_system: super::crystal::CrystalSystem::Hexagonal, luster: Luster::Vitreous, color: "colorless/white".into() }
    }
    #[must_use] pub fn feldspar() -> Self {
        Self { name: "Feldspar".into(), formula: "KAlSi₃O₈".into(), hardness: 6.0, density: 2.56, crystal_system: super::crystal::CrystalSystem::Monoclinic, luster: Luster::Vitreous, color: "white/pink".into() }
    }
    #[must_use] pub fn calcite() -> Self {
        Self { name: "Calcite".into(), formula: "CaCO₃".into(), hardness: 3.0, density: 2.71, crystal_system: super::crystal::CrystalSystem::Hexagonal, luster: Luster::Vitreous, color: "white/colorless".into() }
    }
    #[must_use] pub fn diamond() -> Self {
        Self { name: "Diamond".into(), formula: "C".into(), hardness: 10.0, density: 3.52, crystal_system: super::crystal::CrystalSystem::Cubic, luster: Luster::Adamantine, color: "colorless".into() }
    }
    #[must_use] pub fn talc() -> Self {
        Self { name: "Talc".into(), formula: "Mg₃Si₄O₁₀(OH)₂".into(), hardness: 1.0, density: 2.75, crystal_system: super::crystal::CrystalSystem::Monoclinic, luster: Luster::Pearly, color: "white/green".into() }
    }
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
        assert!((Mineral::quartz().hardness - 7.0).abs() < f32::EPSILON);
        assert!((Mineral::diamond().hardness - 10.0).abs() < f32::EPSILON);
        assert!((Mineral::talc().hardness - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn density_positive() {
        let minerals = [Mineral::quartz(), Mineral::feldspar(), Mineral::calcite(), Mineral::diamond(), Mineral::talc()];
        for m in &minerals { assert!(m.density > 0.0, "{} density should be positive", m.name); }
    }
}
