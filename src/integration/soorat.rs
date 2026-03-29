//! Soorat integration — visualization data structures for geological analysis.
//!
//! Provides structured types that soorat can render: geologic grid maps,
//! crystal unit cells, stratigraphic columns, and strike/dip markers.

use serde::{Deserialize, Serialize};

// ── Geologic grid visualization ────────────────────────────────────────────

/// A 2D geologic map for colored grid/voxel rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeologicGridVisualization {
    /// Rock type ID at each cell (0 = empty). Flattened row-major: `cells[y * nx + x]`.
    pub cells: Vec<u16>,
    /// Age (Ma) at each cell (for color mapping).
    pub ages: Vec<f64>,
    /// Grid dimensions (nx, ny).
    pub dimensions: [usize; 2],
    /// Cell size in metres.
    pub cell_size: f64,
    /// Unique rock type names, indexed by cell value.
    pub rock_types: Vec<String>,
}

impl GeologicGridVisualization {
    /// Create from a khanij `GeologicGrid`.
    #[must_use]
    pub fn from_grid(grid: &crate::grid::GeologicGrid) -> Self {
        let mut rock_types: Vec<String> = vec!["empty".to_string()];
        let mut cells = Vec::with_capacity(grid.nx * grid.ny);
        let mut ages = Vec::with_capacity(grid.nx * grid.ny);

        for y in 0..grid.ny {
            for x in 0..grid.nx {
                if let Some(unit) = grid.get(x, y) {
                    // Find or add rock type
                    let type_idx = rock_types
                        .iter()
                        .position(|t| t == &unit.rock_type)
                        .unwrap_or_else(|| {
                            rock_types.push(unit.rock_type.clone());
                            rock_types.len() - 1
                        });
                    cells.push(type_idx as u16);
                    ages.push(unit.age_ma);
                } else {
                    cells.push(0);
                    ages.push(0.0);
                }
            }
        }

        Self {
            cells,
            ages,
            dimensions: [grid.nx, grid.ny],
            cell_size: grid.cell_size_m,
            rock_types,
        }
    }
}

// ── Crystal structure visualization ────────────────────────────────────────

/// Crystal unit cell for 3D wireframe rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrystalVisualization {
    /// Lattice vectors `[a, b, c]` — each is `[x, y, z]` in angstroms.
    pub lattice_vectors: [[f64; 3]; 3],
    /// Atom positions within the unit cell (fractional coordinates `[u, v, w]`).
    pub atom_positions: Vec<[f64; 3]>,
    /// Atomic number for each atom (for color/radius).
    pub atom_types: Vec<u8>,
    /// Crystal system name.
    pub system: String,
}

impl CrystalVisualization {
    /// Create from unit cell parameters and crystal system.
    ///
    /// Converts cell parameters (a, b, c, angles) to Cartesian lattice vectors.
    #[must_use]
    pub fn from_unit_cell(
        cell: &crate::crystallography::UnitCell,
        system: crate::crystal::CrystalSystem,
    ) -> Self {
        let a = cell.a;
        let b = cell.b;
        let c = cell.c;
        let beta = cell.beta.to_radians();
        let gamma = cell.gamma.to_radians();

        let cos_alpha = cell.alpha.to_radians().cos();
        let cos_beta = beta.cos();
        let cos_gamma = gamma.cos();
        let sin_gamma = gamma.sin();

        let va = [a, 0.0, 0.0];
        let vb = [b * cos_gamma, b * sin_gamma, 0.0];
        let cx = c * cos_beta;
        let cy = if sin_gamma.abs() > 1e-10 {
            c * (cos_alpha - cos_beta * cos_gamma) / sin_gamma
        } else {
            0.0
        };
        let cz_sq = c * c - cx * cx - cy * cy;
        let cz = if cz_sq > 0.0 { cz_sq.sqrt() } else { 0.0 };
        let vc = [cx, cy, cz];

        Self {
            lattice_vectors: [va, vb, vc],
            atom_positions: Vec::new(),
            atom_types: Vec::new(),
            system: format!("{system:?}"),
        }
    }
}

// ── Stratigraphic column visualization ─────────────────────────────────────

/// Stratigraphic column for stacked-bar rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StratColumnVisualization {
    /// Layers from bottom to top.
    pub layers: Vec<StratLayerViz>,
    /// Total column height in metres.
    pub total_height: f64,
}

/// A single stratigraphic layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StratLayerViz {
    /// Layer name.
    pub name: String,
    /// Rock type.
    pub rock_type: String,
    /// Thickness in metres.
    pub thickness: f64,
    /// Age at base (Ma).
    pub age_base_ma: f64,
    /// Age at top (Ma).
    pub age_top_ma: f64,
}

