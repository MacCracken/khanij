//! Sediment budget model — source-to-sink mass balance combining weathering
//! production, transport capacity, and deposition.

use serde::{Deserialize, Serialize};

/// A sediment source: where sediment is produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SedimentSource {
    /// Name/label for this source.
    pub name: String,
    /// Weathering rate producing sediment (tonnes/year).
    pub production_rate: f64,
    /// Grain size distribution: fraction in each size class.
    /// `[clay, silt, fine_sand, coarse_sand, gravel]` summing to 1.0.
    pub grain_fractions: [f64; 5],
}

/// A sediment sink: where sediment accumulates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SedimentSink {
    pub name: String,
    /// Maximum deposition rate capacity (tonnes/year).
    pub capacity: f64,
    /// Current accumulated sediment (tonnes).
    pub accumulated: f64,
}

/// Sediment budget result for a time step.
#[derive(Debug, Clone)]
pub struct BudgetResult {
    /// Total sediment produced (tonnes).
    pub total_production: f64,
    /// Total sediment deposited (tonnes).
    pub total_deposition: f64,
    /// Sediment exported out of the system (tonnes).
    pub total_export: f64,
    /// Net change: production - deposition - export.
    pub net_change: f64,
}

/// Grain size class names for reference.
pub const GRAIN_CLASSES: [&str; 5] = ["clay", "silt", "fine_sand", "coarse_sand", "gravel"];

/// Representative grain diameters (metres) for each size class.
pub const GRAIN_DIAMETERS: [f64; 5] = [0.000_005, 0.000_03, 0.000_2, 0.001, 0.01];

/// Compute sediment production from weathering rate and area.
///
/// - `weathering_rate`: relative rate (0-1) from `physical_weathering_rate` or
///   `chemical_weathering_rate`
/// - `rock_density`: kg/m³
/// - `area_m2`: weathering surface area in m²
/// - `depth_m_per_year`: average weathering depth per year in metres
///
/// Returns sediment production in tonnes/year.
#[must_use]
pub fn sediment_production(
    weathering_rate: f64,
    rock_density: f64,
    area_m2: f64,
    depth_m_per_year: f64,
) -> f64 {
    weathering_rate * rock_density * area_m2 * depth_m_per_year / 1000.0 // kg → tonnes
}

/// Transport capacity of a river reach using a simplified stream power approach.
///
/// Q_s = k · Q_w^a · S^b
///
/// - `discharge_m3_s`: water discharge in m³/s
/// - `slope`: channel slope (dimensionless, e.g., 0.01 = 1%)
/// - `k`: transport coefficient (typical: 0.001-0.01)
/// - `a`: discharge exponent (typical: 1.5)
/// - `b`: slope exponent (typical: 1.0)
///
/// Returns transport capacity in tonnes/year.
#[must_use]
pub fn transport_capacity(discharge_m3_s: f64, slope: f64, k: f64, a: f64, b: f64) -> f64 {
    let rate_kg_s = k * discharge_m3_s.powf(a) * slope.powf(b);
    rate_kg_s * 365.25 * 86400.0 / 1000.0 // kg/s → tonnes/year
}

/// Compute a simple sediment budget for one time step.
///
/// - `sources`: all sediment sources with production rates
/// - `transport_cap`: maximum transport capacity (tonnes/year)
/// - `sinks`: deposition sinks with capacities
///
/// Returns the budget breakdown.
#[must_use]
pub fn compute_budget(
    sources: &[SedimentSource],
    transport_cap: f64,
    sinks: &[SedimentSink],
) -> BudgetResult {
    let total_production: f64 = sources.iter().map(|s| s.production_rate).sum();
    let transportable = total_production.min(transport_cap);
    let total_sink_capacity: f64 = sinks.iter().map(|s| s.capacity).sum();
    let total_deposition = transportable.min(total_sink_capacity);
    let total_export = transportable - total_deposition;

    BudgetResult {
        total_production,
        total_deposition,
        total_export,
        net_change: total_production - total_deposition - total_export,
    }
}

