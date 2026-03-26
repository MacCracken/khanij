use criterion::{criterion_group, criterion_main, Criterion};
use khanij::{
    Mineral, MohsHardness, Rock, RockType, GeologicalProcess,
    SoilComposition, rock_cycle_next,
    physical_weathering_rate, chemical_weathering_rate, erosion_rate,
    is_economically_viable,
};

fn bench_mineral_presets(c: &mut Criterion) {
    c.bench_function("mineral_preset_quartz", |b| b.iter(Mineral::quartz));
    c.bench_function("mineral_preset_diamond", |b| b.iter(Mineral::diamond));
}

fn bench_mohs_hardness(c: &mut Criterion) {
    let diamond = MohsHardness::new(10.0).unwrap();
    let quartz = MohsHardness::new(7.0).unwrap();
    c.bench_function("mohs_scratches", |b| b.iter(|| diamond.scratches(&quartz)));
    c.bench_function("mohs_new_valid", |b| b.iter(|| MohsHardness::new(5.5)));
}

fn bench_rock_cycle(c: &mut Criterion) {
    c.bench_function("rock_cycle_next", |b| {
        b.iter(|| rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering))
    });
    c.bench_function("rock_new_validated", |b| {
        b.iter(|| Rock::new("Test", RockType::Igneous, 2.7, 0.05, vec!["Quartz".into()]))
    });
}

fn bench_soil(c: &mut Criterion) {
    c.bench_function("soil_composition_new", |b| {
        b.iter(|| SoilComposition::new(0.4, 0.4, 0.2))
    });
    let soil = SoilComposition::new(0.4, 0.4, 0.2).unwrap();
    c.bench_function("soil_texture_classify", |b| b.iter(|| soil.texture()));
}

fn bench_weathering(c: &mut Criterion) {
    c.bench_function("physical_weathering_rate", |b| {
        b.iter(|| physical_weathering_rate(25.0, 0.6))
    });
    c.bench_function("chemical_weathering_rate", |b| {
        b.iter(|| chemical_weathering_rate(20.0, 1200.0))
    });
    c.bench_function("erosion_rate", |b| {
        b.iter(|| erosion_rate(40.0, 15.0, 0.5))
    });
}

fn bench_ore(c: &mut Criterion) {
    c.bench_function("is_economically_viable", |b| {
        b.iter(|| is_economically_viable(0.05, 1_000_000.0, 5000.0, 100_000_000.0))
    });
}

criterion_group!(
    benches,
    bench_mineral_presets,
    bench_mohs_hardness,
    bench_rock_cycle,
    bench_soil,
    bench_weathering,
    bench_ore,
);
criterion_main!(benches);
