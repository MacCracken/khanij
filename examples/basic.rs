use khanij::{
    CrystalSystem, GeologicalProcess, Mineral, Rock, RockType, SoilComposition,
    DepositType, OreDeposit,
    rock_cycle_next, is_economically_viable,
    physical_weathering_rate, chemical_weathering_rate, erosion_rate,
};

fn main() {
    // --- Minerals ---
    let quartz = Mineral::quartz();
    let diamond = Mineral::diamond();
    println!("=== Minerals ===");
    println!("{}: hardness {}, density {} g/cm³, {:?} system",
        quartz.name, quartz.hardness.value(), quartz.density, quartz.crystal_system);
    println!("Diamond scratches quartz? {}", diamond.hardness.scratches(&quartz.hardness));

    // --- Crystal Systems ---
    println!("\n=== Crystal Systems ===");
    println!("Cubic symmetry order: {}", CrystalSystem::Cubic.symmetry_order());
    println!("Triclinic symmetry order: {}", CrystalSystem::Triclinic.symmetry_order());

    // --- Rock Cycle ---
    println!("\n=== Rock Cycle ===");
    let granite = Rock::granite();
    println!("{}: {:?}, density {} g/cm³", granite.name, granite.rock_type, granite.density);
    let next = rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering);
    println!("Igneous + Weathering → {:?}", next.unwrap());

    // --- Soil ---
    println!("\n=== Soil ===");
    let soil = SoilComposition::new(0.4, 0.4, 0.2).unwrap();
    println!("Sand {}, Silt {}, Clay {} → {:?}", soil.sand, soil.silt, soil.clay, soil.texture());

    // --- Weathering ---
    println!("\n=== Weathering ===");
    println!("Physical (25°C range, 60% moisture): {:.3}", physical_weathering_rate(25.0, 0.6));
    println!("Chemical (20°C mean, 1200mm rain): {:.3}", chemical_weathering_rate(20.0, 1200.0));
    println!("Erosion (40mm/h rain, 15° slope, 50% cover): {:.3}", erosion_rate(40.0, 15.0, 0.5));

    // --- Ore Deposits ---
    println!("\n=== Ore Deposits ===");
    let deposit = OreDeposit::new("Gold", DepositType::Vein, 0.01, 200.0, 50_000.0).unwrap();
    println!("{} {:?} deposit: grade {}, depth {}m, tonnage {}t",
        deposit.mineral, deposit.deposit_type, deposit.grade, deposit.depth_m, deposit.tonnage);
    let viable = is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0);
    println!("High-grade deposit economically viable? {viable}");
}
