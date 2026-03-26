# Khanij

**Khanij** (खनिज — Sanskrit for "mineral, born from mining") is a geology and
mineralogy engine for the [AGNOS](https://github.com/MacCracken) scientific
computing ecosystem.

It provides mineral properties, crystal systems, rock classification, soil
composition, radiometric dating, tectonics, volcanic processes, ore deposit
modeling, and more — all built on the [hisab](https://crates.io/crates/hisab)
numerical library.

## Quick Start

Add khanij to your `Cargo.toml`:

```toml
[dependencies]
khanij = "0.1"
```

```rust
use khanij::*;

// Mineral identification
let quartz = Mineral::quartz();
assert!(quartz.hardness.scratches(&MohsHardness::new(5.0).unwrap()));

// Formula parsing (handles Unicode subscripts, hydrates, solid solutions)
let f = Formula::parse("CaSO4·2H2O").unwrap(); // gypsum
assert_eq!(f.count("Ca"), 1);
assert_eq!(f.count("O"), 6);

// Rock cycle
let next = rock_cycle_next(RockType::Igneous, GeologicalProcess::Weathering);
assert_eq!(next, Some(RockType::Sedimentary));

// Radiometric dating
let age = c14_age(0.5); // 50% modern carbon remaining
assert!((age - 5730.0).abs() < 1.0); // ~one half-life

// Geologic timescale
assert_eq!(period_at_age(150.0), Some(Period::Jurassic));
```

## Optional Features

Enable optional integrations with other AGNOS crates:

| Feature | Dependency | Capabilities |
|---------|-----------|--------------|
| `chemistry` | [kimiya](https://crates.io/crates/kimiya) | Mineral composition, dissolution kinetics, lattice energy, Gibbs stability |
| `thermodynamics` | [ushma](https://crates.io/crates/ushma) | Geothermal gradients, heat flow, metamorphic facies |
| `fluids` | [pravash](https://crates.io/crates/pravash) | Groundwater flow, sediment transport, Hjulstrom curves, Shields parameter |
| `mechanics` | [dravya](https://crates.io/crates/dravya) | Mohr-Coulomb failure, seismic velocities, slope stability |
| `weather` | [badal](https://crates.io/crates/badal) | Climate-driven weathering and erosion |
| `logging` | tracing-subscriber | Runtime logging via `KHANIJ_LOG` env filter |

```toml
[dependencies]
khanij = { version = "0.1", features = ["chemistry", "fluids"] }
```

## Modules

| Module | Domain |
|--------|--------|
| `mineral` | Mineral properties, Mohs/Vickers/Knoop hardness |
| `crystal` | Crystal system classification (7 systems) |
| `crystallography` | Unit cells, Miller indices, Bragg diffraction |
| `formula` | Chemical formula parser (Unicode, hydrates, solid solutions) |
| `rock` | Rock types, bulk density, porosity, rock cycle |
| `soil` | USDA texture classification, horizons, fertility, hydraulic conductivity |
| `texture` | Wentworth grain size, phi scale, sorting, igneous/metamorphic fabrics |
| `geochemistry` | TAS classification, ASI, Mg#, Rayleigh fractionation |
| `volcanic` | VEI, magma viscosity, eruption columns, pyroclastic flows |
| `weathering` | Physical/chemical weathering rates, erosion |
| `sediment` | Sediment production, transport capacity, budgets |
| `stratigraphy` | Systems tracts, sea-level cycles, Walther's Law |
| `tectonics` | Euler poles, spreading rates, subduction geometry, ocean depth |
| `glaciology` | Glen's flow law, mass balance, isostatic rebound |
| `hydrothermal` | Alteration zones, metal solubility, ore grade estimation |
| `ore` | Deposit classification, cutoff grade, NPV, tonnage-grade curves |
| `timescale` | Geologic time (eons, eras, periods, epochs), age classification |
| `dating` | Radiometric dating (U-Pb, K-Ar, Rb-Sr, C-14, Sm-Nd, Lu-Hf), isochrons |
| `grid` | 2D geologic grids, stratigraphic columns |
| `stability` | Gibbs energy, phase stability, reaction spontaneity *(chemistry)* |
| `geothermal` | Heat flux, metamorphic facies, intrusion cooling *(thermodynamics)* |
| `hydrology` | Darcy flow, Theis/Cooper-Jacob, Shields parameter *(fluids)* |
| `rock_mechanics` | Mohr-Coulomb, seismic velocities, slope stability *(mechanics)* |

## Requirements

- **Rust**: 1.89+ (edition 2024)
- **MSRV**: 1.89

## License

GPL-3.0-only. See [LICENSE](LICENSE) for details.
