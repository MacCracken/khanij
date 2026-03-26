use criterion::{Criterion, criterion_group, criterion_main};
use khanij::{
    // Core types
    CrystalSystem, Formula, GeologicalProcess, Mineral, MohsHardness, Rock, RockType,
    SoilComposition,
    // Crystallography
    UnitCell, bragg_angle, d_spacing,
    // Dating
    IsotopeSystem, c14_age, c14_fraction_remaining, decay_constant, half_life, parent_remaining,
    // Geochemistry
    MajorOxides, alumina_saturation_index, classify_asi, classify_tas, fractional_crystallization,
    mg_number,
    // Glaciology
    basal_sliding_velocity, equilibrium_line_altitude, glen_flow_law,
    ice_velocity_depth_integrated, isostatic_depression, isostatic_rebound_time, mass_balance,
    // Hydrothermal
    classify_alteration, estimated_ore_grade, metal_solubility, precipitation_rate,
    // Ore
    cutoff_grade, is_economically_viable, net_present_value, tonnage_grade_curve,
    // Rock
    bulk_density, bulk_density_from_minerals, porosity_from_density, rock_cycle_next,
    // Sediment
    SedimentSink, SedimentSource, compute_budget, denudation_rate, sediment_delivery_ratio,
    sediment_production, transport_capacity,
    // Stratigraphy
    SeaLevelCycle, accommodation_space, sediment_supply_ratio,
    // Tectonics
    EulerPole, SubductionZone, classify_ridge, full_spreading_rate, lithosphere_thickness,
    ocean_depth_m, ocean_floor_age,
    // Texture
    classify_grain_size, classify_sorting, mm_to_phi, phi_to_mm,
    // Timescale
    classify_age, eon_at_age, epoch_at_age, era_at_age, period_at_age,
    // Volcanic
    classify_magma, classify_vei, eruption_column_height, lava_flow_velocity, magma_viscosity,
    pyroclastic_flow_runout,
    // Weathering
    chemical_weathering_rate, erosion_rate, physical_weathering_rate,
};

// ---------------------------------------------------------------------------
// Mineral & Hardness
// ---------------------------------------------------------------------------

fn bench_mineral_presets(c: &mut Criterion) {
    c.bench_function("mineral_preset_quartz", |b| b.iter(Mineral::quartz));
    c.bench_function("mineral_preset_diamond", |b| b.iter(Mineral::diamond));
}

fn bench_mohs_hardness(c: &mut Criterion) {
    let diamond = MohsHardness::new(10.0).unwrap();
    let quartz = MohsHardness::new(7.0).unwrap();
    c.bench_function("mohs_scratches", |b| b.iter(|| diamond.scratches(&quartz)));
    c.bench_function("mohs_new_valid", |b| b.iter(|| MohsHardness::new(5.5)));
    c.bench_function("mohs_to_vickers", |b| b.iter(|| quartz.to_vickers()));
    c.bench_function("mohs_from_vickers", |b| {
        b.iter(|| MohsHardness::from_vickers(535.0))
    });
}

// ---------------------------------------------------------------------------
// Crystal & Crystallography
// ---------------------------------------------------------------------------

fn bench_crystallography(c: &mut Criterion) {
    c.bench_function("crystal_symmetry_order", |b| {
        b.iter(|| CrystalSystem::Cubic.symmetry_order())
    });
    let cell = UnitCell::halite();
    let hkl = khanij::MillerIndex::new(1, 1, 1);
    c.bench_function("unit_cell_volume", |b| b.iter(|| cell.volume()));
    c.bench_function("d_spacing_halite_111", |b| {
        b.iter(|| d_spacing(&cell, &hkl))
    });
    c.bench_function("bragg_angle", |b| b.iter(|| bragg_angle(2.82, 1.5406)));
}

// ---------------------------------------------------------------------------
// Rock
// ---------------------------------------------------------------------------

