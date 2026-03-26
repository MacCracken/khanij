# Changelog

All notable changes to khanij are documented in this file.

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
