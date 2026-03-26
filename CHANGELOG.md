# Changelog

All notable changes to khanij are documented in this file.

## [Unreleased]

### Changed

- **f32 → f64 standardization** — all numeric types unified to `f64` across
  `mineral.rs`, `rock.rs`, `soil.rs`, `weathering.rs`, and `ore.rs` for
  consistent precision. Breaking change from 0.1.0.
- **`fractional_crystallization()`** now returns `Option<f64>` instead of
  panicking on invalid `f_remaining`. Breaking change from 0.1.0.
- **`from_vickers()`** now uses `hisab::num::bisection` instead of hand-rolled
  binary search.
- **`well_function()`** now uses `hisab::calc::integral_gauss_legendre` with
  log-substitution instead of manual series expansion.

### Added

- **291 doc-tests** across all 24 modules (up from 8). Every public function,
  struct, and enum now has runnable examples.
- **111 benchmarks** covering all modules including feature-gated ones (up from
  19 covering 7 modules).
- **7 new integration tests** — serde roundtrips for 11 types, cross-module
  workflows (mineral→formula pipeline, crystallography→Bragg, timescale
  consistency, tectonics ocean floor, sediment budget, volcanic eruption
  cascade).
- **3 new examples** — `dating.rs` (radiometric dating workflow),
  `rock_cycle.rs` (full cycle simulation), `ore_deposit.rs` (economics
  evaluation).
- **`GRAIN_CLASSES`** and **`GRAIN_DIAMETERS`** re-exported from `lib.rs`.
- **README.md** — comprehensive crate documentation with quick-start, feature
  matrix, and module overview.
- **CONTRIBUTING.md** — contributor guide with workflow, code style, and module
  checklist.
- **SECURITY.md** — security policy with SLA table, GitHub Advisory reporting,
  and coordinated disclosure.
- **CODE_OF_CONDUCT.md** — Contributor Covenant 2.1 with enforcement details.
- **docs/architecture/overview.md** — module map, design principles, data flow,
  feature independence, dependency inventory.
- **docs/development/roadmap.md** — scope, completed work, P1/P2/P3 backlog,
  consumer crates, crate boundaries.
- **benchmarks.md** — benchmark tracking baseline.
- CI: bench job, coverage threshold (80%), multi-target release builds.

### Quality

- 693 tests (374 unit + 28 integration + 291 doc-tests), 0 failures
- 111 criterion benchmarks, all modules covered
- 0 clippy warnings, 0 doc warnings
- codecov target: 80% project, 75% patch

## [0.1.0] — 2026-03-26

Initial release of the khanij geology and mineralogy engine.

### Core modules (always available)