impl StratColumnVisualization {
    /// Create from a list of layer descriptions.
    #[must_use]
    pub fn from_layers(layers: Vec<StratLayerViz>) -> Self {
        let total_height = layers.iter().map(|l| l.thickness).sum();
        Self {
            layers,
            total_height,
        }
    }
}

// ── Strike/dip markers ─────────────────────────────────────────────────────

/// Strike/dip measurement at a map position for symbol rendering.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StrikeDipMarker {
    /// Map position `[x, z]` in metres.
    pub position: [f64; 2],
    /// Strike azimuth in degrees (0–360).
    pub strike_deg: f64,
    /// Dip angle in degrees (0–90).
    pub dip_deg: f64,
    /// Dip direction in degrees (strike + 90).
    pub dip_direction_deg: f64,
}

impl StrikeDipMarker {
    /// Create from a position and `StrikeDip` measurement.
    #[must_use]
    pub fn new(position: [f64; 2], sd: &crate::grid::StrikeDip) -> Self {
        Self {
            position,
            strike_deg: sd.strike_deg,
            dip_deg: sd.dip_deg,
            dip_direction_deg: sd.dip_direction(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geologic_grid_empty() {
        let grid = crate::grid::GeologicGrid::new(3, 3, 10.0);
        let viz = GeologicGridVisualization::from_grid(&grid);
        assert_eq!(viz.dimensions, [3, 3]);
        assert_eq!(viz.cells.len(), 9);
        // All empty
        assert!(viz.cells.iter().all(|&c| c == 0));
    }

    #[test]
    fn geologic_grid_with_units() {
        let mut grid = crate::grid::GeologicGrid::new(2, 2, 100.0);
        grid.set(
            0,
            0,
            crate::grid::GeologicUnit {
                name: "Granite".into(),
                rock_type: "igneous".into(),
                age_ma: 300.0,
            },
        );
        grid.set(
            1,
            0,
            crate::grid::GeologicUnit {
                name: "Sandstone".into(),
                rock_type: "sedimentary".into(),
                age_ma: 100.0,
            },
        );
        let viz = GeologicGridVisualization::from_grid(&grid);
        assert_eq!(viz.rock_types.len(), 3); // empty + igneous + sedimentary
        assert!(viz.cells[0] > 0); // granite
        assert!(viz.cells[1] > 0); // sandstone
        assert_eq!(viz.cells[2], 0); // empty
    }

    #[test]
    fn crystal_cubic() {
        let cell = crate::crystallography::UnitCell {
            a: 5.0,
            b: 5.0,
            c: 5.0,
            alpha: 90.0,
            beta: 90.0,
            gamma: 90.0,
        };
        let viz = CrystalVisualization::from_unit_cell(&cell, crate::crystal::CrystalSystem::Cubic);
        // a vector should be [5, 0, 0]
        assert!((viz.lattice_vectors[0][0] - 5.0).abs() < 0.01);
        assert!(viz.lattice_vectors[0][1].abs() < 0.01);
        assert!(viz.system.contains("Cubic"));
    }

    #[test]
    fn strat_column_from_layers() {
        let layers = vec![
            StratLayerViz {
                name: "Limestone".into(),
                rock_type: "sedimentary".into(),
                thickness: 50.0,
                age_base_ma: 350.0,
                age_top_ma: 340.0,
            },
            StratLayerViz {
                name: "Shale".into(),
                rock_type: "sedimentary".into(),
                thickness: 30.0,
                age_base_ma: 340.0,
                age_top_ma: 330.0,
            },
        ];
        let viz = StratColumnVisualization::from_layers(layers);
        assert_eq!(viz.layers.len(), 2);
        assert!((viz.total_height - 80.0).abs() < 0.01);
    }

    #[test]
    fn strike_dip_marker() {
        let sd = crate::grid::StrikeDip {
            strike_deg: 45.0,
            dip_deg: 30.0,
        };
        let marker = StrikeDipMarker::new([100.0, 200.0], &sd);
        assert!((marker.strike_deg - 45.0).abs() < 0.01);
        assert!((marker.dip_direction_deg - 135.0).abs() < 0.01);
    }

    #[test]
    fn geologic_grid_serializes() {
        let grid = crate::grid::GeologicGrid::new(2, 2, 10.0);
        let viz = GeologicGridVisualization::from_grid(&grid);
        let json = serde_json::to_string(&viz);
        assert!(json.is_ok());
    }
}
