use serde::{Deserialize, Serialize};

/// Soil texture classification (USDA).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let comp = SoilComposition::new(0.40, 0.40, 0.20).unwrap();
/// assert_eq!(comp.texture(), SoilTexture::Loam);
/// ```
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
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let comp = SoilComposition::new(0.40, 0.40, 0.20).unwrap();
/// assert_eq!(comp.texture(), SoilTexture::Loam);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoilComposition {
    pub sand: f32, // > 0.05mm
    pub silt: f32, // 0.002-0.05mm
    pub clay: f32, // < 0.002mm
}

impl SoilComposition {
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert!(SoilComposition::new(0.40, 0.40, 0.20).is_some());
    /// assert!(SoilComposition::new(0.5, 0.5, 0.5).is_none()); // sum != 1
    /// ```
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
    /// classes reachable. Percentages are expressed as fractions (0.0-1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let comp = SoilComposition::new(0.90, 0.05, 0.05).unwrap();
    /// assert_eq!(comp.texture(), SoilTexture::Sand);
    /// ```
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

// ---------------------------------------------------------------------------
// USDA Soil Taxonomy — Orders
// ---------------------------------------------------------------------------

/// USDA Soil Taxonomy order -- the 12 top-level soil classifications.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(SoilOrder::ALL.len(), 12);
/// assert_eq!(SoilOrder::Mollisol.fertility(), SoilFertility::High);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SoilOrder {
    /// Soils with subsurface clay accumulation and moderate-high base saturation.
    Alfisol,
    /// Soils formed in volcanic ash with high water-holding capacity.
    Andisol,
    /// Dry soils of arid regions with accumulations of salts or carbonates.
    Aridisol,
    /// Young soils with little profile development (recently deposited).
    Entisol,
    /// Soils with permafrost within 2m of the surface.
    Gelisol,
    /// Organic soils (peat/muck) with >20% organic matter.
    Histosol,
    /// Young soils with beginning horizon development.
    Inceptisol,
    /// Dark, fertile prairie/grassland soils with thick organic-rich A horizon.
    Mollisol,
    /// Highly weathered tropical soils dominated by iron/aluminium oxides.
    Oxisol,
    /// Acidic forest soils with a subsurface accumulation of humus and Al/Fe.
    Spodosol,
    /// Highly weathered soils with low base saturation and clay accumulation.
    Ultisol,
    /// Clay-rich soils that shrink and swell with moisture changes.
    Vertisol,
}

impl SoilOrder {
    /// All 12 soil orders.
    pub const ALL: &'static [SoilOrder] = &[
        Self::Alfisol,
        Self::Andisol,
        Self::Aridisol,
        Self::Entisol,
        Self::Gelisol,
        Self::Histosol,
        Self::Inceptisol,
        Self::Mollisol,
        Self::Oxisol,
        Self::Spodosol,
        Self::Ultisol,
        Self::Vertisol,
    ];

    /// Typical climate/environment for this soil order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let env = SoilOrder::Mollisol.typical_environment();
    /// assert_eq!(env, "grasslands/prairies");
    /// ```
    #[must_use]
    pub fn typical_environment(&self) -> &'static str {
        match self {
            Self::Alfisol => "temperate deciduous forest",
            Self::Andisol => "volcanic regions",
            Self::Aridisol => "arid/semi-arid deserts",
            Self::Entisol => "recent deposits (floodplains, dunes)",
            Self::Gelisol => "permafrost regions (tundra)",
            Self::Histosol => "wetlands (bogs, marshes)",
            Self::Inceptisol => "young landscapes (mountains, river terraces)",
            Self::Mollisol => "grasslands/prairies",
            Self::Oxisol => "tropical rainforest",
            Self::Spodosol => "coniferous forest (boreal/cool humid)",
            Self::Ultisol => "subtropical humid forest",
            Self::Vertisol => "seasonal wet-dry climates",
        }
    }

    /// Typical fertility level.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// assert_eq!(SoilOrder::Oxisol.fertility(), SoilFertility::VeryLow);
    /// ```
    #[must_use]
    pub fn fertility(&self) -> SoilFertility {
        match self {
            Self::Mollisol | Self::Alfisol | Self::Andisol => SoilFertility::High,
            Self::Vertisol | Self::Inceptisol | Self::Entisol => SoilFertility::Moderate,
            Self::Histosol | Self::Aridisol | Self::Gelisol => SoilFertility::Low,
            Self::Ultisol | Self::Spodosol | Self::Oxisol => SoilFertility::VeryLow,
        }
    }
}

