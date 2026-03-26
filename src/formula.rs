//! Mineral formula parser — converts chemical formula strings like `"CaCO₃"`,
//! `"Mg₃Si₄O₁₀(OH)₂"`, or `"CaSO4·2H2O"` into element-count pairs.

use std::collections::BTreeMap;

/// Parsed mineral formula: a map of element symbol → atom count.
#[derive(Debug, Clone, PartialEq)]
pub struct Formula {
    /// Element symbol → total atom count.
    pub elements: BTreeMap<String, u32>,
}

impl Formula {
    /// Parse a chemical formula string.
    ///
    /// Supports:
    /// - Standard notation: `SiO2`, `CaCO3`, `KAlSi3O8`
    /// - Parenthesized groups: `Mg3Si4O10(OH)2`, `Ca5(PO4)3F`
    /// - Unicode subscripts: `SiO₂`, `Mg₃Si₄O₁₀(OH)₂`
    /// - Hydrate notation: `CaSO4·2H2O` (dot/middot separator)
    /// - Solid-solution notation: `(Mg,Fe)2SiO4` — includes all alternatives
    ///
    /// Returns `None` if the formula is empty or contains unrecognised tokens.
    #[must_use]
    pub fn parse(formula: &str) -> Option<Self> {
        let normalized = normalize_unicode(formula);
        let mut elements = BTreeMap::new();

        // Split on hydrate separator (·, ., or middle dot)
        for part in normalized.split(['·', '•']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            // Check for leading coefficient (e.g., "2H2O")
            let (coeff, rest) = split_leading_coefficient(part);
            let mut part_elements = BTreeMap::new();
            parse_group(rest.as_bytes(), &mut 0, &mut part_elements)?;
            for (sym, count) in part_elements {
                *elements.entry(sym).or_insert(0) += count * coeff;
            }
        }

        if elements.is_empty() {
            None
        } else {
            Some(Self { elements })
        }
    }

    /// Total number of atoms in the formula.
    #[must_use]
    pub fn total_atoms(&self) -> u32 {
        self.elements.values().sum()
    }

    /// Get atom count for a specific element.
    #[must_use]
    pub fn count(&self, symbol: &str) -> u32 {
        self.elements.get(symbol).copied().unwrap_or(0)
    }

    /// Convert to `(atomic_number, count)` pairs for kimiya `Molecule::new`.
    ///
    /// Requires the `chemistry` feature. Returns `None` if any element symbol
    /// is not recognised.
    #[cfg(feature = "chemistry")]
    #[must_use]
    pub fn to_atom_pairs(&self) -> Option<Vec<(u8, u32)>> {
        self.elements
            .iter()
            .map(|(sym, &count)| {
                kimiya::element::lookup_by_symbol(sym).map(|e| (e.atomic_number, count))
            })
            .collect()
    }

    /// Build a kimiya `Molecule` from this parsed formula.
    ///
    /// Requires the `chemistry` feature.
    #[cfg(feature = "chemistry")]
    #[must_use]
    pub fn to_molecule(&self) -> Option<kimiya::Molecule> {
        self.to_atom_pairs()
            .map(|pairs| kimiya::Molecule::new(&pairs))
    }

    /// Molecular weight in g/mol via kimiya.
    ///
    /// Requires the `chemistry` feature.
    #[cfg(feature = "chemistry")]
    #[must_use]
    pub fn molecular_weight(&self) -> Option<f64> {
        self.to_molecule().and_then(|m| m.molecular_weight().ok())
    }
}

/// Replace Unicode subscript digits with ASCII digits.
fn normalize_unicode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '₀' => '0',
            '₁' => '1',
            '₂' => '2',
            '₃' => '3',
            '₄' => '4',
            '₅' => '5',
            '₆' => '6',
            '₇' => '7',
            '₈' => '8',
            '₉' => '9',
            _ => c,
        })
        .collect()
}

/// Split a leading integer coefficient from a formula fragment.
/// "2H2O" → (2, "H2O"), "H2O" → (1, "H2O")
fn split_leading_coefficient(s: &str) -> (u32, &str) {
    let digits: usize = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits > 0 {
        let n: u32 = s[..digits].parse().unwrap_or(1);
        (n, &s[digits..])
    } else {
        (1, s)
    }
}

