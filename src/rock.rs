use serde::{Deserialize, Serialize};

/// Rock classification by formation process.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let rock_type = RockType::Igneous;
/// assert_eq!(rock_type, RockType::Igneous);
/// assert_ne!(rock_type, RockType::Sedimentary);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RockType {
    Igneous,     // from magma/lava
    Sedimentary, // from deposition
    Metamorphic, // from heat/pressure transformation
}

/// Geological process that drives rock cycle transitions.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let process = GeologicalProcess::Weathering;
/// assert_eq!(process, GeologicalProcess::Weathering);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GeologicalProcess {
    Weathering,
    Metamorphism,
    Melting,
}

/// A rock with composition and properties.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let rock = Rock::granite();
/// assert_eq!(rock.rock_type, RockType::Igneous);
/// assert!(rock.density > 0.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rock {
    pub name: String,
    pub rock_type: RockType,
    pub density: f64,  // g/cm³
    pub porosity: f64, // 0.0-1.0
    pub primary_minerals: Vec<String>,
}

impl Rock {
    /// Create a new rock with validated density and porosity.
    /// Returns `None` if density is not positive or porosity is not in `[0.0, 1.0]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let rock = Rock::new(
    ///     "Sandstone",
    ///     RockType::Sedimentary,
    ///     2.3,
    ///     0.15,
    ///     vec!["Quartz".into()],
    /// );
    /// assert!(rock.is_some());
    ///
    /// // Invalid density is rejected
    /// let bad = Rock::new("Bad", RockType::Igneous, -1.0, 0.5, vec![]);
    /// assert!(bad.is_none());
    /// ```
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        rock_type: RockType,
        density: f64,
        porosity: f64,
        primary_minerals: Vec<String>,
    ) -> Option<Self> {
        if density <= 0.0 || !(0.0..=1.0).contains(&porosity) {
            return None;
        }
        Some(Self {
            name: name.into(),
            rock_type,
            density,
            porosity,
            primary_minerals,
        })
    }

    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let granite = Rock::granite();
    /// assert_eq!(granite.name, "Granite");
    /// assert_eq!(granite.rock_type, RockType::Igneous);
    /// assert!((granite.density - 2.7).abs() < 0.01);
    /// assert!(granite.primary_minerals.contains(&"Quartz".to_string()));
    /// ```
    #[must_use]
    pub fn granite() -> Self {
        Self {
            name: "Granite".into(),
            rock_type: RockType::Igneous,
            density: 2.7,
            porosity: 0.01,
            primary_minerals: vec!["Quartz".into(), "Feldspar".into(), "Mica".into()],
        }
    }
    #[must_use]
    pub fn sandstone() -> Self {
        Self {
            name: "Sandstone".into(),
            rock_type: RockType::Sedimentary,
            density: 2.3,
            porosity: 0.15,
            primary_minerals: vec!["Quartz".into()],
        }
    }
    #[must_use]
    pub fn marble() -> Self {
        Self {
            name: "Marble".into(),
            rock_type: RockType::Metamorphic,
            density: 2.7,
            porosity: 0.005,
            primary_minerals: vec!["Calcite".into()],
        }
    }
    #[must_use]
    pub fn basalt() -> Self {
        Self {
            name: "Basalt".into(),
            rock_type: RockType::Igneous,
            density: 3.0,
            porosity: 0.01,
            primary_minerals: vec!["Feldspar".into(), "Pyroxene".into()],
        }
    }
    #[must_use]
    pub fn obsidian() -> Self {
        Self {
            name: "Obsidian".into(),
            rock_type: RockType::Igneous,
            density: 2.35,
            porosity: 0.001,
            primary_minerals: vec!["Volcanic glass".into()],
        }
    }
    #[must_use]
    pub fn rhyolite() -> Self {
        Self {
            name: "Rhyolite".into(),
            rock_type: RockType::Igneous,
            density: 2.5,
            porosity: 0.05,
            primary_minerals: vec!["Quartz".into(), "Feldspar".into()],
        }
    }
    #[must_use]
    pub fn limestone() -> Self {
        Self {
            name: "Limestone".into(),
            rock_type: RockType::Sedimentary,
            density: 2.5,
            porosity: 0.10,
            primary_minerals: vec!["Calcite".into()],
        }
    }
    #[must_use]
    pub fn shale() -> Self {
        Self {
            name: "Shale".into(),
            rock_type: RockType::Sedimentary,
            density: 2.4,
            porosity: 0.05,
            primary_minerals: vec!["Clay minerals".into(), "Quartz".into()],
        }
    }
    #[must_use]
    pub fn conglomerate() -> Self {
        Self {
            name: "Conglomerate".into(),
            rock_type: RockType::Sedimentary,
            density: 2.5,
            porosity: 0.12,
            primary_minerals: vec!["Quartz".into(), "Feldspar".into()],
        }
    }
    #[must_use]
    pub fn slate() -> Self {
        Self {
            name: "Slate".into(),
            rock_type: RockType::Metamorphic,
            density: 2.75,
            porosity: 0.005,
            primary_minerals: vec!["Quartz".into(), "Muscovite".into()],
        }
    }
    #[must_use]
    pub fn gneiss() -> Self {
        Self {
            name: "Gneiss".into(),
            rock_type: RockType::Metamorphic,
            density: 2.7,
            porosity: 0.005,
            primary_minerals: vec!["Feldspar".into(), "Quartz".into(), "Mica".into()],
        }
    }
    #[must_use]
    pub fn quartzite() -> Self {
        Self {
            name: "Quartzite".into(),
            rock_type: RockType::Metamorphic,
            density: 2.65,
            porosity: 0.005,
            primary_minerals: vec!["Quartz".into()],
        }
    }
    #[must_use]
    pub fn schist() -> Self {
        Self {
            name: "Schist".into(),
            rock_type: RockType::Metamorphic,
            density: 2.65,
            porosity: 0.01,
            primary_minerals: vec!["Mica".into(), "Quartz".into(), "Feldspar".into()],
        }
    }
}

