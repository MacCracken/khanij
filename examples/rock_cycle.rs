use khanij::*;

fn main() {
    // --- Create Rocks from All Three Categories ---
    println!("=== Rock Inventory ===");
    let granite = Rock::granite();
    let sandstone = Rock::sandstone();
    let marble = Rock::marble();
    for rock in [&granite, &sandstone, &marble] {
        println!(
            "{:10} | {:?} | density {:.2} g/cm3 | porosity {:.1}%",
            rock.name,
            rock.rock_type,
            rock.density,
            rock.porosity * 100.0
        );
    }

    // --- Full Rock Cycle ---
    println!("\n=== The Rock Cycle ===");
    let mut current = RockType::Igneous;
    let transitions = [
        ("Weathering & erosion", GeologicalProcess::Weathering),
        ("Burial & metamorphism", GeologicalProcess::Metamorphism),
        ("Subduction & melting", GeologicalProcess::Melting),
    ];
    println!("Starting: {:?} (granite)", current);
    for (description, process) in &transitions {
        let next = rock_cycle_next(current, *process).unwrap();
        println!("  -> {description} -> {:?}", next);
        current = next;
    }
    println!("Cycle complete: back to {:?}", current);

    // --- Cross-Transitions ---
    println!("\n=== Alternative Paths ===");
    let alt = rock_cycle_next(RockType::Igneous, GeologicalProcess::Metamorphism).unwrap();
    println!(
        "Igneous + direct metamorphism -> {:?} (e.g., granite to gneiss)",
        alt
    );
    let alt2 = rock_cycle_next(RockType::Metamorphic, GeologicalProcess::Weathering).unwrap();
    println!(
        "Metamorphic + weathering -> {:?} (e.g., marble to limestone)",
        alt2
    );

    // --- Bulk Density & Porosity ---
    println!("\n=== Density & Porosity Analysis ===");
    let grain_density = 2.65;
    let measured_bulk = 2.25;
    let phi = porosity_from_density(measured_bulk, grain_density);
    println!("Measured bulk density: {measured_bulk} g/cm3, grain density: {grain_density} g/cm3");
    println!("Computed porosity: {:.1}%", phi * 100.0);
    let saturated = bulk_density(grain_density, phi, 1.0);
    let dry = bulk_density(grain_density, phi, 0.001);
    println!("Water-saturated bulk density: {saturated:.3} g/cm3");
    println!("Air-dry bulk density: {dry:.3} g/cm3");

    // --- Mineral Mixture ---
    let minerals = [(2.65, 0.30), (2.56, 0.60), (2.82, 0.10)];
    let mix_bd = bulk_density_from_minerals(&minerals, 0.01, 1.0);
    println!("Granite mineral mix (qtz 30%, fsp 60%, mica 10%): {mix_bd:.3} g/cm3");

    // --- Weathering Rates ---
    println!("\n=== Weathering Rates ===");
    let phys = physical_weathering_rate(30.0, 0.7);
    let chem = chemical_weathering_rate(25.0, 1500.0);
    println!("Physical weathering (30 C range, 70% moisture): {phys:.3}");
    println!("Chemical weathering (25 C mean, 1500 mm rain): {chem:.3}");
    let bare = erosion_rate(50.0, 20.0, 0.0);
    let vegetated = erosion_rate(50.0, 20.0, 0.8);
    println!("Erosion (bare slope 20 deg): {bare:.2} -> sediment produced");
    println!("Erosion (vegetated slope):   {vegetated:.2} -> 80% cover reduces loss");

    // --- Grain Size Classification ---
    println!("\n=== Weathering Products: Grain Size ===");
    let sizes_mm = [0.002, 0.05, 0.3, 2.5, 70.0];
    for size in sizes_mm {
        let class = classify_grain_size(size);
        println!("  {size:6.3} mm -> {:?}", class);
    }
    println!("\nWeathering breaks rock into progressively finer sediment over time.");
}