fn bench_rock(c: &mut Criterion) {
    c.bench_function("rock_cycle_next", |b| {
        b.iter(|| rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering))
    });
    c.bench_function("rock_new_validated", |b| {
        b.iter(|| Rock::new("Test", RockType::Igneous, 2.7, 0.05, vec!["Quartz".into()]))
    });
    c.bench_function("bulk_density", |b| {
        b.iter(|| bulk_density(2.65, 0.15, 1.0))
    });
    let minerals = [(2.65_f32, 0.30_f32), (2.56, 0.60), (2.82, 0.10)];
    c.bench_function("bulk_density_from_minerals", |b| {
        b.iter(|| bulk_density_from_minerals(&minerals, 0.01, 0.001))
    });
    c.bench_function("porosity_from_density", |b| {
        b.iter(|| porosity_from_density(2.25, 2.65))
    });
}

// ---------------------------------------------------------------------------
// Soil
// ---------------------------------------------------------------------------

fn bench_soil(c: &mut Criterion) {
    c.bench_function("soil_composition_new", |b| {
        b.iter(|| SoilComposition::new(0.4, 0.4, 0.2))
    });
    let soil = SoilComposition::new(0.4, 0.4, 0.2).unwrap();
    c.bench_function("soil_texture_classify", |b| b.iter(|| soil.texture()));
}

// ---------------------------------------------------------------------------
// Texture
// ---------------------------------------------------------------------------

fn bench_texture(c: &mut Criterion) {
    c.bench_function("classify_grain_size", |b| {
        b.iter(|| classify_grain_size(0.5))
    });
    c.bench_function("classify_sorting", |b| b.iter(|| classify_sorting(0.5)));
    c.bench_function("mm_to_phi", |b| b.iter(|| mm_to_phi(0.25)));
    c.bench_function("phi_to_mm", |b| b.iter(|| phi_to_mm(2.0)));
}

// ---------------------------------------------------------------------------
// Weathering
// ---------------------------------------------------------------------------

fn bench_weathering(c: &mut Criterion) {
    c.bench_function("physical_weathering_rate", |b| {
        b.iter(|| physical_weathering_rate(25.0, 0.6))
    });
    c.bench_function("chemical_weathering_rate", |b| {
        b.iter(|| chemical_weathering_rate(20.0, 1200.0))
    });
    c.bench_function("erosion_rate", |b| b.iter(|| erosion_rate(40.0, 15.0, 0.5)));
}

// ---------------------------------------------------------------------------
// Formula Parsing
// ---------------------------------------------------------------------------

fn bench_formula(c: &mut Criterion) {
    c.bench_function("formula_parse_simple", |b| {
        b.iter(|| Formula::parse("SiO2"))
    });
    c.bench_function("formula_parse_complex", |b| {
        b.iter(|| Formula::parse("Mg3Si4O10(OH)2"))
    });
    c.bench_function("formula_parse_hydrate", |b| {
        b.iter(|| Formula::parse("CaSO4·2H2O"))
    });
    c.bench_function("formula_parse_unicode", |b| {
        b.iter(|| Formula::parse("Mg₃Si₄O₁₀(OH)₂"))
    });
    c.bench_function("formula_parse_solid_solution", |b| {
        b.iter(|| Formula::parse("(Mg,Fe)2SiO4"))
    });
}

// ---------------------------------------------------------------------------
// Geochemistry
// ---------------------------------------------------------------------------

fn bench_geochemistry(c: &mut Criterion) {
    let oxides = MajorOxides {
        sio2: 72.0,
        tio2: 0.3,
        al2o3: 14.0,
        fe2o3: 1.5,
        feo: 1.0,
        mno: 0.05,
        mgo: 0.5,
        cao: 1.5,
        na2o: 3.5,
        k2o: 5.0,
        p2o5: 0.1,
        h2o: 0.5,
    };
    c.bench_function("major_oxides_tas", |b| {
        b.iter(|| oxides.tas_classification())
    });
    c.bench_function("classify_tas", |b| b.iter(|| classify_tas(52.0, 5.0)));
    c.bench_function("mg_number", |b| b.iter(|| mg_number(5.0, 3.0)));
    c.bench_function("alumina_saturation_index", |b| {
        b.iter(|| alumina_saturation_index(14.0, 1.5, 3.5, 5.0))
    });
    c.bench_function("classify_asi", |b| b.iter(|| classify_asi(1.1)));
    c.bench_function("fractional_crystallization", |b| {
        b.iter(|| fractional_crystallization(100.0, 0.5, 2.0))
    });
}

