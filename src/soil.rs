use serde::{Deserialize, Serialize};

/// Soil texture classification (USDA).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SoilTexture { Sand, LoamySand, SandyLoam, Loam, SiltLoam, Silt, SandyClayLoam, ClayLoam, SiltyClayLoam, SandyClay, SiltyClay, Clay }

/// Soil composition by particle size fractions (sum to 1.0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoilComposition {
    pub sand: f32,  // > 0.05mm
    pub silt: f32,  // 0.002-0.05mm
    pub clay: f32,  // < 0.002mm
}

impl SoilComposition {
    #[must_use]
    pub fn new(sand: f32, silt: f32, clay: f32) -> Option<Self> {
        let sum = sand + silt + clay;
        if (sum - 1.0).abs() > 0.01 || sand < 0.0 || silt < 0.0 || clay < 0.0 { return None; }
        Some(Self { sand, silt, clay })
    }

    /// Classify soil texture from composition (simplified USDA triangle).
    #[must_use]
    pub fn texture(&self) -> SoilTexture {
        if self.clay > 0.4 { SoilTexture::Clay }
        else if self.sand > 0.7 { SoilTexture::Sand }
        else if self.silt > 0.7 { SoilTexture::Silt }
        else { SoilTexture::Loam }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_composition() {
        assert!(SoilComposition::new(0.4, 0.4, 0.2).is_some());
    }

    #[test]
    fn invalid_composition_sum() {
        assert!(SoilComposition::new(0.5, 0.5, 0.5).is_none());
    }

    #[test]
    fn sandy_soil() {
        let s = SoilComposition::new(0.8, 0.1, 0.1).unwrap();
        assert_eq!(s.texture(), SoilTexture::Sand);
    }

    #[test]
    fn clay_soil() {
        let s = SoilComposition::new(0.2, 0.3, 0.5).unwrap();
        assert_eq!(s.texture(), SoilTexture::Clay);
    }
}
