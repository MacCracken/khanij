# Khanij Roadmap

> **Khanij** is the geology and mineralogy crate. Math foundations come from [hisab](https://github.com/MacCracken/hisab). Chemistry integration is via [kimiya](https://github.com/MacCracken/kimiya).

## 1. Scope

Khanij owns **earth-science simulation**: geology, mineralogy, crystallography, rock mechanics, radiometric dating, ore economics, sedimentology, tectonics, and geothermal processes. It provides the domain models and formulas; consumers decide what to do with them (render terrain, run simulations, grade deposits).

Khanij does NOT own:
- **Math primitives** → hisab (vectors, geometry, numerical methods)
- **Chemistry** → kimiya (reactions, elements, bonding)
- **Fluid dynamics** → pravash (SPH, shallow-water, buoyancy)
- **Solid mechanics** → dravya (stress, strain, material properties)
- **Thermodynamics** → ushma (heat transfer, phase transitions)
- **Weather / climate** → badal (precipitation, erosion forcing)

## 2. Completed — V0.1.0

| Aspect | Detail |
|--------|--------|
| Modules | 24 — crystallography, crystal, dating, formula, geochemistry, geothermal, glaciology, grid, hydrology, hydrothermal, mineral, ore, rock_mechanics, rock, sediment, soil, stability, stratigraphy, tectonics, texture, timescale, volcanic, weathering, logging |
| Integrations | 5 optional AGNOS features (chemistry/kimiya, fluids/pravash, mechanics/dravya, thermodynamics/ushma, weather/badal) + hisab (always-on) |
| Tests | 693 total — 374 unit, 28 integration, 291 doc-tests |
| Benchmarks | 111 (criterion) |
| Examples | 4 |
| CI/CD | Full pipeline |

## 3. Engineering Backlog

### P1 — Post-V0.1 Hardening

- [ ] Expand mineral database (currently ~15 presets → target 50+)
- [ ] Expand rock presets (currently ~13 → target 30+)
- [ ] Property-based testing (proptest) for numerical stability
- [ ] Fuzz testing for formula parser
- [ ] Coverage target: 80%+

### P1 — Consumer Integration Gaps

- [ ] kiran integration for procedural terrain generation from geological models
- [ ] joshua integration for geological simulation scenarios
- [ ] Sediment transport coupling with pravash SPH solver

### P2 — Domain Completeness

- [ ] Petrology module (metamorphic reactions, phase diagrams)
- [ ] Structural geology (faults, folds, stereographic projection)
- [ ] Geophysics (gravity, magnetics, resistivity)
- [ ] Paleontology (fossil classification, biostratigraphy)
- [ ] Economic geology (resource estimation, geostatistics)

### P3 — Advanced / Demand-Gated

- [ ] 3D geological modeling
- [ ] Stratigraphic forward modeling
- [ ] Basin modeling
- [ ] Reactive transport coupling (with kimiya + pravash)

### P3 — Infrastructure

- [ ] `cargo semver-checks` in CI
- [ ] Benchmark regression CI gate
- [ ] Coverage gate in CI (80% threshold)

## 4. Consumer Crates

| Consumer | What it uses |
|----------|-------------|
| **kiran** | Terrain generation from rock/soil types, mineral textures |
| **joshua** | Geological simulation: tectonics, erosion, volcanic events |
| **badal** | Weathering rates as climate feedback, glaciology coupling |
| **pravash** | Sediment properties for transport simulation |
| **kimiya** | Mineral compositions, geochemical reactions |

## 5. Boundary with Other Crates

| Feature | khanij | other |
|---------|--------|-------|
| Mineral hardness, density, crystal system | Yes | — |
| Rock classification & presets | Yes | — |
| Radiometric dating formulas | Yes | — |
| Ore grade & economic valuation | Yes | — |
| Weathering & erosion models | Yes | badal (climate forcing) |
| Geothermal gradient | Yes | ushma (heat transfer math) |
| Soil mechanics (bearing, compaction) | Yes | dravya (stress-strain) |
| Sediment transport in fluid | — | pravash (SPH solver) |
| Chemical element data | — | kimiya |
| Vector / matrix math | — | hisab |
| Grid & spatial indexing | Yes (geological grids) | hisab (geometry primitives) |
| Hydrothermal alteration | Yes | kimiya (reactions), ushma (heat) |
| Volcanic eruption dynamics | Yes (models) | pravash (fluid flow) |