/// Bulk density of a rock from mineral grain density and porosity.
///
/// ρ_bulk = ρ_grain × (1 - φ) + ρ_fluid × φ
///
/// - `grain_density`: mineral grain density in g/cm³
/// - `porosity`: pore volume fraction (0.0-1.0)
/// - `fluid_density`: pore fluid density in g/cm³ (water: 1.0, air: 0.001)
///
/// Returns bulk density in g/cm³.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Water-saturated sandstone: grain density 2.65, 15% porosity
/// let bd = bulk_density(2.65, 0.15, 1.0);
/// assert!((bd - 2.4025).abs() < 0.01);
/// ```
#[must_use]
pub fn bulk_density(grain_density: f64, porosity: f64, fluid_density: f64) -> f64 {
    grain_density * (1.0 - porosity) + fluid_density * porosity
}

/// Bulk density of a rock from a mixture of minerals.
///
/// - `minerals`: slice of `(density, volume_fraction)` pairs. Volume fractions
///   should sum to 1.0 (the solid portion).
/// - `porosity`: pore volume fraction (0.0-1.0)
/// - `fluid_density`: pore fluid density in g/cm³
///
/// Returns bulk density in g/cm³.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Granite-like mix: quartz 30%, feldspar 60%, mica 10%
/// let minerals = [(2.65, 0.30), (2.56, 0.60), (2.82, 0.10)];
/// let bd = bulk_density_from_minerals(&minerals, 0.01, 1.0);
/// assert!(bd > 2.5 && bd < 2.7);
/// ```
#[must_use]
pub fn bulk_density_from_minerals(
    minerals: &[(f64, f64)],
    porosity: f64,
    fluid_density: f64,
) -> f64 {
    let grain_density: f64 = minerals.iter().map(|(d, f)| d * f).sum();
    bulk_density(grain_density, porosity, fluid_density)
}

/// Porosity from bulk and grain density (assuming air-filled pores).
///
/// φ = 1 - ρ_bulk / ρ_grain
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Sandstone with bulk density 2.25 and grain density 2.65
/// let porosity = porosity_from_density(2.25, 2.65);
/// assert!((porosity - 0.151).abs() < 0.01);
/// ```
#[must_use]
pub fn porosity_from_density(bulk_density: f64, grain_density: f64) -> f64 {
    if grain_density <= 0.0 {
        return 0.0;
    }
    (1.0 - bulk_density / grain_density).clamp(0.0, 1.0)
}