/// Soil fertility classification.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(SoilFertility::High > SoilFertility::VeryLow);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoilFertility {
    VeryLow,
    Low,
    Moderate,
    High,
}

// ---------------------------------------------------------------------------
// Soil Horizons
// ---------------------------------------------------------------------------

/// Master soil horizon designation.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let ht = HorizonType::A;
/// assert_eq!(ht, HorizonType::A);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HorizonType {
    /// Organic layer (decomposing plant material on surface).
    O,
    /// Topsoil — mineral horizon with organic matter accumulation.
    A,
    /// Eluviation zone — leached of clay, iron, and organic matter.
    E,
    /// Subsoil — zone of accumulation (illuviation) of clay, iron, carbonates.
    B,
    /// Parent material — partially weathered rock.
    C,
    /// Bedrock — unweathered rock.
    R,
}

/// A single soil horizon with properties.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let h = SoilHorizon {
///     horizon_type: HorizonType::A,
///     depth_top_cm: 0.0,
///     depth_bottom_cm: 30.0,
///     organic_matter: 0.04,
///     ph: 6.5,
///     texture: SoilTexture::Loam,
///     color: "10YR 3/2".into(),
/// };
/// assert!((h.thickness_cm() - 30.0).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilHorizon {
    /// Horizon type (O, A, E, B, C, R).
    pub horizon_type: HorizonType,
    /// Depth to top of horizon in cm.
    pub depth_top_cm: f32,
    /// Depth to bottom of horizon in cm.
    pub depth_bottom_cm: f32,
    /// Organic matter content as fraction (0.0-1.0).
    pub organic_matter: f32,
    /// pH value (0-14).
    pub ph: f32,
    /// Texture of this horizon.
    pub texture: SoilTexture,
    /// Color (Munsell notation or description).
    pub color: String,
}

impl SoilHorizon {
    /// Thickness of this horizon in cm.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let h = SoilHorizon {
    ///     horizon_type: HorizonType::B,
    ///     depth_top_cm: 30.0,
    ///     depth_bottom_cm: 80.0,
    ///     organic_matter: 0.01,
    ///     ph: 6.0,
    ///     texture: SoilTexture::ClayLoam,
    ///     color: "7.5YR 4/4".into(),
    /// };
    /// assert!((h.thickness_cm() - 50.0).abs() < 0.01);
    /// ```
    #[must_use]
    pub fn thickness_cm(&self) -> f32 {
        self.depth_bottom_cm - self.depth_top_cm
    }
}

/// A complete soil profile -- a vertical sequence of horizons.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let profile = SoilProfile {
///     horizons: vec![SoilHorizon {
///         horizon_type: HorizonType::A,
///         depth_top_cm: 0.0,
///         depth_bottom_cm: 25.0,
///         organic_matter: 0.03,
///         ph: 6.0,
///         texture: SoilTexture::SandyLoam,
///         color: "10YR 3/2".into(),
///     }],
///     location: "field site".into(),
/// };
/// assert!((profile.total_depth_cm() - 25.0).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilProfile {
    pub horizons: Vec<SoilHorizon>,
    pub location: String,
}

