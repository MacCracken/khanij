use khanij::*;

fn main() {
    // --- Create the Deposit ---
    println!("=== Gold Vein Deposit: Project Evaluation ===");
    let deposit = OreDeposit::new("Gold", DepositType::Vein, 0.008, 150.0, 500_000.0).unwrap();
    println!("Mineral:      {}", deposit.mineral);
    println!("Type:         {:?}", deposit.deposit_type);
    println!("Grade:        {:.2}% ({:.1} g/t)", deposit.grade * 100.0, deposit.grade * 1e6);
    println!("Tonnage:      {:.0} t", deposit.tonnage);
    println!("Depth:        {:.0} m", deposit.depth_m);
    println!("Contained Au: {:.1} t", deposit.contained_metal());
    println!("Strip ratio:  {:.1}:1", deposit.stripping_ratio());

    // --- Economic Viability ---
    println!("\n=== Economic Viability ===");
    let gold_price = 60_000_000.0; // $/tonne of pure gold
    let extraction_cost = 150_000_000.0;
    let viable = is_economically_viable(deposit.grade, deposit.tonnage, gold_price, extraction_cost);
    let revenue = deposit.gross_revenue(gold_price);
    println!("Gold price:       ${:.0}/oz (${:.0}M/t)", gold_price / 32150.0, gold_price / 1e6);
    println!("Gross revenue:    ${:.1}M", revenue / 1e6);
    println!("Extraction cost:  ${:.1}M", extraction_cost / 1e6);
    println!("Viable:           {}", if viable { "YES" } else { "NO" });

    // --- Cutoff Grade ---
    println!("\n=== Cutoff Grade Analysis ===");
    let mining_cost_per_t = 80.0;
    let recovery = 0.92;
    let cog = cutoff_grade(gold_price, mining_cost_per_t, recovery).unwrap();
    println!("Mining cost:   ${mining_cost_per_t}/t ore");
    println!("Recovery:      {:.0}%", recovery * 100.0);
    println!("Cutoff grade:  {:.4} g/t", cog * 1e6);
    println!("Deposit grade is {:.0}x the cutoff", deposit.grade / cog);

    // --- NPV ---
    println!("\n=== Net Present Value ===");
    let mine_life = 12.0;
    let annual_rev = revenue / mine_life;
    let annual_cost = extraction_cost / mine_life;
    let discount = 0.08;
    let npv = net_present_value(annual_rev, annual_cost, discount, mine_life).unwrap();
    println!("Mine life:      {mine_life:.0} years");
    println!("Discount rate:  {:.0}%", discount * 100.0);
    println!("Annual revenue: ${:.1}M", annual_rev / 1e6);
    println!("Annual cost:    ${:.1}M", annual_cost / 1e6);
    println!("NPV:            ${:.1}M", npv / 1e6);

    // --- Tonnage-Grade Curve ---
    println!("\n=== Tonnage-Grade Curve ===");
    let blocks = vec![
        (150_000.0, 0.004), (120_000.0, 0.007), (100_000.0, 0.010),
        (80_000.0, 0.015),  (30_000.0, 0.025),  (20_000.0, 0.040),
    ];
    let curve = tonnage_grade_curve(&blocks, 5);
    println!("{:>10}  {:>12}  {:>10}", "Cutoff", "Tonnage", "Avg Grade");
    for pt in &curve {
        println!(
            "{:>9.4}%  {:>10.0} t  {:>9.4}%",
            pt.cutoff_grade * 100.0, pt.tonnage_above_cutoff, pt.average_grade_above_cutoff * 100.0
        );
    }

    // --- Hydrothermal Context ---
    println!("\n=== Hydrothermal Alteration Context ===");
    let temps = [550.0, 400.0, 300.0, 200.0, 100.0];
    for t in temps {
        let zone = classify_alteration(t);
        let grade = estimated_ore_grade(1e-6, t, 300.0, 0.1, 0.001);
        println!("  {t:5.0} C -> {:?}, estimated grade {:.4}%", zone, grade * 100.0);
    }
    println!("\nGold precipitates near 300 C in the argillic-phyllic transition zone.");
}
