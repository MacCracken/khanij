use serde::{Deserialize, Serialize};

/// Ore deposit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DepositType { Vein, Placer, Massive, Disseminated, Skarn, Porphyry }

/// An ore deposit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OreDeposit {
    pub mineral: String,
    pub deposit_type: DepositType,
    pub grade: f32,       // fraction (0.0-1.0) of target mineral
    pub depth_m: f32,
    pub tonnage: f64,     // metric tonnes
}

/// Economic viability check (simplified: grade × tonnage × price > extraction cost).
#[must_use]
pub fn is_economically_viable(grade: f32, tonnage: f64, price_per_tonne: f64, extraction_cost: f64) -> bool {
    (grade as f64) * tonnage * price_per_tonne > extraction_cost
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
        let d = OreDeposit { mineral: "Gold".into(), deposit_type: DepositType::Vein, grade: 0.01, depth_m: 200.0, tonnage: 50_000.0 };
        let json = serde_json::to_string(&d).unwrap();
        let back: OreDeposit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.mineral, "Gold");
    }
}
