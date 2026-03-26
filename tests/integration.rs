use khanij::{
    CrystalSystem, DepositType, GeologicalProcess, Mineral, OreDeposit, Rock, RockType,
    SoilComposition, SoilTexture, chemical_weathering_rate, erosion_rate, is_economically_viable,
    physical_weathering_rate, rock_cycle_next,
};

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
    // Forward cycle
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
    // Cross-paths
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
fn rock_validated_constructor() {
    let r = Rock::new("Test", RockType::Igneous, 2.7, 0.05, vec!["Quartz".into()]).unwrap();
    assert_eq!(r.name, "Test");
    assert_eq!(r.rock_type, RockType::Igneous);
}

#[test]
fn rock_serde_roundtrip() {
    let original = Rock::granite();
    let json = serde_json::to_string(&original).unwrap();
    let restored: Rock = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.name, original.name);
    assert_eq!(restored.rock_type, original.rock_type);
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
            assert!(
                (0.0..=1.0).contains(&rate),
                "physical rate {rate} out of bounds at temp={temp}, moist={moist}"
            );
        }
    }
}

#[test]
fn chemical_weathering_monotonic_with_temperature() {
    let mut prev = chemical_weathering_rate(-10.0, 1500.0);
    for temp in [0.0, 10.0, 20.0, 30.0, 40.0] {
        let rate = chemical_weathering_rate(temp, 1500.0);
        assert!(
            rate >= prev,
            "chemical weathering should increase with temperature"
        );
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
fn ore_deposit_validated_roundtrip() {
    let d = OreDeposit::new("Copper", DepositType::Porphyry, 0.02, 300.0, 500_000.0).unwrap();
    let json = serde_json::to_string(&d).unwrap();
    let back: OreDeposit = serde_json::from_str(&json).unwrap();
    assert_eq!(back.mineral, "Copper");
    assert_eq!(back.deposit_type, DepositType::Porphyry);
}

#[test]
fn economic_viability_scales_with_grade() {
    // Same deposit, higher grade should be more likely viable
    let low = is_economically_viable(0.001, 100_000.0, 5000.0, 10_000_000.0);
    let high = is_economically_viable(0.10, 100_000.0, 5000.0, 10_000_000.0);
    assert!(!low);
    assert!(high);
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
        assert!(
            pair[0].symmetry_order() < pair[1].symmetry_order(),
            "{:?} should have lower symmetry than {:?}",
            pair[0],
            pair[1]
        );
    }
}
