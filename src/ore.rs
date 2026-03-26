use hisab::calc;
use serde::{Deserialize, Serialize};

/// Ore deposit type.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let dt = DepositType::Porphyry;
/// assert_eq!(dt, DepositType::Porphyry);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DepositType {
    Vein,
    Placer,
    Massive,
    Disseminated,
    Skarn,
    Porphyry,
}

/// Resource confidence classification (JORC/CIM/NI 43-101 style).
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(ResourceCategory::Inferred < ResourceCategory::Measured);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ResourceCategory {
    /// Lowest confidence — inferred from limited data.
    Inferred,
    /// Moderate confidence — reasonably estimated from adequate data.
    Indicated,
    /// Highest confidence — estimated with high level of confidence.
    Measured,
}

/// An ore deposit with validated fields.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let d = OreDeposit::new("Gold", DepositType::Vein, 0.01, 200.0, 50_000.0);
/// assert!(d.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OreDeposit {
    pub mineral: String,
    pub deposit_type: DepositType,
    pub grade: f32,   // fraction (0.0-1.0) of target mineral
    pub depth_m: f32, // positive depth in metres
    pub tonnage: f64, // metric tonnes (positive)
}

impl OreDeposit {
    /// Create a new ore deposit with validated fields.
    /// Returns `None` if grade is not in `[0.0, 1.0]`, depth is not positive,
    /// or tonnage is not positive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let valid = OreDeposit::new("Cu", DepositType::Porphyry, 0.02, 500.0, 1e5);
    /// assert!(valid.is_some());
    /// let invalid = OreDeposit::new("Cu", DepositType::Porphyry, -0.1, 500.0, 1e5);
    /// assert!(invalid.is_none());
    /// ```
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
        Some(Self {
            mineral: mineral.into(),
            deposit_type,
            grade,
            depth_m,
            tonnage,
        })
    }

    /// Contained metal in tonnes (grade × tonnage).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let d = OreDeposit::new("Cu", DepositType::Porphyry, 0.05, 100.0, 1_000_000.0).unwrap();
    /// assert!((d.contained_metal() - 50_000.0).abs() < 1.0);
    /// ```
    #[must_use]
    pub fn contained_metal(&self) -> f64 {
        self.grade as f64 * self.tonnage
    }

    /// Revenue estimate at a given metal price (contained metal × price).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let d = OreDeposit::new("Au", DepositType::Vein, 0.001, 200.0, 1e6).unwrap();
    /// let rev = d.gross_revenue(60_000_000.0);
    /// assert!(rev > 0.0);
    /// ```
    #[must_use]
    pub fn gross_revenue(&self, price_per_tonne: f64) -> f64 {
        self.contained_metal() * price_per_tonne
    }

    /// Stripping ratio estimate: depth-based proxy for open-pit vs underground.
    /// Returns approximate waste-to-ore ratio. Higher depth → higher ratio.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let d = OreDeposit::new("Cu", DepositType::Porphyry, 0.02, 500.0, 1e5).unwrap();
    /// assert!(d.stripping_ratio() > 1.0);
    /// ```
    #[must_use]
    pub fn stripping_ratio(&self) -> f64 {
        // Simple model: ratio increases linearly with depth
        // Shallow deposits (<50m) are ~2:1, deep (500m+) approach 10:1+
        (self.depth_m as f64 / 50.0).max(1.0)
    }
}

/// A tonnage-grade data point for building curves.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let pt = TonnageGradePoint {
///     cutoff_grade: 0.01,
///     tonnage_above_cutoff: 5000.0,
///     average_grade_above_cutoff: 0.025,
/// };
/// assert!(pt.tonnage_above_cutoff > 0.0);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TonnageGradePoint {
    pub cutoff_grade: f64,
    pub tonnage_above_cutoff: f64,
    pub average_grade_above_cutoff: f64,
}

