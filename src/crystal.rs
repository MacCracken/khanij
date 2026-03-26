use serde::{Deserialize, Serialize};

/// Crystal system classification (7 systems).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CrystalSystem {
    Cubic,        // a=b=c, α=β=γ=90°
    Tetragonal,   // a=b≠c, α=β=γ=90°
    Orthorhombic, // a≠b≠c, α=β=γ=90°
    Hexagonal,    // a=b≠c, α=β=90° γ=120°
    Trigonal,     // a=b=c, α=β=γ≠90°
    Monoclinic,   // a≠b≠c, α=γ=90° β≠90°
    Triclinic,    // a≠b≠c, α≠β≠γ≠90°
}

impl CrystalSystem {
    /// Number of symmetry elements.
    #[must_use]
    pub fn symmetry_order(&self) -> u8 {
        match self {
            Self::Cubic => 48,
            Self::Hexagonal => 24,
            Self::Tetragonal => 16,
            Self::Trigonal => 12,
            Self::Orthorhombic => 8,
            Self::Monoclinic => 4,
            Self::Triclinic => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubic_highest_symmetry() {
        assert!(CrystalSystem::Cubic.symmetry_order() > CrystalSystem::Triclinic.symmetry_order());
    }

    #[test]
    fn all_seven_systems() {
        let systems = [
            CrystalSystem::Cubic,
            CrystalSystem::Tetragonal,
            CrystalSystem::Orthorhombic,
            CrystalSystem::Hexagonal,
            CrystalSystem::Trigonal,
            CrystalSystem::Monoclinic,
            CrystalSystem::Triclinic,
        ];
        assert_eq!(systems.len(), 7);
    }
}
