# Architecture

## Module Map

| Module | Feature | Lines | Key Types | Purpose |
|--------|---------|-------|-----------|---------|
| `crystal` | always | 74 | `CrystalSystem` | 7 crystal system classifications with symmetry order |
| `crystallography` | always | 523 | `UnitCell`, `MillerIndex` | Unit cell geometry, Bragg diffraction, d-spacing |
| `dating` | always | 501 | `IsotopeSystem`, `IsochronPoint` | Radiometric dating (U-Pb, K-Ar, Rb-Sr, C-14), isochron ages |
| `error` | always | 36 | `KhanijError`, `Result<T>` | Shared error types via thiserror |
| `formula` | always | 420 | `Formula` | Mineral formula parser (e.g. `"CaCO3"` to element-count map) |
| `geochemistry` | always | 653 | `MajorOxides`, `TasClassification`, `AsiClassification` | Major oxide analysis, TAS classification, Mg#, Rayleigh fractionation |
| `glaciology` | always | 448 | `GlacierType` | Glen's flow law, basal sliding, mass balance, isostatic adjustment |
| `grid` | always | 389 | `GeologicGrid`, `GeologicLayer`, `GeologicUnit`, `StratigraphicColumn`, `StrikeDip` | 3D geologic grid, stratigraphic columns, structural orientation |
| `hydrothermal` | always | 248 | `HydrothermalConditions`, `AlterationZone` | Hydrothermal ore formation, metal solubility, alteration zones |
| `mineral` | always | 615 | `Mineral`, `MohsHardness`, `Luster` | Mineral properties, hardness scale, luster classification |
| `ore` | always | 450 | `OreDeposit`, `DepositType`, `ResourceCategory`, `TonnageGradePoint` | Ore deposit modeling, cutoff grade, NPV, tonnage-grade curves |
| `rock` | always | 440 | `Rock`, `RockType`, `GeologicalProcess` | Rock classification, bulk density, porosity, rock cycle transitions |
| `sediment` | always | 348 | `SedimentSource`, `SedimentSink`, `BudgetResult` | Source-to-sink sediment budget, transport capacity, denudation rate |
| `soil` | always | 992 | `SoilProfile`, `SoilHorizon`, `SoilComposition`, `SoilTexture`, `SoilOrder`, `SoilFertility`, `SoilPhClass`, `HorizonType` | USDA soil classification, texture, hydraulic conductivity, CEC |
| `stratigraphy` | always | 436 | `SystemsTract`, `SeaLevelCycle`, `DepositionalEnvironment`, `WalthersLaw`, `ParasequenceBoundary` | Sequence stratigraphy, sea-level cycles, accommodation space |
| `tectonics` | always | 406 | `BoundaryType`, `EulerPole`, `SubductionZone`, `RidgeType` | Plate tectonics, Euler poles, spreading rates, ocean floor age |
| `texture` | always | 414 | `GrainSize`, `IgneousTexture`, `MetamorphicFabric`, `Roundness`, `Sorting` | Petrographic texture, Wentworth scale, phi/mm conversion |
| `timescale` | always | 714 | `Eon`, `Era`, `Period`, `Epoch`, `TimeInterval`, `StratigraphicPosition` | Geologic timescale with absolute age ranges (Ma) |
| `volcanic` | always | 481 | `Vei`, `MagmaType`, `MagmaComposition` | VEI classification, magma viscosity, eruption columns, pyroclastic flows |
| `weathering` | always | 549 | (free functions) | Physical/chemical weathering rates, erosion modeling |
| `stability` | `chemistry` | 327 | (free functions) | Gibbs free energy, mineral polymorph stability via kimiya |
| `geothermal` | `thermodynamics` | 549 | `MetamorphicFacies` | Heat flow, thermal gradients, metamorphic facies via ushma |
| `hydrology` | `fluids` | 780 | `TransportRegime` | Darcy flow, Shields parameter, Hjulstrom, well drawdown via pravash |
| `rock_mechanics` | `mechanics` | 985 | `FailureMode` | Seismic velocities, Mohr-Coulomb failure, slope stability via dravya |
| `logging` | `logging` | 5 | (init function) | tracing-subscriber setup with `KHANIJ_LOG` env filter |

**Total**: ~11,950 lines across 25 source files (+ `lib.rs`).

## Design Principles