/// Sediment delivery ratio: fraction of eroded sediment that reaches a sink.
///
/// SDR decreases with catchment area (larger catchments store more sediment).
///
/// SDR ≈ 0.42 · A^(-0.125) (Vanoni, 1975)
///
/// - `catchment_area_km2`: catchment area in km²
///
/// Returns SDR as a fraction (0.0-1.0).
#[must_use]
pub fn sediment_delivery_ratio(catchment_area_km2: f64) -> f64 {
    if catchment_area_km2 <= 0.0 {
        return 1.0;
    }
    (0.42 * catchment_area_km2.powf(-0.125)).clamp(0.0, 1.0)
}

/// Denudation rate: average surface lowering in mm/year from sediment export.
///
/// - `sediment_export_tonnes_yr`: annual sediment export in tonnes
/// - `catchment_area_m2`: catchment area in m²
/// - `rock_density`: kg/m³
///
/// Returns denudation rate in mm/year.
#[must_use]
pub fn denudation_rate(
    sediment_export_tonnes_yr: f64,
    catchment_area_m2: f64,
    rock_density: f64,
) -> f64 {
    if catchment_area_m2 <= 0.0 || rock_density <= 0.0 {
        return 0.0;
    }
    // tonnes → kg → m³ → m depth → mm
    let volume_m3 = sediment_export_tonnes_yr * 1000.0 / rock_density;
    let depth_m = volume_m3 / catchment_area_m2;
    depth_m * 1000.0 // m → mm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sediment_production_positive() {
        let prod = sediment_production(0.5, 2700.0, 1e6, 0.001);
        assert!(prod > 0.0);
    }

    #[test]
    fn zero_weathering_zero_production() {
        let prod = sediment_production(0.0, 2700.0, 1e6, 0.001);
        assert!(prod.abs() < f64::EPSILON);
    }

    #[test]
    fn transport_capacity_increases_with_discharge() {
        let low = transport_capacity(1.0, 0.01, 0.005, 1.5, 1.0);
        let high = transport_capacity(10.0, 0.01, 0.005, 1.5, 1.0);
        assert!(high > low);
    }

    #[test]
    fn transport_capacity_increases_with_slope() {
        let flat = transport_capacity(5.0, 0.001, 0.005, 1.5, 1.0);
        let steep = transport_capacity(5.0, 0.05, 0.005, 1.5, 1.0);
        assert!(steep > flat);
    }

    #[test]
    fn budget_production_equals_deposition_plus_export() {
        let sources = vec![SedimentSource {
            name: "Hillslope".into(),
            production_rate: 1000.0,
            grain_fractions: [0.2, 0.3, 0.3, 0.15, 0.05],
        }];
        let sinks = vec![SedimentSink {
            name: "Floodplain".into(),
            capacity: 600.0,
            accumulated: 0.0,
        }];
        let result = compute_budget(&sources, 1500.0, &sinks);
        assert!((result.total_production - 1000.0).abs() < 0.01);
        assert!((result.total_deposition - 600.0).abs() < 0.01);
        assert!((result.total_export - 400.0).abs() < 0.01);
    }

    #[test]
    fn transport_limited_budget() {
        let sources = vec![SedimentSource {
            name: "Source".into(),
            production_rate: 1000.0,
            grain_fractions: [0.2, 0.2, 0.2, 0.2, 0.2],
        }];
        let sinks = vec![SedimentSink {
            name: "Sink".into(),
            capacity: 5000.0,
            accumulated: 0.0,
        }];
        // Transport capacity limits to 500
        let result = compute_budget(&sources, 500.0, &sinks);
        assert!((result.total_deposition - 500.0).abs() < 0.01);
        assert!(result.total_export.abs() < 0.01);
    }

    #[test]
    fn sdr_decreases_with_area() {
        let small = sediment_delivery_ratio(1.0);
        let large = sediment_delivery_ratio(1000.0);
        assert!(small > large);
    }

    #[test]
    fn sdr_bounded() {
        assert!(sediment_delivery_ratio(0.1) <= 1.0);
        assert!(sediment_delivery_ratio(100_000.0) > 0.0);
    }

    #[test]
    fn denudation_rate_positive() {
        let d = denudation_rate(10_000.0, 1e9, 2700.0);
        assert!(d > 0.0);
    }

    #[test]
    fn zero_export_zero_denudation() {
        let d = denudation_rate(0.0, 1e9, 2700.0);
        assert!(d.abs() < f64::EPSILON);
    }
}
