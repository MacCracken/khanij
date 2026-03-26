use serde::{Deserialize, Serialize};

/// Rock classification by formation process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RockType {
    Igneous,      // from magma/lava
    Sedimentary,  // from deposition
    Metamorphic,  // from heat/pressure transformation
}

/// A rock with composition and properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rock {
    pub name: String,
    pub rock_type: RockType,
    pub density: f32,       // g/cm³
    pub porosity: f32,      // 0.0-1.0
    pub primary_minerals: Vec<String>,
}

impl Rock {
    #[must_use] pub fn granite() -> Self {
        Self { name: "Granite".into(), rock_type: RockType::Igneous, density: 2.7, porosity: 0.01, primary_minerals: vec!["Quartz".into(), "Feldspar".into(), "Mica".into()] }
    }
    #[must_use] pub fn sandstone() -> Self {
        Self { name: "Sandstone".into(), rock_type: RockType::Sedimentary, density: 2.3, porosity: 0.15, primary_minerals: vec!["Quartz".into()] }
    }
    #[must_use] pub fn marble() -> Self {
        Self { name: "Marble".into(), rock_type: RockType::Metamorphic, density: 2.7, porosity: 0.005, primary_minerals: vec!["Calcite".into()] }
    }
    #[must_use] pub fn basalt() -> Self {
        Self { name: "Basalt".into(), rock_type: RockType::Igneous, density: 3.0, porosity: 0.01, primary_minerals: vec!["Feldspar".into(), "Pyroxene".into()] }
    }
}

/// Rock cycle transition.
#[must_use]
pub fn rock_cycle_next(rock_type: RockType, process: &str) -> Option<RockType> {
    match (rock_type, process) {
        (RockType::Igneous, "weathering") => Some(RockType::Sedimentary),
        (RockType::Sedimentary, "metamorphism") => Some(RockType::Metamorphic),
        (RockType::Metamorphic, "melting") => Some(RockType::Igneous),
        (RockType::Igneous, "metamorphism") => Some(RockType::Metamorphic),
        (RockType::Sedimentary, "melting") => Some(RockType::Igneous),
        (RockType::Metamorphic, "weathering") => Some(RockType::Sedimentary),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_cycle_complete() {
        let start = RockType::Igneous;
        let sed = rock_cycle_next(start, "weathering").unwrap();
        let meta = rock_cycle_next(sed, "metamorphism").unwrap();
        let back = rock_cycle_next(meta, "melting").unwrap();
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
        for r in [Rock::granite(), Rock::sandstone(), Rock::marble(), Rock::basalt()] {
            assert!((0.0..=1.0).contains(&r.porosity), "{} porosity out of range", r.name);
        }
    }
}
