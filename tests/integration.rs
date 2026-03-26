use khanij::{
    AlterationZone, CrystalSystem, DepositType, Eon, Era, EulerPole, GeologicUnit,
    GeologicalProcess, IsochronPoint, MajorOxides, MillerIndex, Mineral, MohsHardness, OreDeposit,
    Period, Rock, RockType, SeaLevelCycle, SedimentSink, SedimentSource, SoilComposition,
    SoilTexture, StrikeDip, UnitCell, Vei, bragg_angle, bulk_density, bulk_density_from_minerals,
    chemical_weathering_rate, classify_age, classify_alteration, classify_vei, compute_budget,
    cutoff_grade, d_spacing, denudation_rate, eon_at_age, era_at_age, erosion_rate,
    eruption_column_height, estimated_ore_grade, net_present_value, ocean_depth_m, ocean_floor_age,
    period_at_age, physical_weathering_rate, porosity_from_density, precipitation_rate,
    pyroclastic_flow_runout, rock_cycle_next, sediment_delivery_ratio, sediment_production,
    tonnage_grade_curve, transport_capacity,
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
    assert!((restored.hardness.value() - original.hardness.value()).abs() < f64::EPSILON);
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
    let minerals = [(2.65_f64, 0.3), (2.56, 0.6), (2.82, 0.1)];
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
    let grain = 2.65_f64;
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

// ---------------------------------------------------------------------------
// Serde roundtrip tests for all major public structs
// ---------------------------------------------------------------------------

#[test]
fn serde_roundtrip_all_types() {
    // Rock::granite()
    {
        let orig = Rock::granite();
        let json = serde_json::to_string(&orig).unwrap();
        let back: Rock = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, orig.name);
        assert_eq!(back.rock_type, orig.rock_type);
        assert!((back.density - orig.density).abs() < f64::EPSILON);
        assert!((back.porosity - orig.porosity).abs() < f64::EPSILON);
        assert_eq!(back.primary_minerals, orig.primary_minerals);
    }

    // SoilComposition::new(0.4, 0.4, 0.2)
    {
        let orig = SoilComposition::new(0.4, 0.4, 0.2).unwrap();
        let json = serde_json::to_string(&orig).unwrap();
        let back: SoilComposition = serde_json::from_str(&json).unwrap();
        assert_eq!(back.texture(), orig.texture());
    }

    // MohsHardness::new(7.0)
    {
        let orig = MohsHardness::new(7.0).unwrap();
        let json = serde_json::to_string(&orig).unwrap();
        let back: MohsHardness = serde_json::from_str(&json).unwrap();
        assert!((back.value() - orig.value()).abs() < f64::EPSILON);
    }

    // UnitCell::cubic(5.64)
    {
        let orig = UnitCell::cubic(5.64);
        let json = serde_json::to_string(&orig).unwrap();
        let back: UnitCell = serde_json::from_str(&json).unwrap();
        assert!((back.volume() - orig.volume()).abs() < 1e-6);
    }

    // OreDeposit
    {
        let orig = OreDeposit::new("Gold", DepositType::Vein, 0.005, 100.0, 1_000_000.0).unwrap();
        let json = serde_json::to_string(&orig).unwrap();
        let back: OreDeposit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.mineral, orig.mineral);
        assert_eq!(back.deposit_type, orig.deposit_type);
        assert!((back.grade - orig.grade).abs() < f64::EPSILON);
        assert!((back.depth_m - orig.depth_m).abs() < f64::EPSILON);
        assert!((back.tonnage - orig.tonnage).abs() < f64::EPSILON);
    }

    // StrikeDip
    {
        let orig = StrikeDip {
            strike_deg: 45.0,
            dip_deg: 30.0,
        };
        let json = serde_json::to_string(&orig).unwrap();
        let back: StrikeDip = serde_json::from_str(&json).unwrap();
        assert_eq!(back, orig);
    }

    // GeologicUnit
    {
        let orig = GeologicUnit {
            name: "Sandstone A".into(),
            rock_type: "Sedimentary".into(),
            age_ma: 250.0,
        };
        let json = serde_json::to_string(&orig).unwrap();
        let back: GeologicUnit = serde_json::from_str(&json).unwrap();
        assert_eq!(back, orig);
    }

    // EulerPole
    {
        let orig = EulerPole {
            latitude_deg: 62.0,
            longitude_deg: -41.0,
            omega_deg_per_myr: 0.95,
        };
        let json = serde_json::to_string(&orig).unwrap();
        let back: EulerPole = serde_json::from_str(&json).unwrap();
        assert!((back.latitude_deg - orig.latitude_deg).abs() < f64::EPSILON);
        assert!((back.longitude_deg - orig.longitude_deg).abs() < f64::EPSILON);
        assert!((back.omega_deg_per_myr - orig.omega_deg_per_myr).abs() < f64::EPSILON);
    }

    // SeaLevelCycle
    {
        let orig = SeaLevelCycle {
            amplitude_m: 50.0,
            period_years: 100_000.0,
        };
        let json = serde_json::to_string(&orig).unwrap();
        let back: SeaLevelCycle = serde_json::from_str(&json).unwrap();
        assert!((back.amplitude_m - orig.amplitude_m).abs() < f64::EPSILON);
        assert!((back.period_years - orig.period_years).abs() < f64::EPSILON);
    }

    // MajorOxides (basalt composition)
    {
        let orig = MajorOxides {
            sio2: 49.5,
            tio2: 1.5,
            al2o3: 15.5,
            fe2o3: 2.5,
            feo: 7.5,
            mno: 0.17,
            mgo: 8.0,
            cao: 11.0,
            na2o: 2.5,
            k2o: 0.5,
            p2o5: 0.2,
            h2o: 0.5,
        };
        let json = serde_json::to_string(&orig).unwrap();
        let back: MajorOxides = serde_json::from_str(&json).unwrap();
        assert!((back.sio2 - orig.sio2).abs() < f64::EPSILON);
        assert!((back.total() - orig.total()).abs() < 1e-10);
    }

    // IsochronPoint
    {
        let orig = IsochronPoint { x: 0.7, y: 0.71 };
        let json = serde_json::to_string(&orig).unwrap();
        let back: IsochronPoint = serde_json::from_str(&json).unwrap();
        assert!((back.x - orig.x).abs() < f64::EPSILON);
        assert!((back.y - orig.y).abs() < f64::EPSILON);
    }
}

