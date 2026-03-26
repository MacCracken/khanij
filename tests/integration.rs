use khanij::{
    AlterationZone, CrystalSystem, Eon, Era, GeologicalProcess, Mineral, Period, Rock, RockType,
    SedimentSink, SedimentSource, SoilComposition, SoilTexture, bulk_density,
    bulk_density_from_minerals, chemical_weathering_rate, classify_age, classify_alteration,
    compute_budget, cutoff_grade, denudation_rate, erosion_rate, estimated_ore_grade,
    net_present_value, physical_weathering_rate, porosity_from_density, precipitation_rate,
    rock_cycle_next, sediment_delivery_ratio, sediment_production, tonnage_grade_curve,
    transport_capacity,
};

// ---------------------------------------------------------------------------
// Original integration tests
// ---------------------------------------------------------------------------

#[test]
fn mineral_presets_have_valid_hardness() {
    for m in [
        Mineral::quartz(),
        Mineral::feldspar(),
        Mineral::calcite(),
        Mineral::diamond(),
        Mineral::talc(),
    ] {
        let h = m.hardness.value();
        assert!(
            (1.0..=10.0).contains(&h),
            "{} hardness {} out of Mohs range",
            m.name,
            h
        );
    }
}

#[test]
fn diamond_is_hardest_preset() {
    let diamond = Mineral::diamond();
    for m in [
        Mineral::quartz(),
        Mineral::feldspar(),
        Mineral::calcite(),
        Mineral::talc(),
    ] {
        assert!(
            diamond.hardness.scratches(&m.hardness),
            "Diamond should scratch {}",
            m.name
        );
    }
}

#[test]
fn mineral_serde_roundtrip() {
    let original = Mineral::quartz();
    let json = serde_json::to_string(&original).unwrap();
    let restored: Mineral = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.name, original.name);
    assert_eq!(restored.formula, original.formula);
    assert!((restored.hardness.value() - original.hardness.value()).abs() < f32::EPSILON);
}

#[test]
fn full_rock_cycle_all_paths() {
    assert_eq!(
        rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering),
        Some(RockType::Sedimentary)
    );
    assert_eq!(
        rock_cycle_next(RockType::Sedimentary, GeologicalProcess::Metamorphism),
        Some(RockType::Metamorphic)
    );
    assert_eq!(
        rock_cycle_next(RockType::Metamorphic, GeologicalProcess::Melting),
        Some(RockType::Igneous)
    );
    assert_eq!(
        rock_cycle_next(RockType::Igneous, GeologicalProcess::Metamorphism),
        Some(RockType::Metamorphic)
    );
    assert_eq!(
        rock_cycle_next(RockType::Sedimentary, GeologicalProcess::Melting),
        Some(RockType::Igneous)
    );
    assert_eq!(
        rock_cycle_next(RockType::Metamorphic, GeologicalProcess::Weathering),
        Some(RockType::Sedimentary)
    );
}

#[test]
fn soil_composition_texture_classification() {
    let sandy = SoilComposition::new(0.90, 0.05, 0.05).unwrap();
    assert_eq!(sandy.texture(), SoilTexture::Sand);
    let clayey = SoilComposition::new(0.20, 0.30, 0.50).unwrap();
    assert_eq!(clayey.texture(), SoilTexture::Clay);
    let silty = SoilComposition::new(0.05, 0.88, 0.07).unwrap();
    assert_eq!(silty.texture(), SoilTexture::Silt);
    let loamy = SoilComposition::new(0.40, 0.40, 0.20).unwrap();
    assert_eq!(loamy.texture(), SoilTexture::Loam);
}

#[test]
fn weathering_rates_are_bounded() {
    for temp in [0.0, 10.0, 25.0, 50.0] {
        for moist in [0.0, 0.5, 1.0] {
            let rate = physical_weathering_rate(temp, moist);
            assert!((0.0..=1.0).contains(&rate), "rate {rate} out of bounds");
        }
    }
}

