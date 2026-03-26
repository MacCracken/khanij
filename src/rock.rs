use serde::{Deserialize, Serialize};

/// Rock classification by formation process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RockType {
    Igneous,     // from magma/lava
    Sedimentary, // from deposition
    Metamorphic, // from heat/pressure transformation
}

/// Geological process that drives rock cycle transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GeologicalProcess {
    Weathering,
    Metamorphism,
    Melting,
}

/// A rock with composition and properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rock {
    pub name: String,
    pub rock_type: RockType,
    pub density: f32,  // g/cm³
    pub porosity: f32, // 0.0-1.0
    pub primary_minerals: Vec<String>,
}

impl Rock {
    /// Create a new rock with validated density and porosity.
    /// Returns `None` if density is not positive or porosity is not in `[0.0, 1.0]`.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        rock_type: RockType,
        density: f32,
        porosity: f32,
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

/// Rock cycle transition using a typed geological process.
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
}