- **Flat library crate** -- no workspace, no internal binaries. All modules live under `src/`.
- **Feature-gating for optional AGNOS dependencies** -- consumers pull only what they need. The core 20 modules are always available; 5 modules require feature flags.
- **f64 precision throughout** -- geology and geophysics demand double precision for age calculations, pressure-temperature modeling, and coordinate transforms.
- **`#[must_use]` on pure functions** -- compile-time enforcement that callers use computed results.
- **`#[non_exhaustive]` on enums** -- all public enums (`CrystalSystem`, `RockType`, `DepositType`, `BoundaryType`, `SoilTexture`, `Vei`, etc.) are non-exhaustive, allowing future variants without breaking changes.
- **All public types derive `Serialize` + `Deserialize`** -- every struct and enum is serde-ready for JSON/TOML/etc. interchange.
- **hisab for numerical methods** -- integration, interpolation, and special functions come from the AGNOS math crate. No hand-rolled numerics.
- **tracing for instrumentation** -- structured logging on multi-step computations.
- **`Option<T>` over panics** -- fallible operations return `Option` or `Result` rather than panicking. `MohsHardness::new` returns `Option`, `Formula::parse` returns `Result`, lookup functions return `Option`.
- **thiserror for errors** -- `KhanijError` variants use thiserror derive, keeping error handling idiomatic.

## Data Flow

```
               Core types                          Processes
          ┌──────────────┐               ┌─────────────────────┐
          │  mineral      │               │  weathering          │
          │  crystal      │──────────────>│  sediment            │
          │  rock         │               │  volcanic            │
          │  soil         │               │  tectonics           │
          │  formula      │               │  glaciology          │
          └──────────────┘               │  hydrothermal        │
                                          └─────────────────────┘
                                                    │
          ┌──────────────┐                          │
          │  Time/Class   │                          │
          │  timescale    │<─────────────────────────┘
          │  dating       │
          │  texture      │
          │  geochemistry │
          │  stratigraphy │
          └──────────────┘

          ┌──────────────┐
          │  Spatial       │
          │  grid          │
          └──────────────┘

          ┌──────────────────────────────────────────┐
          │  Feature-gated extensions                  │
          │                                            │
          │  stability     ← chemistry  (kimiya)       │
          │  geothermal    ← thermodynamics (ushma)    │
          │  hydrology     ← fluids (pravash)          │
          │  rock_mechanics← mechanics (dravya)        │
          │  weathering*   ← weather (badal)           │
          └──────────────────────────────────────────┘

  * weathering has both always-on functions and weather-gated extensions
```

All modules import `crate::error`. Core modules do not import each other. Feature-gated modules extend core functionality by combining khanij's domain types with external AGNOS crate capabilities.

## Feature Independence

Each optional feature can be enabled independently. No cross-feature imports exist:

- `chemistry` enables `stability` and extends `mineral` and `weathering` with kimiya thermochemistry -- does not require `thermodynamics` or `fluids`.
- `thermodynamics` enables `geothermal` with ushma heat transfer -- does not require `chemistry` or `fluids`.
- `fluids` enables `hydrology` with pravash fluid dynamics -- does not require `chemistry` or `thermodynamics`.
- `mechanics` enables `rock_mechanics` with dravya material science -- does not require any other optional feature.
- `weather` extends `weathering` with badal atmospheric state -- does not require any other optional feature.
- `logging` enables tracing-subscriber output -- orthogonal to all science features.

A consumer can use `--features chemistry` without pulling in ushma, pravash, dravya, or badal, and vice versa. The `hydrothermal` module is always available despite its name -- it does not require any feature flag.

Default features: none. All optional features are opt-in.

## Dependencies

### Required

| Crate | Purpose |
|-------|---------|
| `hisab` | Numerical methods -- integration, interpolation, special functions (AGNOS crate) |
| `serde` | Serialization for all public types |
| `thiserror` | Derive `Error` for `KhanijError` |
| `tracing` | Structured instrumentation |

### Optional (feature-gated)

| Crate | Feature | Purpose |
|-------|---------|---------|
| `kimiya` | `chemistry` | Thermochemistry database, Gibbs energy, dissolution kinetics (AGNOS crate) |
| `ushma` | `thermodynamics` | Heat transfer, thermal state functions (AGNOS crate) |
| `pravash` | `fluids` | Fluid dynamics, buoyancy, shallow-water modeling (AGNOS crate) |
| `dravya` | `mechanics` | Material properties, stress tensors, failure criteria (AGNOS crate) |
| `badal` | `weather` | Atmospheric state, climate parameters (AGNOS crate) |
| `tracing-subscriber` | `logging` | Log output formatting and env-based filtering |

### Dev-only

| Crate | Purpose |
|-------|---------|
| `criterion` | Benchmarking |
| `serde_json` | Test serialization round-trips |