#[test]
fn chemical_weathering_monotonic_with_temperature() {
    let mut prev = chemical_weathering_rate(-10.0, 1500.0);
    for temp in [0.0, 10.0, 20.0, 30.0, 40.0] {
        let rate = chemical_weathering_rate(temp, 1500.0);
        assert!(rate >= prev);
        prev = rate;
    }
}

#[test]
fn erosion_increases_with_slope() {
    let gentle = erosion_rate(50.0, 5.0, 0.3);
    let steep = erosion_rate(50.0, 30.0, 0.3);
    assert!(steep > gentle);
}

#[test]
fn crystal_system_symmetry_ordering() {
    let systems = [
        CrystalSystem::Triclinic,
        CrystalSystem::Monoclinic,
        CrystalSystem::Orthorhombic,
        CrystalSystem::Trigonal,
        CrystalSystem::Tetragonal,
        CrystalSystem::Hexagonal,
        CrystalSystem::Cubic,
    ];
    for pair in systems.windows(2) {
        assert!(pair[0].symmetry_order() < pair[1].symmetry_order());
    }
}

// ---------------------------------------------------------------------------
// Cross-module integration tests
// ---------------------------------------------------------------------------

#[test]
fn formula_parses_all_mineral_presets() {
    for m in [
        Mineral::quartz(),
        Mineral::feldspar(),
        Mineral::calcite(),
        Mineral::diamond(),
        Mineral::talc(),
        Mineral::olivine(),
        Mineral::pyrite(),
        Mineral::magnetite(),
        Mineral::halite(),
        Mineral::gypsum(),
        Mineral::fluorite(),
        Mineral::apatite(),
    ] {
        let f = m.parsed_formula();
        assert!(
            f.is_some(),
            "Failed to parse formula for {}: {}",
            m.name,
            m.formula
        );
        assert!(
            f.unwrap().total_atoms() > 0,
            "{} formula has no atoms",
            m.name
        );
    }
}

#[test]
fn mohs_reference_scale_complete() {
    // The 10 reference minerals on the Mohs scale
    let mohs_minerals = [
        Mineral::talc(),     // 1
        Mineral::gypsum(),   // 2
        Mineral::calcite(),  // 3
        Mineral::fluorite(), // 4
        Mineral::apatite(),  // 5
        Mineral::feldspar(), // 6
        Mineral::quartz(),   // 7
        Mineral::topaz(),    // 8
        Mineral::corundum(), // 9
        Mineral::diamond(),  // 10
    ];
    for pair in mohs_minerals.windows(2) {
        assert!(
            pair[1].hardness.scratches(&pair[0].hardness),
            "{} (Mohs {}) should scratch {} (Mohs {})",
            pair[1].name,
            pair[1].hardness.value(),
            pair[0].name,
            pair[0].hardness.value(),
        );
    }
}

#[test]
fn vickers_mohs_roundtrip_all_presets() {
    for mohs_val in [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0] {
        let m = khanij::MohsHardness::new(mohs_val).unwrap();
        let hv = m.to_vickers();
        let recovered = khanij::MohsHardness::from_vickers(hv).unwrap();
        assert!(
            (recovered.value() - mohs_val).abs() < 0.2,
            "Mohs {mohs_val} → HV {hv} → Mohs {} (expected ~{mohs_val})",
            recovered.value()
        );
    }
}

#[test]
fn geologic_timescale_covers_earth_history() {
    // Spot-check key events
    let pos = classify_age(65.0); // just after K-Pg boundary
    assert_eq!(pos.era, Some(Era::Cenozoic));
    assert_eq!(pos.period, Some(Period::Paleogene));

    let pos = classify_age(260.0); // mid-Permian
    assert_eq!(pos.era, Some(Era::Paleozoic));
    assert_eq!(pos.period, Some(Period::Permian));

    let pos = classify_age(3500.0); // early life
    assert_eq!(pos.eon, Some(Eon::Archean));
}

