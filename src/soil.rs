use serde::{Deserialize, Serialize};

/// Soil texture classification (USDA).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SoilTexture {
    Sand,
    LoamySand,
    SandyLoam,
    Loam,
    SiltLoam,
    Silt,
    SandyClayLoam,
    ClayLoam,
    SiltyClayLoam,
    SandyClay,
    SiltyClay,
    Clay,
}

/// Soil composition by particle size fractions (sum to 1.0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoilComposition {
    pub sand: f32, // > 0.05mm
    pub silt: f32, // 0.002-0.05mm
    pub clay: f32, // < 0.002mm
}

impl SoilComposition {
    #[must_use]
    pub fn new(sand: f32, silt: f32, clay: f32) -> Option<Self> {
        let sum = sand + silt + clay;
        if (sum - 1.0).abs() > 0.01 || sand < 0.0 || silt < 0.0 || clay < 0.0 {
            return None;
        }
        Some(Self { sand, silt, clay })
    }

    /// Classify soil texture from composition using the USDA soil texture triangle.
    ///
    /// Boundaries follow the standard USDA classification with all 12 texture
    /// classes reachable. Percentages are expressed as fractions (0.0–1.0).
    #[must_use]
    pub fn texture(&self) -> SoilTexture {
        let sand = self.sand;
        let silt = self.silt;
        let clay = self.clay;

        // Heavy clay region
        if clay >= 0.40 {
            if sand >= 0.45 {
                SoilTexture::SandyClay
            } else if silt >= 0.40 {
                SoilTexture::SiltyClay
            } else {
                SoilTexture::Clay
            }
        }
        // Sandy clay loam band (20-35% clay, high sand)
        else if (0.20..0.35).contains(&clay) && sand >= 0.45 {
            SoilTexture::SandyClayLoam
        }
        // Clay loam region (27-40% clay)
        else if (0.27..0.40).contains(&clay) {
            if silt >= 0.40 {
                SoilTexture::SiltyClayLoam
            } else {
                SoilTexture::ClayLoam
            }
        }
        // Sandy clay loam (remaining 35-40% clay, high sand)
        else if clay >= 0.35 && sand >= 0.45 {
            SoilTexture::SandyClayLoam
        }
        // Sand (>=85% sand, <=10% clay)
        else if sand >= 0.85 && clay < 0.10 {
            SoilTexture::Sand
        }
        // Loamy sand (70-90% sand, <=15% clay)
        else if (0.70..0.90).contains(&sand) && clay < 0.15 {
            SoilTexture::LoamySand
        }
        // Sandy loam (high sand, moderate fines)
        else if sand >= 0.43 && clay < 0.20 && silt < 0.50 {
            SoilTexture::SandyLoam
        }
        // Silt (>=80% silt, <=12% clay)
        else if silt >= 0.80 && clay < 0.12 {
            SoilTexture::Silt
        }
        // Silt loam (>=50% silt, <=27% clay)
        else if silt >= 0.50 && clay < 0.27 {
            SoilTexture::SiltLoam
        }
        // Loam (the middle ground)
        else {
            SoilTexture::Loam
        }
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
    fn negative_fraction_rejected() {
        assert!(SoilComposition::new(-0.1, 0.6, 0.5).is_none());
    }

    // --- All 12 texture classes ---

    #[test]
    fn texture_sand() {
        let s = SoilComposition::new(0.90, 0.05, 0.05).unwrap();
        assert_eq!(s.texture(), SoilTexture::Sand);
    }

    #[test]
    fn texture_loamy_sand() {
        let s = SoilComposition::new(0.80, 0.10, 0.10).unwrap();
        assert_eq!(s.texture(), SoilTexture::LoamySand);
    }

    #[test]
    fn texture_sandy_loam() {
        let s = SoilComposition::new(0.60, 0.25, 0.15).unwrap();
        assert_eq!(s.texture(), SoilTexture::SandyLoam);
    }

    #[test]
    fn texture_loam() {
        let s = SoilComposition::new(0.40, 0.40, 0.20).unwrap();
        assert_eq!(s.texture(), SoilTexture::Loam);
    }

    #[test]
    fn texture_silt_loam() {
        let s = SoilComposition::new(0.20, 0.65, 0.15).unwrap();
        assert_eq!(s.texture(), SoilTexture::SiltLoam);
    }

    #[test]
    fn texture_silt() {
        let s = SoilComposition::new(0.05, 0.88, 0.07).unwrap();
        assert_eq!(s.texture(), SoilTexture::Silt);
    }

    #[test]
    fn texture_sandy_clay_loam() {
        let s = SoilComposition::new(0.55, 0.15, 0.30).unwrap();
        assert_eq!(s.texture(), SoilTexture::SandyClayLoam);
    }

    #[test]
    fn texture_clay_loam() {
        let s = SoilComposition::new(0.30, 0.35, 0.35).unwrap();
        assert_eq!(s.texture(), SoilTexture::ClayLoam);
    }

    #[test]
    fn texture_silty_clay_loam() {
        let s = SoilComposition::new(0.10, 0.55, 0.35).unwrap();
        assert_eq!(s.texture(), SoilTexture::SiltyClayLoam);
    }

    #[test]
    fn texture_sandy_clay() {
        let s = SoilComposition::new(0.50, 0.05, 0.45).unwrap();
        assert_eq!(s.texture(), SoilTexture::SandyClay);
    }

    #[test]
    fn texture_silty_clay() {
        let s = SoilComposition::new(0.05, 0.50, 0.45).unwrap();
        assert_eq!(s.texture(), SoilTexture::SiltyClay);
    }

    #[test]
    fn texture_clay() {
        let s = SoilComposition::new(0.20, 0.30, 0.50).unwrap();
        assert_eq!(s.texture(), SoilTexture::Clay);
    }
}
