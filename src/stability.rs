//! Mineral stability — thermodynamic assessment of mineral phases using Gibbs
//! free energy data from kimiya.
//!
//! Requires the `chemistry` feature.

/// Gibbs free energy of formation for a mineral formula at standard conditions
/// (298.15 K, 1 atm) from kimiya's thermochemistry database.
///
/// Returns ΔG°_f in kJ/mol, or `None` if the formula is not in the database.
#[must_use]
pub fn gibbs_formation(formula: &str) -> Option<f64> {
    kimiya::lookup_thermochem(formula).map(|d| d.delta_gf_kj)
}

/// Enthalpy of formation for a mineral at standard conditions.
///
/// Returns ΔH°_f in kJ/mol.
#[must_use]
pub fn enthalpy_formation(formula: &str) -> Option<f64> {
    kimiya::lookup_thermochem(formula).map(|d| d.delta_hf_kj)
}

/// Standard molar entropy for a mineral.
///
/// Returns S° in J/(mol·K).
#[must_use]
pub fn standard_entropy(formula: &str) -> Option<f64> {
    kimiya::lookup_thermochem(formula).map(|d| d.s_standard_j)
}

/// Gibbs energy change for a reaction at a given temperature.
///
/// ΔG(T) ≈ ΔH° - T·ΔS° (assumes ΔH and ΔS are roughly constant with T).
///
/// - `delta_h_kj`: enthalpy change of reaction in kJ/mol
/// - `delta_s_j_per_k`: entropy change of reaction in J/(mol·K)
/// - `temperature_k`: temperature in kelvin
///
/// Returns ΔG in kJ/mol.
#[must_use]
pub fn gibbs_at_temperature(delta_h_kj: f64, delta_s_j_per_k: f64, temperature_k: f64) -> f64 {
    delta_h_kj - temperature_k * delta_s_j_per_k / 1000.0
}

/// Determine which of two polymorphs is stable at a given temperature.
///
/// Compares two mineral phases by their ΔG(T) = ΔH°_f - T·ΔS°. The phase
/// with the more negative (lower) Gibbs energy is the stable one.
///
/// Returns the formula of the stable phase, or `None` if either phase is
/// not in the thermochemistry database.
#[must_use]
pub fn stable_polymorph<'a>(
    phase_a: &'a str,
    phase_b: &'a str,
    temperature_k: f64,
) -> Option<&'a str> {
    let a = kimiya::lookup_thermochem(phase_a)?;
    let b = kimiya::lookup_thermochem(phase_b)?;

    let g_a = gibbs_at_temperature(a.delta_hf_kj, a.s_standard_j, temperature_k);
    let g_b = gibbs_at_temperature(b.delta_hf_kj, b.s_standard_j, temperature_k);

    if g_a <= g_b {
        Some(phase_a)
    } else {
        Some(phase_b)
    }
}

/// Reaction Gibbs energy from products and reactants at standard conditions.
///
/// ΔG°_rxn = Σ(ΔG°_f products) - Σ(ΔG°_f reactants)
///
/// Each entry is `(formula, stoichiometric_coefficient)`.
/// Returns ΔG° in kJ/mol, or `None` if any formula is missing from the database.
#[must_use]
pub fn reaction_gibbs(products: &[(&str, f64)], reactants: &[(&str, f64)]) -> Option<f64> {
    let sum_products: f64 = products
        .iter()
        .map(|(f, n)| gibbs_formation(f).map(|g| n * g))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();

    let sum_reactants: f64 = reactants
        .iter()
        .map(|(f, n)| gibbs_formation(f).map(|g| n * g))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();

    Some(sum_products - sum_reactants)
}

/// Check if a mineral reaction is spontaneous at standard conditions.
///
/// Returns `true` when ΔG°_rxn < 0.
#[must_use]
pub fn is_reaction_spontaneous(
    products: &[(&str, f64)],
    reactants: &[(&str, f64)],
) -> Option<bool> {
    reaction_gibbs(products, reactants).map(|dg| dg < 0.0)
}