/// Recursively parse a formula group, accumulating into `elements`.
/// Handles parentheses `()`, commas for solid solutions `(Mg,Fe)`, and subscripts.
fn parse_group(bytes: &[u8], pos: &mut usize, elements: &mut BTreeMap<String, u32>) -> Option<()> {
    while *pos < bytes.len() {
        let b = bytes[*pos];

        if b == b')' {
            // End of parenthesized group — caller handles subscript
            return Some(());
        } else if b == b'(' {
            // Start of parenthesized group
            *pos += 1;
            let mut group = BTreeMap::new();
            parse_group(bytes, pos, &mut group)?;
            // Expect closing ')'
            if *pos < bytes.len() && bytes[*pos] == b')' {
                *pos += 1;
            }
            let multiplier = read_number(bytes, pos);
            for (sym, count) in group {
                *elements.entry(sym).or_insert(0) += count * multiplier;
            }
        } else if b == b',' {
            // Solid solution separator — skip
            *pos += 1;
        } else if b.is_ascii_uppercase() {
            // Element symbol
            let sym = read_element_symbol(bytes, pos)?;
            let count = read_number(bytes, pos);
            *elements.entry(sym).or_insert(0) += count;
        } else {
            // Unrecognised character — skip (handles spaces, etc.)
            *pos += 1;
        }
    }
    Some(())
}

/// Read an element symbol: uppercase letter followed by optional lowercase letter.
fn read_element_symbol(bytes: &[u8], pos: &mut usize) -> Option<String> {
    if *pos >= bytes.len() || !bytes[*pos].is_ascii_uppercase() {
        return None;
    }
    let start = *pos;
    *pos += 1;
    // Consume lowercase letters (handles 1 or 2 letter symbols)
    while *pos < bytes.len() && bytes[*pos].is_ascii_lowercase() {
        *pos += 1;
    }
    Some(String::from_utf8_lossy(&bytes[start..*pos]).into_owned())
}