impl SoilProfile {
    /// Total depth of the profile in cm.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 40.0,
    ///         organic_matter: 0.04,
    ///         ph: 6.5,
    ///         texture: SoilTexture::Loam,
    ///         color: "dark".into(),
    ///     }],
    ///     location: "site".into(),
    /// };
    /// assert!((profile.total_depth_cm() - 40.0).abs() < 0.01);
    /// ```
    #[must_use]
    pub fn total_depth_cm(&self) -> f32 {
        self.horizons
            .iter()
            .map(|h| h.depth_bottom_cm)
            .fold(0.0_f32, f32::max)
    }

    /// Check if the profile has a specific horizon type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 30.0,
    ///         organic_matter: 0.03,
    ///         ph: 6.0,
    ///         texture: SoilTexture::Loam,
    ///         color: "dark".into(),
    ///     }],
    ///     location: "site".into(),
    /// };
    /// assert!(profile.has_horizon(HorizonType::A));
    /// assert!(!profile.has_horizon(HorizonType::B));
    /// ```
    #[must_use]
    pub fn has_horizon(&self, ht: HorizonType) -> bool {
        self.horizons.iter().any(|h| h.horizon_type == ht)
    }

    /// Get the A horizon (topsoil) if present.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 30.0,
    ///         organic_matter: 0.04,
    ///         ph: 6.5,
    ///         texture: SoilTexture::Loam,
    ///         color: "dark".into(),
    ///     }],
    ///     location: "site".into(),
    /// };
    /// assert!(profile.a_horizon().is_some());
    /// ```
    #[must_use]
    pub fn a_horizon(&self) -> Option<&SoilHorizon> {
        self.horizons
            .iter()
            .find(|h| h.horizon_type == HorizonType::A)
    }

    /// Get the B horizon (subsoil) if present.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 30.0,
    ///         organic_matter: 0.03,
    ///         ph: 6.0,
    ///         texture: SoilTexture::Loam,
    ///         color: "dark".into(),
    ///     }],
    ///     location: "site".into(),
    /// };
    /// assert!(profile.b_horizon().is_none());
    /// ```
    #[must_use]
    pub fn b_horizon(&self) -> Option<&SoilHorizon> {
        self.horizons
            .iter()
            .find(|h| h.horizon_type == HorizonType::B)
    }