/// Equilibrium temperature for a reaction (where ΔG = 0).
///
/// T_eq = ΔH° / ΔS° (in kelvin). This is the temperature at which the
/// reaction switches from non-spontaneous to spontaneous (or vice versa).
///
/// Returns `None` if ΔS° ≈ 0 (no temperature dependence) or if any formula
/// is missing.
#[must_use]
pub fn equilibrium_temperature(products: &[(&str, f64)], reactants: &[(&str, f64)]) -> Option<f64> {
    let dh = reaction_enthalpy(products, reactants)?;
    let ds = reaction_entropy(products, reactants)?;

    if ds.abs() < 1e-10 {
        return None; // no crossing point
    }

    let t_eq = (dh * 1000.0) / ds; // convert kJ to J to match S in J/(mol·K)
    if t_eq > 0.0 { Some(t_eq) } else { None }
}

/// Reaction enthalpy from products and reactants.
/// ΔH°_rxn = Σ(ΔH°_f products) - Σ(ΔH°_f reactants)
#[must_use]
pub fn reaction_enthalpy(products: &[(&str, f64)], reactants: &[(&str, f64)]) -> Option<f64> {
    let sum_p: f64 = products
        .iter()
        .map(|(f, n)| enthalpy_formation(f).map(|h| n * h))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();
    let sum_r: f64 = reactants
        .iter()
        .map(|(f, n)| enthalpy_formation(f).map(|h| n * h))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();
    Some(sum_p - sum_r)
}

/// Reaction entropy from products and reactants.
/// ΔS°_rxn = Σ(S° products) - Σ(S° reactants)
#[must_use]
pub fn reaction_entropy(products: &[(&str, f64)], reactants: &[(&str, f64)]) -> Option<f64> {
    let sum_p: f64 = products
        .iter()
        .map(|(f, n)| standard_entropy(f).map(|s| n * s))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();
    let sum_r: f64 = reactants
        .iter()
        .map(|(f, n)| standard_entropy(f).map(|s| n * s))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();
    Some(sum_p - sum_r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quartz_gibbs_negative() {
        // SiO2 is thermodynamically stable — should have negative ΔG°_f
        let g = gibbs_formation("SiO2(s)").unwrap();
        assert!(g < 0.0, "SiO₂ should have negative ΔG°_f, got {g}");
    }

    #[test]
    fn calcite_gibbs_negative() {
        let g = gibbs_formation("CaCO3(s)").unwrap();
        assert!(g < 0.0);
    }

    #[test]
    fn unknown_formula_returns_none() {
        assert!(gibbs_formation("unobtanium").is_none());
    }

    #[test]
    fn gibbs_at_temperature_standard() {
        // At 298.15K with ΔH=-100 kJ and ΔS=50 J/K:
        // ΔG = -100 - 298.15 * 50/1000 = -100 - 14.9075 = -114.9075
        let g = gibbs_at_temperature(-100.0, 50.0, 298.15);
        assert!((g - (-114.9075)).abs() < 0.01);
    }

    #[test]
    fn calcite_decomposition_not_spontaneous_at_room_temp() {
        // CaCO3(s) → CaO(s) + CO2(g)
        // This requires ~900°C, so should NOT be spontaneous at 298K
        let spontaneous =
            is_reaction_spontaneous(&[("CaO(s)", 1.0), ("CO2(g)", 1.0)], &[("CaCO3(s)", 1.0)]);
        assert_eq!(spontaneous, Some(false));
    }

    #[test]
    fn calcite_decomposition_equilibrium_temperature() {
        // CaCO3 → CaO + CO2 equilibrium ~1100K
        let t_eq =
            equilibrium_temperature(&[("CaO(s)", 1.0), ("CO2(g)", 1.0)], &[("CaCO3(s)", 1.0)]);
        assert!(t_eq.is_some());
        let t = t_eq.unwrap();
        // Should be around 1100K (literature: ~1115K)
        assert!(
            t > 800.0 && t < 1400.0,
            "CaCO₃ decomposition T_eq should be ~1100K, got {t}"
        );
    }

    #[test]
    fn iron_oxide_formation_spontaneous() {
        // 2Fe(s) + 3/2 O2(g) → Fe2O3(s) — rusting is spontaneous
        let spontaneous =
            is_reaction_spontaneous(&[("Fe2O3(s)", 1.0)], &[("Fe(s)", 2.0), ("O2(g)", 1.5)]);
        assert_eq!(spontaneous, Some(true));
    }

    #[test]
    fn reaction_gibbs_with_missing_formula() {
        let result = reaction_gibbs(&[("unobtanium", 1.0)], &[("SiO2(s)", 1.0)]);
        assert!(result.is_none());
    }
}
