use hisab::calc;
use serde::{Deserialize, Serialize};

/// Ore deposit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DepositType { Vein, Placer, Massive, Disseminated, Skarn, Porphyry }

/// An ore deposit with validated fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OreDeposit {
    pub mineral: String,
    pub deposit_type: DepositType,
    pub grade: f32,       // fraction (0.0-1.0) of target mineral
    pub depth_m: f32,     // positive depth in metres
    pub tonnage: f64,     // metric tonnes (positive)
}

impl OreDeposit {
    /// Create a new ore deposit with validated fields.
    /// Returns `None` if grade is not in `[0.0, 1.0]`, depth is not positive,
    /// or tonnage is not positive.
    #[must_use]
    pub fn new(
        mineral: impl Into<String>,
        deposit_type: DepositType,
        grade: f32,
        depth_m: f32,
        tonnage: f64,
    ) -> Option<Self> {
        if !(0.0..=1.0).contains(&grade) || depth_m <= 0.0 || tonnage <= 0.0 {
            return None;
        }
        Some(Self { mineral: mineral.into(), deposit_type, grade, depth_m, tonnage })
    }
}

/// Economic viability check using hisab numerical integration to model
/// decreasing marginal return over the deposit's tonnage.
///
/// Integrates `grade * price_per_tonne` over the tonnage range and compares
/// against extraction cost.
#[must_use]
pub fn is_economically_viable(grade: f32, tonnage: f64, price_per_tonne: f64, extraction_cost: f64) -> bool {
    let g = grade as f64;
    // Integrate revenue over tonnage with a diminishing-return factor:
    // as you mine more, later tonnes are slightly harder to extract.
    let revenue = calc::integral_simpson(
        |t| g * price_per_tonne * (-0.0001 * t / tonnage).exp(),
        0.0,
        tonnage,
        50,
    )
    .unwrap_or(0.0);
    revenue > extraction_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_grade_viable() {
        assert!(is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0));
    }

    #[test]
    fn low_grade_not_viable() {
        assert!(!is_economically_viable(0.001, 1000.0, 100.0, 1_000_000.0));
    }

    #[test]
    fn deposit_serde() {
        let d = OreDeposit::new("Gold", DepositType::Vein, 0.01, 200.0, 50_000.0).unwrap();
        let json = serde_json::to_string(&d).unwrap();
        let back: OreDeposit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.mineral, "Gold");
    }

    #[test]
    fn validated_deposit_rejects_invalid_grade() {
        assert!(OreDeposit::new("Iron", DepositType::Massive, -0.1, 100.0, 1000.0).is_none());
        assert!(OreDeposit::new("Iron", DepositType::Massive, 1.5, 100.0, 1000.0).is_none());
    }

    #[test]
    fn validated_deposit_rejects_invalid_depth() {
        assert!(OreDeposit::new("Iron", DepositType::Massive, 0.5, -10.0, 1000.0).is_none());
        assert!(OreDeposit::new("Iron", DepositType::Massive, 0.5, 0.0, 1000.0).is_none());
    }

    #[test]
    fn validated_deposit_rejects_invalid_tonnage() {
        assert!(OreDeposit::new("Iron", DepositType::Massive, 0.5, 100.0, -500.0).is_none());
        assert!(OreDeposit::new("Iron", DepositType::Massive, 0.5, 100.0, 0.0).is_none());
    }

    #[test]
    fn valid_deposit_created() {
        let d = OreDeposit::new("Copper", DepositType::Porphyry, 0.02, 500.0, 100_000.0);
        assert!(d.is_some());
    }
}