- **mineral** — 15 mineral presets (full Mohs reference scale: talc through diamond, plus olivine, pyrite, magnetite, halite, gypsum, muscovite), `MohsHardness` newtype with Vickers/Knoop conversion, `Luster` enum
- **formula** — chemical formula parser supporting standard notation, parenthesized groups, Unicode subscripts, hydrates (`CaSO4·2H2O`), and solid solutions (`(Mg,Fe)2SiO4`)
- **crystal** — 7 crystal systems with symmetry orders
- **crystallography** — unit cell parameters, Miller indices, d-spacing calculation, Bragg's law, 4 preset unit cells (halite, quartz, calcite, diamond)
- **rock** — 13 rock presets (granite, basalt, obsidian, rhyolite, limestone, sandstone, shale, conglomerate, marble, slate, gneiss, quartzite, schist), validated constructors, bulk density calculator, `GeologicalProcess` enum for typed rock cycle transitions
- **soil** — full USDA soil texture triangle (12 classes), 12 soil orders with fertility/environment, 6 horizon types, `SoilProfile` with order classification, pH classification, CEC, available water capacity, hydraulic conductivity
- **ore** — `OreDeposit` with validation, `ResourceCategory` (Measured/Indicated/Inferred), cutoff grade, tonnage-grade curves, net present value, economic viability with diminishing returns
- **weathering** — physical and chemical weathering rates via hisab numerical integration, RUSLE erosion model, freeze-thaw cycling
- **sediment** — source-to-sink sediment budget, stream power transport capacity, sediment delivery ratio, denudation rate
- **timescale** — 4 eons, 3 eras, 12 periods, 7 Cenozoic epochs with ICS 2023 boundaries, `classify_age()` for full stratigraphic position
- **dating** — 7 isotope systems (U-Pb, K-Ar, Rb-Sr, C-14, Sm-Nd, Lu-Hf), isochron dating with linear regression, closure temperatures for common mineral-system pairs
- **tectonics** — Euler poles with velocity calculation, ridge classification (ultra-slow to ultra-fast), subduction zone geometry, ocean floor age, lithosphere thickness, ocean depth from plate age
- **texture** — Wentworth/Udden grain size scale with phi conversion, Folk & Ward sorting, Powers roundness, igneous texture classification, metamorphic fabric types
- **geochemistry** — TAS diagram classification (16 rock types), Mg#, alumina saturation index, Rayleigh fractional crystallization, `MajorOxides` with validation
- **volcanic** — VEI scale (0-8), magma viscosity from composition, eruption column height (Sparks 1986), pyroclastic flow runout, Jeffreys lava flow velocity, magma type classification
- **hydrothermal** — alteration zone classification (potassic/phyllic/argillic/propylitic), metal solubility and precipitation rate models, ore grade estimation from fluid focusing
- **grid** — 2D geologic grid with `GeologicUnit` cells, `StrikeDip` with dip direction, `StratigraphicColumn` with layer stacking and depth lookup
- **stratigraphy** — systems tracts (LST/TST/HST/FSST), sea level cycles, accommodation space, sediment supply ratio, Walther's law lateral facies equivalents
- **glaciology** — Glen's flow law, Weertman basal sliding, mass balance, ELA, isostatic depression and rebound, depth-integrated ice velocity
- **error** — `KhanijError` with `thiserror`, 4 variants

### Optional features

- **`chemistry`** (kimiya) — `Mineral::molecule()` and `molecular_weight()` via formula parser, `dissolution_rate()` with Arrhenius kinetics, `lattice_energy()` (Born-Lande), `ionic_radius()` lookup, weathering reaction products for 6 minerals, mineral stability module with Gibbs energy, reaction thermodynamics, equilibrium temperature
- **`thermodynamics`** (ushma) — geothermal heat flux, temperature-at-depth, thermal diffusivity, heat storage, lithostatic pressure, Gibbs free energy, volatile pressure, metamorphic facies classification (7 facies), intrusion cooling model, contact aureole temperature
- **`fluids`** (pravash) — Stokes settling velocity, Hjulstrom curve (erosion/deposition thresholds), Shields parameter, flow regime classification, Darcy groundwater flow, Theis well drawdown, Cooper-Jacob approximation, radius of influence, brine and sediment-laden fluid presets, SPH simulation setup
- **`mechanics`** (dravya) — seismic P-wave and S-wave velocities, Vp/Vs ratio, Poisson's ratio from velocities, velocity-depth profiles with temperature correction, Mohr-Coulomb failure criterion and safety factor, Drucker-Prager conversion, brittle-ductile transition depth, infinite slope stability, 8 rock material presets, weathered material degradation model
- **`weather`** (badal) — climate-driven weathering from `AtmosphericState` (physical, chemical, erosion), freeze-thaw cycle estimation, combined weathering intensity index
- **`logging`** — tracing-subscriber initialization via `KHANIJ_LOG` env var

### Infrastructure

- 395 tests (374 unit + 21 integration)
- Criterion benchmarks for minerals, hardness, rock cycle, soil, weathering, ore economics, formula parser
- Working example (`examples/basic.rs`)
- `cargo fmt`, `cargo clippy`, `cargo audit`, `cargo deny`, `cargo doc` all clean
- GPL-3.0-only license
- Rust 2024 edition, MSRV 1.89