// ---------------------------------------------------------------------------
// Cross-module workflow tests
// ---------------------------------------------------------------------------

#[test]
fn test_mineral_to_formula_pipeline() {
    let mineral = Mineral::quartz(); // SiO2
    let formula = mineral
        .parsed_formula()
        .expect("quartz formula should parse");
    let si = formula.count("Si");
    let o = formula.count("O");
    assert_eq!(si, 1, "SiO2 should have 1 Si");
    assert_eq!(o, 2, "SiO2 should have 2 O");
    assert_eq!(formula.total_atoms(), 3, "SiO2 has 3 total atoms");
}

#[test]
fn test_crystallography_bragg_workflow() {
    // NaCl cubic cell, a = 5.64 A
    let cell = UnitCell::cubic(5.64);
    let hkl = MillerIndex { h: 1, k: 0, l: 0 };

    // d-spacing for (1,0,0) of a cubic cell = a / sqrt(1) = a
    let d = d_spacing(&cell, &hkl);
    assert!(
        (d - 5.64).abs() < 1e-6,
        "d_100 should equal a for cubic cell"
    );

    // Bragg angle with Cu K-alpha (1.5406 A)
    let wavelength = 1.5406;
    let theta = bragg_angle(d, wavelength).expect("Bragg angle should exist");
    // sin(theta) = lambda / (2d) = 1.5406 / 11.28 ~ 0.1365
    // theta ~ 7.85 degrees
    assert!(
        theta > 5.0 && theta < 15.0,
        "Bragg angle should be reasonable, got {theta}"
    );
}