// ---------------------------------------------------------------------------
// Dating
// ---------------------------------------------------------------------------

fn bench_dating(c: &mut Criterion) {
    c.bench_function("c14_age", |b| b.iter(|| c14_age(0.5)));
    c.bench_function("c14_fraction_remaining", |b| {
        b.iter(|| c14_fraction_remaining(5730.0))
    });
    c.bench_function("decay_constant", |b| b.iter(|| decay_constant(4.47e9)));
    c.bench_function("half_life", |b| b.iter(|| half_life(1.55e-10)));
    c.bench_function("parent_remaining", |b| {
        b.iter(|| parent_remaining(1.55e-10, 1e9))
    });
    c.bench_function("isotope_system_age", |b| {
        b.iter(|| IsotopeSystem::C14.age(1.0))
    });
}

// ---------------------------------------------------------------------------
// Timescale
// ---------------------------------------------------------------------------

fn bench_timescale(c: &mut Criterion) {
    c.bench_function("classify_age", |b| b.iter(|| classify_age(66.0)));
    c.bench_function("period_at_age", |b| b.iter(|| period_at_age(150.0)));
    c.bench_function("era_at_age", |b| b.iter(|| era_at_age(150.0)));
    c.bench_function("eon_at_age", |b| b.iter(|| eon_at_age(150.0)));
    c.bench_function("epoch_at_age", |b| b.iter(|| epoch_at_age(5.0)));
}

// ---------------------------------------------------------------------------
// Tectonics
// ---------------------------------------------------------------------------

fn bench_tectonics(c: &mut Criterion) {
    let pole = EulerPole {
        latitude_deg: 62.0,
        longitude_deg: -41.0,
        omega_deg_per_myr: 0.95,
    };
    c.bench_function("euler_pole_velocity", |b| {
        b.iter(|| pole.velocity_mm_yr(30.0))
    });
    c.bench_function("full_spreading_rate", |b| {
        b.iter(|| full_spreading_rate(25.0))
    });
    c.bench_function("classify_ridge", |b| b.iter(|| classify_ridge(50.0)));
    c.bench_function("ocean_floor_age", |b| {
        b.iter(|| ocean_floor_age(500.0, 25.0))
    });
    c.bench_function("ocean_depth_m", |b| b.iter(|| ocean_depth_m(80.0)));
    c.bench_function("lithosphere_thickness", |b| {
        b.iter(|| lithosphere_thickness(80.0))
    });
    let sz = SubductionZone {
        dip_deg: 45.0,
        convergence_rate_mm_yr: 60.0,
        plate_age_ma: 80.0,
    };
    c.bench_function("slab_depth_km", |b| b.iter(|| sz.slab_depth_km(200.0)));
}

// ---------------------------------------------------------------------------
// Glaciology
// ---------------------------------------------------------------------------

fn bench_glaciology(c: &mut Criterion) {
    c.bench_function("glen_flow_law", |b| {
        b.iter(|| glen_flow_law(100_000.0, -10.0, 3.0))
    });
    c.bench_function("basal_sliding_velocity", |b| {
        b.iter(|| basal_sliding_velocity(100_000.0, 500_000.0))
    });
    c.bench_function("mass_balance", |b| b.iter(|| mass_balance(2.0, 3.0)));
    c.bench_function("equilibrium_line_altitude", |b| {
        b.iter(|| equilibrium_line_altitude(4000.0, 2000.0))
    });
    c.bench_function("isostatic_depression", |b| {
        b.iter(|| isostatic_depression(3000.0))
    });
    c.bench_function("isostatic_rebound_time", |b| {
        b.iter(|| isostatic_rebound_time(300.0, 1e21))
    });
    c.bench_function("ice_velocity_depth_integrated", |b| {
        b.iter(|| ice_velocity_depth_integrated(0.05, 500.0, -10.0))
    });
}