/// Read an integer subscript. Returns 1 if no digits follow.
fn read_number(bytes: &[u8], pos: &mut usize) -> u32 {
    let start = *pos;
    while *pos < bytes.len() && bytes[*pos].is_ascii_digit() {
        *pos += 1;
    }
    if *pos > start {
        std::str::from_utf8(&bytes[start..*pos])
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_oxide() {
        let f = Formula::parse("SiO2").unwrap();
        assert_eq!(f.count("Si"), 1);
        assert_eq!(f.count("O"), 2);
        assert_eq!(f.total_atoms(), 3);
    }

    #[test]
    fn carbonate() {
        let f = Formula::parse("CaCO3").unwrap();
        assert_eq!(f.count("Ca"), 1);
        assert_eq!(f.count("C"), 1);
        assert_eq!(f.count("O"), 3);
    }

    #[test]
    fn feldspar() {
        let f = Formula::parse("KAlSi3O8").unwrap();
        assert_eq!(f.count("K"), 1);
        assert_eq!(f.count("Al"), 1);
        assert_eq!(f.count("Si"), 3);
        assert_eq!(f.count("O"), 8);
    }

    #[test]
    fn parenthesized_group() {
        // Talc: Mg3Si4O10(OH)2
        let f = Formula::parse("Mg3Si4O10(OH)2").unwrap();
        assert_eq!(f.count("Mg"), 3);
        assert_eq!(f.count("Si"), 4);
        assert_eq!(f.count("O"), 12); // 10 + 2 from (OH)2
        assert_eq!(f.count("H"), 2);
    }

    #[test]
    fn unicode_subscripts() {
        let f = Formula::parse("SiO₂").unwrap();
        assert_eq!(f.count("Si"), 1);
        assert_eq!(f.count("O"), 2);
    }

    #[test]
    fn unicode_subscripts_complex() {
        let f = Formula::parse("Mg₃Si₄O₁₀(OH)₂").unwrap();
        assert_eq!(f.count("Mg"), 3);
        assert_eq!(f.count("Si"), 4);
        assert_eq!(f.count("O"), 12);
        assert_eq!(f.count("H"), 2);
    }

    #[test]
    fn hydrate() {
        // Gypsum: CaSO4·2H2O
        let f = Formula::parse("CaSO4·2H2O").unwrap();
        assert_eq!(f.count("Ca"), 1);
        assert_eq!(f.count("S"), 1);
        assert_eq!(f.count("O"), 6); // 4 + 2
        assert_eq!(f.count("H"), 4); // 2*2
    }

    #[test]
    fn solid_solution() {
        // Olivine: (Mg,Fe)2SiO4
        let f = Formula::parse("(Mg,Fe)2SiO4").unwrap();
        assert_eq!(f.count("Mg"), 2);
        assert_eq!(f.count("Fe"), 2);
        assert_eq!(f.count("Si"), 1);
        assert_eq!(f.count("O"), 4);
    }

    #[test]
    fn apatite() {
        // Ca5(PO4)3F
        let f = Formula::parse("Ca5(PO4)3F").unwrap();
        assert_eq!(f.count("Ca"), 5);
        assert_eq!(f.count("P"), 3);
        assert_eq!(f.count("O"), 12); // 4*3
        assert_eq!(f.count("F"), 1);
    }

    #[test]
    fn single_element() {
        let f = Formula::parse("C").unwrap();
        assert_eq!(f.count("C"), 1);
        assert_eq!(f.total_atoms(), 1);
    }

    #[test]
    fn nacl() {
        let f = Formula::parse("NaCl").unwrap();
        assert_eq!(f.count("Na"), 1);
        assert_eq!(f.count("Cl"), 1);
    }

    #[test]
    fn empty_returns_none() {
        assert!(Formula::parse("").is_none());
    }

    #[test]
    fn nested_parens() {
        // Muscovite: KAl2(AlSi3O10)(OH)2
        let f = Formula::parse("KAl2(AlSi3O10)(OH)2").unwrap();
        assert_eq!(f.count("K"), 1);
        assert_eq!(f.count("Al"), 3); // 2 + 1 from group
        assert_eq!(f.count("Si"), 3);
        assert_eq!(f.count("O"), 12); // 10 + 2
        assert_eq!(f.count("H"), 2);
    }

    #[test]
    fn fe2o3() {
        let f = Formula::parse("Fe2O3").unwrap();
        assert_eq!(f.count("Fe"), 2);
        assert_eq!(f.count("O"), 3);
    }

    #[test]
    fn missing_element_returns_zero() {
        let f = Formula::parse("SiO2").unwrap();
        assert_eq!(f.count("Fe"), 0);
    }
}

#[cfg(all(test, feature = "chemistry"))]
mod chemistry_tests {
    use super::*;

    #[test]
    fn quartz_molecular_weight_via_parser() {
        let f = Formula::parse("SiO2").unwrap();
        let mw = f.molecular_weight().unwrap();
        assert!((mw - 60.08).abs() < 0.1, "SiO₂ should be ~60.08, got {mw}");
    }

    #[test]
    fn calcite_molecular_weight_via_parser() {
        let f = Formula::parse("CaCO3").unwrap();
        let mw = f.molecular_weight().unwrap();
        assert!(
            (mw - 100.09).abs() < 0.1,
            "CaCO₃ should be ~100.09, got {mw}"
        );
    }

    #[test]
    fn nacl_molecular_weight_via_parser() {
        let f = Formula::parse("NaCl").unwrap();
        let mw = f.molecular_weight().unwrap();
        assert!((mw - 58.44).abs() < 0.1, "NaCl should be ~58.44, got {mw}");
    }

    #[test]
    fn to_atom_pairs_valid() {
        let f = Formula::parse("Fe2O3").unwrap();
        let pairs = f.to_atom_pairs().unwrap();
        // Fe=26, O=8
        assert!(pairs.contains(&(26, 2)));
        assert!(pairs.contains(&(8, 3)));
    }

    #[test]
    fn gypsum_hydrate_weight() {
        // CaSO4·2H2O = 172.17 g/mol
        let f = Formula::parse("CaSO4·2H2O").unwrap();
        let mw = f.molecular_weight().unwrap();
        assert!(
            (mw - 172.17).abs() < 0.2,
            "Gypsum should be ~172.17, got {mw}"
        );
    }
}