#[test]
fn rock_density_from_mineral_composition() {
    // Granite: quartz(2.65) 30%, feldspar(2.56) 60%, mica(2.82) 10%, 1% porosity
    let minerals = [(2.65_f32, 0.3), (2.56, 0.6), (2.82, 0.1)];
    let bd = bulk_density_from_minerals(&minerals, 0.01, 1.0);
    let granite = Rock::granite();
    // Should be close to granite's stated density
    assert!(
        (bd - granite.density).abs() < 0.2,
        "Computed {bd}, expected ~{}",
        granite.density
    );
}

#[test]
fn porosity_density_relationship() {
    let grain = 2.65_f32;
    for phi in [0.0, 0.05, 0.10, 0.20, 0.30] {
        let bd = bulk_density(grain, phi, 0.001);
        let recovered_phi = porosity_from_density(bd, grain);
        assert!((recovered_phi - phi).abs() < 0.02);
    }
}

#[test]
fn ore_economics_full_workflow() {
    // Define deposit blocks
    let blocks: Vec<(f64, f64)> = vec![
        (5000.0, 0.005),
        (3000.0, 0.01),
        (2000.0, 0.02),
        (1000.0, 0.04),
        (500.0, 0.08),
    ];
    let curve = tonnage_grade_curve(&blocks, 10);
    assert!(!curve.is_empty());

    // Calculate cutoff grade for copper at $8000/t, $30/t mining cost, 85% recovery
    let cog = cutoff_grade(8000.0, 30.0, 0.85).unwrap();
    assert!(cog > 0.0 && cog < 0.01);

    // NPV of a 15-year mine
    let npv = net_present_value(50e6, 35e6, 0.10, 15.0).unwrap();
    assert!(npv > 0.0); // profitable project
}

#[test]
fn sediment_source_to_sink() {
    // Weathering produces sediment
    let prod = sediment_production(0.3, 2700.0, 1e6, 0.0005);
    assert!(prod > 0.0);

    // River can transport some fraction
    let cap = transport_capacity(10.0, 0.005, 0.005, 1.5, 1.0);
    assert!(cap > 0.0);

    // Delivery ratio for a 100 km² catchment
    let sdr = sediment_delivery_ratio(100.0);
    assert!(sdr > 0.0 && sdr < 1.0);

    // Denudation rate
    let d = denudation_rate(prod * sdr, 1e8, 2700.0);
    assert!(d > 0.0);
}

#[test]
fn sediment_budget_balances() {
    let sources = vec![SedimentSource {
        name: "Hillslope".into(),
        production_rate: 1000.0,
        grain_fractions: [0.2, 0.3, 0.3, 0.15, 0.05],
    }];
    let sinks = vec![SedimentSink {
        name: "Delta".into(),
        capacity: 800.0,
        accumulated: 0.0,
    }];
    let budget = compute_budget(&sources, 1500.0, &sinks);
    assert!((budget.total_production - budget.total_deposition - budget.total_export).abs() < 0.01);
}

#[test]
fn hydrothermal_alteration_zones_ordered() {
    let temps = [600.0, 400.0, 300.0, 200.0, 100.0];
    let expected = [
        AlterationZone::Potassic,
        AlterationZone::Phyllic,
        AlterationZone::Argillic,
        AlterationZone::Propylitic,
        AlterationZone::Unaltered,
    ];
    for (t, e) in temps.iter().zip(expected.iter()) {
        assert_eq!(classify_alteration(*t), *e);
    }
}

#[test]
fn gold_precipitation_peaks_at_300c() {
    let at_300 = precipitation_rate(300.0, 300.0);
    let at_200 = precipitation_rate(200.0, 300.0);
    let at_400 = precipitation_rate(400.0, 300.0);
    assert!(at_300 > at_200);
    assert!(at_300 > at_400);
}

#[test]
fn ore_grade_enhanced_by_fluid_focusing() {
    let bg = 0.001;
    let enhanced = estimated_ore_grade(1e-6, 300.0, 300.0, 0.15, bg);
    assert!(enhanced > bg);
    let far = estimated_ore_grade(1e-6, 100.0, 300.0, 0.15, bg);
    assert!(enhanced > far);
}