// ---------------------------------------------------------------------------
// Volcanic
// ---------------------------------------------------------------------------

fn bench_volcanic(c: &mut Criterion) {
    c.bench_function("classify_vei", |b| b.iter(|| classify_vei(1e9)));
    c.bench_function("classify_magma", |b| b.iter(|| classify_magma(65.0)));
    c.bench_function("magma_viscosity", |b| {
        b.iter(|| magma_viscosity(65.0, 1000.0))
    });
    c.bench_function("eruption_column_height", |b| {
        b.iter(|| eruption_column_height(1e6))
    });
    c.bench_function("lava_flow_velocity", |b| {
        b.iter(|| lava_flow_velocity(10.0, 100.0, 5.0, 2700.0))
    });
    c.bench_function("pyroclastic_flow_runout", |b| {
        b.iter(|| pyroclastic_flow_runout(25.0, 10.0))
    });
}

// ---------------------------------------------------------------------------
// Stratigraphy
// ---------------------------------------------------------------------------

fn bench_stratigraphy(c: &mut Criterion) {
    let cycle = SeaLevelCycle {
        amplitude_m: 50.0,
        period_years: 100_000.0,
    };
    c.bench_function("sea_level_at", |b| b.iter(|| cycle.sea_level_at(25_000.0)));
    c.bench_function("classify_systems_tract", |b| {
        b.iter(|| SeaLevelCycle::classify_systems_tract(0.25))
    });
    c.bench_function("accommodation_space", |b| {
        b.iter(|| accommodation_space(10.0, 5.0))
    });
    c.bench_function("sediment_supply_ratio", |b| {
        b.iter(|| sediment_supply_ratio(15.0, 10.0))
    });
}

// ---------------------------------------------------------------------------
// Sediment
// ---------------------------------------------------------------------------

fn bench_sediment(c: &mut Criterion) {
    c.bench_function("sediment_production", |b| {
        b.iter(|| sediment_production(0.01, 2650.0, 1e6, 0.001))
    });
    c.bench_function("transport_capacity", |b| {
        b.iter(|| transport_capacity(50.0, 0.01, 10.0, 1.5, 1.0))
    });
    c.bench_function("sediment_delivery_ratio", |b| {
        b.iter(|| sediment_delivery_ratio(100.0))
    });
    c.bench_function("denudation_rate", |b| {
        b.iter(|| denudation_rate(5000.0, 1e8, 2650.0))
    });
    let sources = vec![SedimentSource {
        name: "Hillslope".into(),
        production_rate: 1000.0,
        grain_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    }];
    let sinks = vec![SedimentSink {
        name: "Floodplain".into(),
        capacity: 500.0,
        accumulated: 0.0,
    }];
    c.bench_function("compute_budget", |b| {
        b.iter(|| compute_budget(&sources, 800.0, &sinks))
    });
}

// ---------------------------------------------------------------------------
// Hydrothermal
// ---------------------------------------------------------------------------

fn bench_hydrothermal(c: &mut Criterion) {
    c.bench_function("classify_alteration", |b| {
        b.iter(|| classify_alteration(350.0))
    });
    c.bench_function("metal_solubility", |b| {
        b.iter(|| metal_solubility(400.0, 300.0))
    });
    c.bench_function("precipitation_rate", |b| {
        b.iter(|| precipitation_rate(400.0, 300.0))
    });
    c.bench_function("estimated_ore_grade", |b| {
        b.iter(|| estimated_ore_grade(1.0, 400.0, 300.0, 0.1, 0.001))
    });
}

// ---------------------------------------------------------------------------
// Ore
// ---------------------------------------------------------------------------

