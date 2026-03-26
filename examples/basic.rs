use khanij::{
    CrystalSystem, DepositType, Formula, GeologicalProcess, Mineral, OreDeposit, Period, Rock,
    RockType, SoilComposition, bulk_density, chemical_weathering_rate, classify_age,
    classify_alteration, erosion_rate, estimated_ore_grade, is_economically_viable,
    physical_weathering_rate, precipitation_rate, rock_cycle_next,
};

fn main() {
    // --- Minerals ---
    println!("=== Minerals ===");
    let quartz = Mineral::quartz();
    let diamond = Mineral::diamond();
    println!(
        "{}: hardness {}, density {} g/cm³, {:?}",
        quartz.name,
        quartz.hardness.value(),
        quartz.density,
        quartz.crystal_system
    );
    println!(
        "Diamond scratches quartz? {}",
        diamond.hardness.scratches(&quartz.hardness)
    );
    println!(
        "Diamond Vickers hardness: {:.0} HV",
        diamond.hardness.to_vickers()
    );

    // --- Formula Parser ---
    println!("\n=== Formula Parser ===");
    let f = Formula::parse("Mg₃Si₄O₁₀(OH)₂").unwrap();
    println!(
        "Talc: Mg={}, Si={}, O={}, H={}",
        f.count("Mg"),
        f.count("Si"),
        f.count("O"),
        f.count("H")
    );
    let gypsum = Formula::parse("CaSO4·2H2O").unwrap();
    println!(
        "Gypsum: Ca={}, S={}, O={}, H={}",
        gypsum.count("Ca"),
        gypsum.count("S"),
        gypsum.count("O"),
        gypsum.count("H")
    );

    // --- Crystal Systems ---
    println!("\n=== Crystal Systems ===");
    println!(
        "Cubic: {} symmetry ops, Triclinic: {}",
        CrystalSystem::Cubic.symmetry_order(),
        CrystalSystem::Triclinic.symmetry_order()
    );

    // --- Rock Cycle ---
    println!("\n=== Rock Cycle ===");
    let granite = Rock::granite();
    println!(
        "{}: {:?}, density {} g/cm³",
        granite.name, granite.rock_type, granite.density
    );
    let next = rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering).unwrap();
    println!("Igneous + Weathering → {:?}", next);

    // --- Rock Density ---
    println!("\n=== Rock Density ===");
    let bd = bulk_density(2.65, 0.15, 1.0); // sandstone, water-filled
    println!("Sandstone bulk density (15% porosity, water-filled): {bd:.2} g/cm³");

    // --- Soil ---
    println!("\n=== Soil ===");
    let soil = SoilComposition::new(0.55, 0.15, 0.30).unwrap();
    println!(
        "Sand {}, Silt {}, Clay {} → {:?}",
        soil.sand,
        soil.silt,
        soil.clay,
        soil.texture()
    );

    // --- Weathering ---
    println!("\n=== Weathering ===");
    println!(
        "Physical (25°C range, 60% moisture): {:.3}",
        physical_weathering_rate(25.0, 0.6)
    );
    println!(
        "Chemical (20°C mean, 1200mm rain): {:.3}",
        chemical_weathering_rate(20.0, 1200.0)
    );
    println!(
        "Erosion (40mm/h rain, 15° slope, 50% cover): {:.3}",
        erosion_rate(40.0, 15.0, 0.5)
    );

    // --- Geologic Timescale ---
    println!("\n=== Geologic Timescale ===");
    let pos = classify_age(150.0);
    println!(
        "150 Ma: {:?} era, {:?} period",
        pos.era.unwrap(),
        pos.period.unwrap()
    );
    println!(
        "Jurassic: {:.1}-{:.1} Ma",
        Period::Jurassic.interval().start_ma,
        Period::Jurassic.interval().end_ma
    );

    // --- Ore Deposits ---
    println!("\n=== Ore Deposits ===");
    let deposit = OreDeposit::new("Gold", DepositType::Vein, 0.01, 200.0, 50_000.0).unwrap();
    println!(
        "{} {:?}: grade {}, {:.0}t contained metal",
        deposit.mineral,
        deposit.deposit_type,
        deposit.grade,
        deposit.contained_metal()
    );
    let viable = is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0);
    println!("High-grade deposit viable? {viable}");

    // --- Hydrothermal ---
    println!("\n=== Hydrothermal Ore Formation ===");
    println!("Alteration at 400°C: {:?}", classify_alteration(400.0));
    let precip = precipitation_rate(300.0, 300.0);
    println!("Gold precipitation rate at 300°C: {precip:.2}");
    let grade = estimated_ore_grade(1e-6, 300.0, 300.0, 0.1, 0.001);
    println!("Enhanced ore grade: {grade:.4} (background: 0.001)");
}