#[test]
fn test_geologic_timescale_consistency() {
    let ages = [0.01, 66.0, 252.0, 541.0, 2500.0, 4000.0];
    for age in ages {
        let eon = eon_at_age(age);
        let era = era_at_age(age);
        assert!(eon.is_some(), "eon_at_age({age}) should return Some");

        // For Phanerozoic ages, period and era should both exist and be consistent
        if age < 538.8 {
            let period = period_at_age(age);
            assert!(
                period.is_some(),
                "period_at_age({age}) should return Some for Phanerozoic"
            );
            assert!(
                era.is_some(),
                "era_at_age({age}) should return Some for Phanerozoic"
            );
            // Check hierarchy: period's parent era matches era_at_age
            let p = period.unwrap();
            let expected_era = era.unwrap();
            assert_eq!(
                p.era(),
                expected_era,
                "Period {:?} at age {age} Ma should belong to era {:?}, got {:?}",
                p,
                expected_era,
                p.era()
            );
        }
    }
}

#[test]
fn test_tectonics_ocean_floor_workflow() {
    let pole = EulerPole {
        latitude_deg: 62.0,
        longitude_deg: -41.0,
        omega_deg_per_myr: 0.95,
    };
    // Compute velocity at 90 degrees from pole (maximum velocity)
    let v = pole.velocity_mm_yr(90.0);
    assert!(v > 0.0, "velocity should be positive");

    // Use half the velocity as a half-spreading rate
    let half_rate = v / 2.0;
    // Ocean floor age at 500 km from ridge
    let age = ocean_floor_age(500.0, half_rate);
    assert!(age > 0.0, "ocean floor age should be positive");

    // Depth at that age
    let depth = ocean_depth_m(age);
    assert!(
        depth > 2500.0,
        "ocean depth should be greater than ridge crest depth (2500 m), got {depth}"
    );
}

#[test]
fn test_sediment_budget_balances() {
    let sources = vec![
        SedimentSource {
            name: "Hillslope".into(),
            production_rate: 1000.0,
            grain_fractions: [0.2, 0.3, 0.3, 0.15, 0.05],
        },
        SedimentSource {
            name: "River bank".into(),
            production_rate: 500.0,
            grain_fractions: [0.1, 0.2, 0.4, 0.2, 0.1],
        },
    ];
    let sinks = vec![
        SedimentSink {
            name: "Delta".into(),
            capacity: 800.0,
            accumulated: 0.0,
        },
        SedimentSink {
            name: "Floodplain".into(),
            capacity: 400.0,
            accumulated: 100.0,
        },
    ];
    let budget = compute_budget(&sources, 1500.0, &sinks);
    // Conservation: total_production = total_deposition + total_export
    assert!(
        (budget.total_production - budget.total_deposition - budget.total_export).abs() < 0.01,
        "sediment budget must balance: production={} deposition={} export={}",
        budget.total_production,
        budget.total_deposition,
        budget.total_export
    );
    // total_production should be >= total_deposition + total_export (by definition they equal)
    assert!(
        budget.total_production >= budget.total_deposition,
        "production should be >= deposition"
    );
}

#[test]
fn test_volcanic_eruption_cascade() {
    // A VEI-5 eruption: ~1e9 m3 ejecta
    let volume = 1e9;
    let vei = classify_vei(volume);
    assert_eq!(vei, Vei::V5, "1e9 m3 should classify as VEI 5");

    // Eruption column height from a mass flux (typical VEI-5 mass flux ~1e7 kg/s)
    let mass_flux = 1e7;
    let column_height = eruption_column_height(mass_flux);
    assert!(
        column_height > 0.0,
        "column height should be positive, got {column_height}"
    );
    // Typical VEI-5 column heights are ~25 km; our formula gives 0.236 * (1e7)^0.25
    // = 0.236 * 56.23 = ~13.3 km
    assert!(
        column_height > 5.0 && column_height < 50.0,
        "column height should be in reasonable range (5-50 km), got {column_height}"
    );

    // Pyroclastic flow runout on a 10-degree slope
    let runout = pyroclastic_flow_runout(column_height, 10.0);
    assert!(
        runout > 0.0,
        "pyroclastic flow runout should be positive, got {runout}"
    );
    assert!(
        runout > column_height,
        "runout on a gentle slope should exceed column height"
    );
}