fn bench_ore(c: &mut Criterion) {
    c.bench_function("is_economically_viable", |b| {
        b.iter(|| is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0))
    });
    c.bench_function("cutoff_grade", |b| {
        b.iter(|| cutoff_grade(60_000_000.0, 50.0, 0.90))
    });
    c.bench_function("net_present_value", |b| {
        b.iter(|| net_present_value(10_000_000.0, 7_000_000.0, 0.08, 10.0))
    });
    let blocks: Vec<(f64, f64)> = (0..100)
        .map(|i| (1000.0, 0.001 + 0.001 * i as f64))
        .collect();
    c.bench_function("tonnage_grade_curve_100_blocks", |b| {
        b.iter(|| tonnage_grade_curve(&blocks, 20))
    });
}

// ---------------------------------------------------------------------------
// Feature-gated benchmarks
// ---------------------------------------------------------------------------

#[cfg(feature = "fluids")]
fn bench_hydrology(c: &mut Criterion) {
    use khanij::{
        darcy_flow, is_grain_mobile, shields_parameter, stokes_settling_velocity, theis_drawdown,
        transport_regime, well_function,
    };
    c.bench_function("stokes_settling_velocity", |b| {
        b.iter(|| stokes_settling_velocity(2650.0, 1000.0, 0.0005, 0.001, 9.81))
    });
    c.bench_function("darcy_flow", |b| b.iter(|| darcy_flow(1e-6, 0.01, 100.0)));
    c.bench_function("well_function", |b| b.iter(|| well_function(0.01)));
    c.bench_function("theis_drawdown", |b| {
        b.iter(|| theis_drawdown(0.01, 0.001, 1e-4, 10.0, 86400.0))
    });
    c.bench_function("shields_parameter", |b| {
        b.iter(|| shields_parameter(5.0, 2650.0, 1000.0, 0.001, 9.81))
    });
    c.bench_function("is_grain_mobile", |b| {
        b.iter(|| is_grain_mobile(5.0, 2650.0, 1000.0, 0.001, 9.81))
    });
    c.bench_function("transport_regime", |b| {
        b.iter(|| transport_regime(0.001, 0.5))
    });
}

#[cfg(feature = "thermodynamics")]
fn bench_geothermal(c: &mut Criterion) {
    use khanij::{
        classify_facies, contact_aureole_temperature, facies_at_depth, heat_flux, heat_stored,
        intrusion_cooling, lithostatic_pressure, rock_thermal_diffusivity, temperature_at_depth,
    };
    c.bench_function("heat_flux", |b| {
        b.iter(|| heat_flux(3.0, 1.0, 373.15, 288.15, 1000.0))
    });
    c.bench_function("temperature_at_depth", |b| {
        b.iter(|| temperature_at_depth(288.15, 0.025, 1000.0))
    });
    c.bench_function("rock_thermal_diffusivity", |b| {
        b.iter(|| rock_thermal_diffusivity(3.0, 2700.0, 790.0))
    });
    c.bench_function("heat_stored", |b| {
        b.iter(|| heat_stored(1000.0, 790.0, 100.0))
    });
    c.bench_function("lithostatic_pressure", |b| {
        b.iter(|| lithostatic_pressure(2700.0, 9.81, 5000.0))
    });
    c.bench_function("classify_facies", |b| {
        b.iter(|| classify_facies(500.0, 0.5))
    });
    c.bench_function("facies_at_depth", |b| {
        b.iter(|| facies_at_depth(20_000.0, 25.0, 288.15, 2700.0))
    });
    c.bench_function("intrusion_cooling", |b| {
        b.iter(|| intrusion_cooling(1473.15, 573.15, 500.0, 1e-6, 1e6))
    });
    c.bench_function("contact_aureole_temperature", |b| {
        b.iter(|| contact_aureole_temperature(100.0, 500.0, 1473.15, 573.15))
    });
}

