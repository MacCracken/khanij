use khanij::*;

fn main() {
    // --- Carbon-14 Dating ---
    println!("=== Carbon-14 Dating: Charcoal from Archaeological Site ===");
    let fraction = 0.35;
    let age = c14_age(fraction).unwrap();
    println!("Sample retains {:.0}% of modern C-14", fraction * 100.0);
    println!("Conventional radiocarbon age: {age:.0} years BP");
    let check = c14_fraction_remaining(age);
    println!("Verification: at {age:.0} yr, fraction remaining = {check:.4}");
    let pos = classify_age(age / 1e6);
    println!(
        "Epoch: {:?} (well within the Quaternary)",
        pos.epoch.unwrap()
    );

    // --- U-Pb Zircon Geochronology ---
    println!("\n=== U-Pb Zircon Dating ===");
    let u_pb = IsotopeSystem::U238Pb206;
    let lambda = u_pb.decay_constant();
    let t_half = u_pb.half_life_years();
    println!("U-238 decay constant: {lambda:.4e} yr^-1");
    println!("U-238 half-life: {:.3} Ga", t_half / 1e9);
    let (range_min, range_max) = u_pb.useful_range();
    println!(
        "Useful range: {:.0} Ma - {:.0} Ma",
        range_min / 1e6,
        range_max / 1e6
    );

    let zircon_ratio = 0.18; // Pb206/U238 measured in zircon
    let zircon_age = u_pb.age(zircon_ratio).unwrap();
    println!(
        "Zircon Pb206/U238 = {zircon_ratio} -> age = {:.1} Ga",
        zircon_age / 1e9
    );

    let tc = closure_temperature(IsotopeSystem::U238Pb206, "zircon").unwrap();
    println!("Zircon U-Pb closure temperature: {tc:.0} C");
    let tc_ap = closure_temperature(IsotopeSystem::U238Pb206, "apatite").unwrap();
    println!("Apatite U-Pb closure temperature: {tc_ap:.0} C (lower = later closure)");

    // --- Rb-Sr Isochron Dating ---
    println!("\n=== Rb-Sr Isochron Dating: Granodiorite Whole-Rock ===");
    let rb_sr = IsotopeSystem::Rb87Sr87;
    let target_age = 1.2e9;
    let slope = (rb_sr.decay_constant() * target_age).exp() - 1.0;
    let initial_sr = 0.7040;
    let points = vec![
        IsochronPoint {
            x: 0.2,
            y: initial_sr + slope * 0.2,
        },
        IsochronPoint {
            x: 0.8,
            y: initial_sr + slope * 0.8,
        },
        IsochronPoint {
            x: 2.5,
            y: initial_sr + slope * 2.5,
        },
        IsochronPoint {
            x: 5.0,
            y: initial_sr + slope * 5.0,
        },
    ];
    println!("Analysed 4 mineral separates (87Rb/86Sr vs 87Sr/86Sr):");
    for (i, p) in points.iter().enumerate() {
        println!("  Sample {}: x = {:.3}, y = {:.6}", i + 1, p.x, p.y);
    }
    let (iso_age, init) = isochron_age(rb_sr, &points).unwrap();
    println!("Isochron age: {:.2} Ga", iso_age / 1e9);
    println!("Initial 87Sr/86Sr: {init:.4}");

    // --- Closure Temperature Summary ---
    println!("\n=== Closure Temperature Reference ===");
    let systems = [
        (IsotopeSystem::K40Ar40, "hornblende"),
        (IsotopeSystem::K40Ar40, "biotite"),
        (IsotopeSystem::Rb87Sr87, "muscovite"),
        (IsotopeSystem::Sm147Nd143, "garnet"),
    ];
    for (sys, mineral) in systems {
        let tc = closure_temperature(sys, mineral).unwrap();
        println!("  {mineral:12} ({sys:?}): {tc:.0} C");
    }
    println!("\nCooling history reconstructed from multiple thermochronometers.");
}