/// Rock cycle transition using a typed geological process.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// // Full rock cycle: Igneous -> Sedimentary -> Metamorphic -> Igneous
/// let sed = rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering).unwrap();
/// assert_eq!(sed, RockType::Sedimentary);
///
/// let meta = rock_cycle_next(sed, GeologicalProcess::Metamorphism).unwrap();
/// assert_eq!(meta, RockType::Metamorphic);
///
/// let igneous = rock_cycle_next(meta, GeologicalProcess::Melting).unwrap();
/// assert_eq!(igneous, RockType::Igneous);
/// ```
#[must_use]
pub fn rock_cycle_next(rock_type: RockType, process: GeologicalProcess) -> Option<RockType> {
    match (rock_type, process) {
        (RockType::Igneous, GeologicalProcess::Weathering) => Some(RockType::Sedimentary),
        (RockType::Sedimentary, GeologicalProcess::Metamorphism) => Some(RockType::Metamorphic),
        (RockType::Metamorphic, GeologicalProcess::Melting) => Some(RockType::Igneous),
        (RockType::Igneous, GeologicalProcess::Metamorphism) => Some(RockType::Metamorphic),
        (RockType::Sedimentary, GeologicalProcess::Melting) => Some(RockType::Igneous),
        (RockType::Metamorphic, GeologicalProcess::Weathering) => Some(RockType::Sedimentary),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_cycle_complete() {
        let start = RockType::Igneous;
        let sed = rock_cycle_next(start, GeologicalProcess::Weathering).unwrap();
        let meta = rock_cycle_next(sed, GeologicalProcess::Metamorphism).unwrap();
        let back = rock_cycle_next(meta, GeologicalProcess::Melting).unwrap();
        assert_eq!(back, RockType::Igneous);
    }

    #[test]
    fn granite_is_igneous() {
        assert_eq!(Rock::granite().rock_type, RockType::Igneous);
    }

    #[test]
    fn marble_is_metamorphic() {
        assert_eq!(Rock::marble().rock_type, RockType::Metamorphic);
    }

    #[test]
    fn porosity_in_range() {
        for r in [
            Rock::granite(),
            Rock::sandstone(),
            Rock::marble(),
            Rock::basalt(),
        ] {
            assert!(
                (0.0..=1.0).contains(&r.porosity),
                "{} porosity out of range",
                r.name
            );
        }
    }

    #[test]
    fn invalid_process_returns_none() {
        assert!(rock_cycle_next(RockType::Igneous, GeologicalProcess::Melting).is_none());
    }

    #[test]
    fn validated_rock_rejects_bad_density() {
        assert!(Rock::new("Bad", RockType::Igneous, -1.0, 0.5, vec![]).is_none());
        assert!(Rock::new("Bad", RockType::Igneous, 0.0, 0.5, vec![]).is_none());
    }

    #[test]
    fn validated_rock_rejects_bad_porosity() {
        assert!(Rock::new("Bad", RockType::Igneous, 2.5, -0.1, vec![]).is_none());
        assert!(Rock::new("Bad", RockType::Igneous, 2.5, 1.1, vec![]).is_none());
    }

    #[test]
    fn validated_rock_accepts_valid() {
        assert!(
            Rock::new(
                "Good",
                RockType::Sedimentary,
                2.3,
                0.15,
                vec!["Quartz".into()]
            )
            .is_some()
        );
    }

    #[test]
    fn bulk_density_no_porosity() {
        let bd = bulk_density(2.65, 0.0, 1.0);
        assert!((bd - 2.65).abs() < 0.01);
    }

    #[test]
    fn bulk_density_with_water() {
        // Sandstone: grain 2.65, porosity 15%, water-filled
        let bd = bulk_density(2.65, 0.15, 1.0);
        // Expected: 2.65*0.85 + 1.0*0.15 = 2.4025
        assert!((bd - 2.4025).abs() < 0.01);
    }

    #[test]
    fn bulk_density_from_mineral_mix() {
        // Granite: quartz (2.65, 30%), feldspar (2.56, 60%), mica (2.82, 10%)
        let minerals = [(2.65, 0.30), (2.56, 0.60), (2.82, 0.10)];
        let bd = bulk_density_from_minerals(&minerals, 0.01, 0.001);
        assert!(bd > 2.5 && bd < 2.7);
    }

    #[test]
    fn porosity_from_density_roundtrip() {
        let grain = 2.65_f64;
        let phi = 0.15_f64;
        let bd = bulk_density(grain, phi, 0.001); // air-filled
        let recovered = porosity_from_density(bd, grain);
        assert!((recovered - phi).abs() < 0.01);
    }
}