#[cfg(feature = "mechanics")]
fn bench_rock_mechanics(c: &mut Criterion) {
    use khanij::{
        brittle_ductile_transition_depth, granite_material, mohr_coulomb_strength,
        mohr_coulomb_to_drucker_prager, p_wave_velocity, s_wave_velocity, vp_vs_ratio,
    };
    let mat = granite_material();
    c.bench_function("granite_material", |b| b.iter(granite_material));
    c.bench_function("p_wave_velocity", |b| b.iter(|| p_wave_velocity(&mat)));
    c.bench_function("s_wave_velocity", |b| b.iter(|| s_wave_velocity(&mat)));
    c.bench_function("vp_vs_ratio", |b| b.iter(|| vp_vs_ratio(&mat)));
    c.bench_function("mohr_coulomb_strength", |b| {
        b.iter(|| mohr_coulomb_strength(50e6, 10e6, std::f64::consts::FRAC_PI_6))
    });
    c.bench_function("mohr_coulomb_to_drucker_prager", |b| {
        b.iter(|| mohr_coulomb_to_drucker_prager(std::f64::consts::FRAC_PI_6, 10e6))
    });
    c.bench_function("brittle_ductile_transition_depth", |b| {
        b.iter(|| {
            brittle_ductile_transition_depth(&mat, 10e6, std::f64::consts::FRAC_PI_6, 9.81)
        })
    });
}

#[cfg(feature = "chemistry")]
fn bench_stability(c: &mut Criterion) {
    use khanij::{
        equilibrium_temperature, gibbs_formation, is_reaction_spontaneous, reaction_gibbs,
        stable_polymorph,
    };
    c.bench_function("gibbs_formation", |b| {
        b.iter(|| gibbs_formation("SiO2(s)"))
    });
    c.bench_function("stable_polymorph", |b| {
        b.iter(|| stable_polymorph("SiO2(s)", "CaCO3(s)", 800.0))
    });
    c.bench_function("reaction_gibbs", |b| {
        b.iter(|| {
            reaction_gibbs(
                &[("Fe2O3(s)", 1.0)],
                &[("Fe(s)", 2.0), ("O2(g)", 1.5)],
            )
        })
    });
    c.bench_function("is_reaction_spontaneous", |b| {
        b.iter(|| {
            is_reaction_spontaneous(
                &[("Fe2O3(s)", 1.0)],
                &[("Fe(s)", 2.0), ("O2(g)", 1.5)],
            )
        })
    });
    c.bench_function("equilibrium_temperature", |b| {
        b.iter(|| {
            equilibrium_temperature(
                &[("CaO(s)", 1.0), ("CO2(g)", 1.0)],
                &[("CaCO3(s)", 1.0)],
            )
        })
    });
}

// ---------------------------------------------------------------------------
// Groups
// ---------------------------------------------------------------------------

criterion_group!(
    core_benches,
    bench_mineral_presets,
    bench_mohs_hardness,
    bench_crystallography,
    bench_rock,
    bench_soil,
    bench_texture,
    bench_weathering,
    bench_formula,
    bench_geochemistry,
    bench_dating,
    bench_timescale,
    bench_tectonics,
    bench_glaciology,
    bench_volcanic,
    bench_stratigraphy,
    bench_sediment,
    bench_hydrothermal,
    bench_ore,
);

#[cfg(feature = "fluids")]
criterion_group!(fluids_benches, bench_hydrology);
#[cfg(feature = "thermodynamics")]
criterion_group!(thermo_benches, bench_geothermal);
#[cfg(feature = "mechanics")]
criterion_group!(mechanics_benches, bench_rock_mechanics);
#[cfg(feature = "chemistry")]
criterion_group!(chemistry_benches, bench_stability);

#[cfg(all(
    feature = "fluids",
    feature = "thermodynamics",
    feature = "mechanics",
    feature = "chemistry"
))]
criterion_main!(
    core_benches,
    fluids_benches,
    thermo_benches,
    mechanics_benches,
    chemistry_benches
);

#[cfg(not(all(
    feature = "fluids",
    feature = "thermodynamics",
    feature = "mechanics",
    feature = "chemistry"
)))]
criterion_main!(core_benches);