/// Build a tonnage-grade curve from a set of block grades.
///
/// Given a list of `(tonnage, grade)` blocks, computes at each cutoff grade
/// the total tonnage and average grade above that cutoff.
///
/// - `blocks`: slice of `(tonnage, grade)` pairs
/// - `cutoff_steps`: number of cutoff grades to evaluate between min and max grade
///
/// Returns a sorted vector of [`TonnageGradePoint`]s from lowest to highest cutoff.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let blocks = vec![(1000.0, 0.01), (2000.0, 0.02), (500.0, 0.05)];
/// let curve = tonnage_grade_curve(&blocks, 4);
/// assert!(!curve.is_empty());
/// assert!(curve[0].tonnage_above_cutoff >= curve.last().unwrap().tonnage_above_cutoff);
/// ```
#[must_use]
pub fn tonnage_grade_curve(blocks: &[(f64, f64)], cutoff_steps: usize) -> Vec<TonnageGradePoint> {
    if blocks.is_empty() || cutoff_steps == 0 {
        return Vec::new();
    }

    let min_grade = blocks.iter().map(|b| b.1).fold(f64::INFINITY, f64::min);
    let max_grade = blocks.iter().map(|b| b.1).fold(f64::NEG_INFINITY, f64::max);

    if (max_grade - min_grade).abs() < f64::EPSILON {
        let total_tonnage: f64 = blocks.iter().map(|b| b.0).sum();
        return vec![TonnageGradePoint {
            cutoff_grade: min_grade,
            tonnage_above_cutoff: total_tonnage,
            average_grade_above_cutoff: min_grade,
        }];
    }

    let step = (max_grade - min_grade) / cutoff_steps as f64;
    (0..=cutoff_steps)
        .map(|i| {
            let cutoff = min_grade + step * i as f64;
            let mut total_tonnage = 0.0;
            let mut weighted_grade = 0.0;
            for &(t, g) in blocks {
                if g >= cutoff {
                    total_tonnage += t;
                    weighted_grade += t * g;
                }
            }
            let avg = if total_tonnage > 0.0 {
                weighted_grade / total_tonnage
            } else {
                0.0
            };
            TonnageGradePoint {
                cutoff_grade: cutoff,
                tonnage_above_cutoff: total_tonnage,
                average_grade_above_cutoff: avg,
            }
        })
        .collect()
}

/// Cutoff grade: the minimum grade at which mining is profitable.
///
/// Solves for the grade where `grade × price × recovery = cost_per_tonne`.
///
/// - `price_per_tonne`: metal price per tonne of pure metal
/// - `cost_per_tonne`: total mining + processing cost per tonne of ore
/// - `recovery`: metallurgical recovery fraction (0.0-1.0, typically 0.7-0.95)
///
/// Returns the cutoff grade as a fraction.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let cog = cutoff_grade(60_000_000.0, 50.0, 0.90).unwrap();
/// assert!(cog > 0.0 && cog < 0.001);
/// ```
#[must_use]
pub fn cutoff_grade(price_per_tonne: f64, cost_per_tonne: f64, recovery: f64) -> Option<f64> {
    if price_per_tonne <= 0.0 || recovery <= 0.0 || recovery > 1.0 {
        return None;
    }
    let cog = cost_per_tonne / (price_per_tonne * recovery);
    if cog > 1.0 {
        None // uneconomic at any grade
    } else {
        Some(cog)
    }
}