    /// Average organic matter content of the A horizon.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 30.0,
    ///         organic_matter: 0.05,
    ///         ph: 6.5,
    ///         texture: SoilTexture::Loam,
    ///         color: "dark".into(),
    ///     }],
    ///     location: "site".into(),
    /// };
    /// let om = profile.topsoil_organic_matter().unwrap();
    /// assert!((om - 0.05).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn topsoil_organic_matter(&self) -> Option<f32> {
        self.a_horizon().map(|h| h.organic_matter)
    }

    /// Simplified soil order classification from profile properties.
    ///
    /// This is a simplified key -- real taxonomy uses many more diagnostic criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let profile = SoilProfile {
    ///     horizons: vec![SoilHorizon {
    ///         horizon_type: HorizonType::A,
    ///         depth_top_cm: 0.0,
    ///         depth_bottom_cm: 20.0,
    ///         organic_matter: 0.02,
    ///         ph: 6.0,
    ///         texture: SoilTexture::Sand,
    ///         color: "10YR 5/3".into(),
    ///     }],
    ///     location: "floodplain".into(),
    /// };
    /// assert_eq!(profile.classify_order(), SoilOrder::Entisol);
    /// ```
    #[must_use]
    pub fn classify_order(&self) -> SoilOrder {
        let a = self.a_horizon();
        let b = self.b_horizon();

        // Histosol: very high organic matter
        if a.is_some_and(|h| h.organic_matter > 0.20) {
            return SoilOrder::Histosol;
        }

        // Vertisol: very high clay content
        if a.is_some_and(|h| h.texture == SoilTexture::Clay)
            && b.is_some_and(|h| h.texture == SoilTexture::Clay)
        {
            return SoilOrder::Vertisol;
        }

        // Mollisol: thick, dark, organic-rich A horizon
        if a.is_some_and(|h| h.organic_matter > 0.03 && h.thickness_cm() > 25.0 && h.ph > 5.5) {
            return SoilOrder::Mollisol;
        }

        // Spodosol: E horizon above B with acidic pH
        if self.has_horizon(HorizonType::E) && a.is_some_and(|h| h.ph < 5.0) {
            return SoilOrder::Spodosol;
        }

        // Oxisol: very thick, clay-rich B, low pH
        if b.is_some_and(|h| h.thickness_cm() > 100.0 && h.ph < 5.5) {
            return SoilOrder::Oxisol;
        }

        // Aridisol: very low organic matter, high pH
        if a.is_some_and(|h| h.organic_matter < 0.01 && h.ph > 7.5) {
            return SoilOrder::Aridisol;
        }

        // Ultisol: clay-rich B, acidic
        if b.is_some_and(|h| {
            matches!(
                h.texture,
                SoilTexture::Clay | SoilTexture::SandyClay | SoilTexture::SiltyClay
            ) && h.ph < 5.5
        }) {
            return SoilOrder::Ultisol;
        }

        // Alfisol: clay-rich B, moderate-high pH
        if b.is_some_and(|h| {
            matches!(
                h.texture,
                SoilTexture::Clay
                    | SoilTexture::ClayLoam
                    | SoilTexture::SandyClayLoam
                    | SoilTexture::SiltyClayLoam
            ) && h.ph >= 5.5
        }) {
            return SoilOrder::Alfisol;
        }

        // Entisol: no B horizon at all
        if !self.has_horizon(HorizonType::B) {
            return SoilOrder::Entisol;
        }

        // Inceptisol: weak B horizon development (default for young soils)
        SoilOrder::Inceptisol
    }
}

/// Soil pH classification.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_ph(6.8), SoilPhClass::Neutral);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SoilPhClass {
    UltraAcid,
    ExtremelyAcid,
    VeryStronglyAcid,
    StronglyAcid,
    ModeratelyAcid,
    SlightlyAcid,
    Neutral,
    SlightlyAlkaline,
    ModeratelyAlkaline,
    StronglyAlkaline,
    VeryStronglyAlkaline,
}

/// Classify soil pH into USDA categories.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert_eq!(classify_ph(4.0), SoilPhClass::ExtremelyAcid);
/// assert_eq!(classify_ph(7.0), SoilPhClass::Neutral);
/// assert_eq!(classify_ph(8.5), SoilPhClass::StronglyAlkaline);
/// ```
#[must_use]
pub fn classify_ph(ph: f32) -> SoilPhClass {
    if ph < 3.5 {
        SoilPhClass::UltraAcid
    } else if ph < 4.5 {
        SoilPhClass::ExtremelyAcid
    } else if ph < 5.0 {
        SoilPhClass::VeryStronglyAcid
    } else if ph < 5.5 {
        SoilPhClass::StronglyAcid
    } else if ph < 6.0 {
        SoilPhClass::ModeratelyAcid
    } else if ph < 6.5 {
        SoilPhClass::SlightlyAcid
    } else if ph < 7.3 {
        SoilPhClass::Neutral
    } else if ph < 7.8 {
        SoilPhClass::SlightlyAlkaline
    } else if ph < 8.4 {
        SoilPhClass::ModeratelyAlkaline
    } else if ph < 9.0 {
        SoilPhClass::StronglyAlkaline
    } else {
        SoilPhClass::VeryStronglyAlkaline
    }
}