/// Economic viability check using hisab numerical integration to model
/// decreasing marginal return over the deposit's tonnage.
///
/// Integrates `grade * price_per_tonne` over the tonnage range and compares
/// against extraction cost.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// assert!(is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0));
/// ```
#[must_use]
pub fn is_economically_viable(
    grade: f32,
    tonnage: f64,
    price_per_tonne: f64,
    extraction_cost: f64,
) -> bool {
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

/// Net present value of a deposit mined over `years` at a given discount rate.
///
/// - `annual_revenue`: expected yearly revenue
/// - `annual_cost`: expected yearly operating cost
/// - `discount_rate`: annual discount rate (e.g., 0.08 for 8%)
/// - `years`: mine life in years
///
/// Uses hisab integration over continuous discounting.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let npv = net_present_value(10_000_000.0, 7_000_000.0, 0.08, 10.0).unwrap();
/// assert!(npv > 0.0);
/// ```
#[must_use]
pub fn net_present_value(
    annual_revenue: f64,
    annual_cost: f64,
    discount_rate: f64,
    years: f64,
) -> Option<f64> {
    if discount_rate < 0.0 || years <= 0.0 {
        return None;
    }
    let net_annual = annual_revenue - annual_cost;
    // NPV = ∫₀ᵗ net_annual · e^(-r·t) dt
    calc::integral_simpson(|t| net_annual * (-discount_rate * t).exp(), 0.0, years, 50).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_grade_viable() {
        assert!(is_economically_viable(
            0.05,
            1_000_000.0,
            5000.0,
            100_000_000.0
        ));
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

    #[test]
    fn contained_metal() {
        let d = OreDeposit::new("Cu", DepositType::Porphyry, 0.05, 100.0, 1_000_000.0).unwrap();
        assert!((d.contained_metal() - 50_000.0).abs() < 1.0);
    }

    #[test]
    fn cutoff_grade_gold() {
        // Gold at $60M/t, mining cost $50/t ore, 90% recovery
        let cog = cutoff_grade(60_000_000.0, 50.0, 0.90).unwrap();
        // cog = 50 / (60_000_000 * 0.9) ≈ 9.26e-7
        assert!(cog > 0.0 && cog < 0.001);
    }

    #[test]
    fn cutoff_grade_uneconomic() {
        // Price too low to cover costs at any grade
        assert!(cutoff_grade(10.0, 100.0, 0.90).is_none());
    }

    #[test]
    fn tonnage_grade_curve_basic() {
        let blocks = vec![
            (1000.0, 0.01),
            (2000.0, 0.02),
            (1500.0, 0.03),
            (500.0, 0.05),
        ];
        let curve = tonnage_grade_curve(&blocks, 4);
        assert_eq!(curve.len(), 5); // 0..=4 cutoff steps
        // At lowest cutoff, all tonnage included
        assert!((curve[0].tonnage_above_cutoff - 5000.0).abs() < 1.0);
        // Tonnage decreases as cutoff increases
        for pair in curve.windows(2) {
            assert!(pair[1].tonnage_above_cutoff <= pair[0].tonnage_above_cutoff);
        }
        // Average grade above cutoff increases as cutoff increases
        for pair in curve.windows(2) {
            if pair[1].tonnage_above_cutoff > 0.0 {
                assert!(pair[1].average_grade_above_cutoff >= pair[0].average_grade_above_cutoff);
            }
        }
    }

    #[test]
    fn tonnage_grade_empty() {
        assert!(tonnage_grade_curve(&[], 10).is_empty());
    }

    #[test]
    fn npv_positive_project() {
        let npv = net_present_value(10_000_000.0, 7_000_000.0, 0.08, 10.0).unwrap();
        assert!(npv > 0.0); // net $3M/yr for 10 years at 8% discount
    }

    #[test]
    fn npv_negative_project() {
        let npv = net_present_value(5_000_000.0, 8_000_000.0, 0.10, 5.0).unwrap();
        assert!(npv < 0.0); // losing $3M/yr
    }

    #[test]
    fn resource_category_ordering() {
        assert!(ResourceCategory::Inferred < ResourceCategory::Indicated);
        assert!(ResourceCategory::Indicated < ResourceCategory::Measured);
    }

    #[test]
    fn stripping_ratio_increases_with_depth() {
        let shallow = OreDeposit::new("Cu", DepositType::Porphyry, 0.02, 50.0, 100_000.0).unwrap();
        let deep = OreDeposit::new("Cu", DepositType::Porphyry, 0.02, 500.0, 100_000.0).unwrap();
        assert!(deep.stripping_ratio() > shallow.stripping_ratio());
    }

    #[test]
    fn gross_revenue() {
        let d = OreDeposit::new("Au", DepositType::Vein, 0.001, 200.0, 1_000_000.0).unwrap();
        let rev = d.gross_revenue(60_000_000.0); // gold ~$60M/t
        // 1000t contained * $60M = $60B
        assert!((rev - 60_000_000_000.0).abs() < 1_000_000.0);
    }
}