/// Cation exchange capacity estimate from clay content and organic matter.
///
/// CEC = 0.5 x clay% + 2.0 x OM%
///
/// - `clay_fraction`: clay content as fraction (0.0-1.0)
/// - `organic_matter_fraction`: OM as fraction (0.0-1.0)
///
/// Returns CEC in meq/100g (milliequivalents per 100 grams).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cec = cation_exchange_capacity(0.30, 0.05);
/// assert!((cec - 25.0).abs() < 0.01);
/// ```
#[must_use]
pub fn cation_exchange_capacity(clay_fraction: f32, organic_matter_fraction: f32) -> f32 {
    0.5 * (clay_fraction * 100.0) + 2.0 * (organic_matter_fraction * 100.0)
}

/// Soil water holding capacity estimate.
///
/// Returns available water capacity in mm/m depth.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let awc = available_water_capacity(SoilTexture::Loam);
/// assert!((awc - 170.0).abs() < 0.01);
/// ```
#[must_use]
pub fn available_water_capacity(texture: SoilTexture) -> f32 {
    match texture {
        SoilTexture::Sand => 60.0,
        SoilTexture::LoamySand => 80.0,
        SoilTexture::SandyLoam => 120.0,
        SoilTexture::Loam => 170.0,
        SoilTexture::SiltLoam => 200.0,
        SoilTexture::Silt => 180.0,
        SoilTexture::SandyClayLoam => 140.0,
        SoilTexture::ClayLoam => 170.0,
        SoilTexture::SiltyClayLoam => 190.0,
        SoilTexture::SandyClay => 130.0,
        SoilTexture::SiltyClay => 160.0,
        SoilTexture::Clay => 150.0,
    }
}

/// Saturated hydraulic conductivity estimate from texture in mm/hr.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let k = hydraulic_conductivity_mm_hr(SoilTexture::Sand);
/// assert!((k - 210.0).abs() < 0.01);
/// ```
#[must_use]
pub fn hydraulic_conductivity_mm_hr(texture: SoilTexture) -> f32 {
    match texture {
        SoilTexture::Sand => 210.0,
        SoilTexture::LoamySand => 61.0,
        SoilTexture::SandyLoam => 26.0,
        SoilTexture::Loam => 13.0,
        SoilTexture::SiltLoam => 7.0,
        SoilTexture::Silt => 7.0,
        SoilTexture::SandyClayLoam => 4.0,
        SoilTexture::ClayLoam => 2.0,
        SoilTexture::SiltyClayLoam => 1.5,
        SoilTexture::SandyClay => 1.2,
        SoilTexture::SiltyClay => 0.9,
        SoilTexture::Clay => 0.6,
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

    // --- Soil taxonomy tests ---

    #[test]
    fn twelve_soil_orders() {
        assert_eq!(SoilOrder::ALL.len(), 12);
    }

    #[test]
    fn mollisol_high_fertility() {
        assert_eq!(SoilOrder::Mollisol.fertility(), SoilFertility::High);
    }

    #[test]
    fn oxisol_very_low_fertility() {
        assert_eq!(SoilOrder::Oxisol.fertility(), SoilFertility::VeryLow);
    }

    #[test]
    fn fertility_ordering() {
        assert!(SoilFertility::High > SoilFertility::VeryLow);
        assert!(SoilFertility::Moderate > SoilFertility::Low);
    }

    fn make_mollisol_profile() -> SoilProfile {
        SoilProfile {
            horizons: vec![
                SoilHorizon {
                    horizon_type: HorizonType::A,
                    depth_top_cm: 0.0,
                    depth_bottom_cm: 40.0,
                    organic_matter: 0.05,
                    ph: 6.5,
                    texture: SoilTexture::Loam,
                    color: "10YR 2/1".into(),
                },
                SoilHorizon {
                    horizon_type: HorizonType::B,
                    depth_top_cm: 40.0,
                    depth_bottom_cm: 100.0,
                    organic_matter: 0.01,
                    ph: 6.8,
                    texture: SoilTexture::ClayLoam,
                    color: "10YR 4/3".into(),
                },
                SoilHorizon {
                    horizon_type: HorizonType::C,
                    depth_top_cm: 100.0,
                    depth_bottom_cm: 150.0,
                    organic_matter: 0.002,
                    ph: 7.0,
                    texture: SoilTexture::SiltLoam,
                    color: "10YR 5/4".into(),
                },
            ],
            location: "Kansas prairie".into(),
        }
    }

    #[test]
    fn classify_mollisol() {
        let profile = make_mollisol_profile();
        assert_eq!(profile.classify_order(), SoilOrder::Mollisol);
    }

    #[test]
    fn classify_entisol_no_b_horizon() {
        let profile = SoilProfile {
            horizons: vec![SoilHorizon {
                horizon_type: HorizonType::A,
                depth_top_cm: 0.0,
                depth_bottom_cm: 20.0,
                organic_matter: 0.02,
                ph: 6.0,
                texture: SoilTexture::Sand,
                color: "10YR 5/3".into(),
            }],
            location: "River floodplain".into(),
        };
        assert_eq!(profile.classify_order(), SoilOrder::Entisol);
    }

    #[test]
    fn classify_histosol_high_om() {
        let profile = SoilProfile {
            horizons: vec![
                SoilHorizon {
                    horizon_type: HorizonType::O,
                    depth_top_cm: 0.0,
                    depth_bottom_cm: 10.0,
                    organic_matter: 0.50,
                    ph: 4.5,
                    texture: SoilTexture::Loam,
                    color: "5YR 2/1".into(),
                },
                SoilHorizon {
                    horizon_type: HorizonType::A,
                    depth_top_cm: 10.0,
                    depth_bottom_cm: 50.0,
                    organic_matter: 0.30,
                    ph: 4.5,
                    texture: SoilTexture::SiltLoam,
                    color: "10YR 2/1".into(),
                },
            ],
            location: "Bog".into(),
        };
        assert_eq!(profile.classify_order(), SoilOrder::Histosol);
    }

    #[test]
    fn profile_total_depth() {
        let profile = make_mollisol_profile();
        assert!((profile.total_depth_cm() - 150.0).abs() < 0.01);
    }

    #[test]
    fn horizon_thickness() {
        let h = SoilHorizon {
            horizon_type: HorizonType::A,
            depth_top_cm: 0.0,
            depth_bottom_cm: 30.0,
            organic_matter: 0.04,
            ph: 6.0,
            texture: SoilTexture::Loam,
            color: "dark".into(),
        };
        assert!((h.thickness_cm() - 30.0).abs() < 0.01);
    }

    #[test]
    fn ph_classification() {
        assert_eq!(classify_ph(4.0), SoilPhClass::ExtremelyAcid);
        assert_eq!(classify_ph(5.2), SoilPhClass::StronglyAcid);
        assert_eq!(classify_ph(6.8), SoilPhClass::Neutral);
        assert_eq!(classify_ph(7.5), SoilPhClass::SlightlyAlkaline);
        assert_eq!(classify_ph(8.5), SoilPhClass::StronglyAlkaline);
    }

    #[test]
    fn cec_increases_with_clay_and_om() {
        let low = cation_exchange_capacity(0.10, 0.01);
        let high = cation_exchange_capacity(0.50, 0.05);
        assert!(high > low);
    }

    #[test]
    fn awc_sand_lowest() {
        let sand_awc = available_water_capacity(SoilTexture::Sand);
        let loam_awc = available_water_capacity(SoilTexture::Loam);
        assert!(sand_awc < loam_awc);
    }

    #[test]
    fn ksat_sand_highest() {
        let sand_k = hydraulic_conductivity_mm_hr(SoilTexture::Sand);
        let clay_k = hydraulic_conductivity_mm_hr(SoilTexture::Clay);
        assert!(sand_k > clay_k);
    }

    #[test]
    fn topsoil_organic_matter_from_profile() {
        let profile = make_mollisol_profile();
        let om = profile.topsoil_organic_matter().unwrap();
        assert!((om - 0.05).abs() < 0.001);
    }
}
