use serde::{Deserialize, Serialize};

/// A geologic unit representing a distinct rock body.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let unit = GeologicUnit {
///     name: "Basalt".into(),
///     rock_type: "igneous".into(),
///     age_ma: 65.0,
/// };
/// assert_eq!(unit.name, "Basalt");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeologicUnit {
    pub name: String,
    pub rock_type: String,
    pub age_ma: f64,
}

/// Strike and dip measurement for a geologic surface.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let sd = StrikeDip { strike_deg: 45.0, dip_deg: 30.0 };
/// assert!((sd.dip_direction() - 135.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StrikeDip {
    pub strike_deg: f64,
    pub dip_deg: f64,
}

impl StrikeDip {
    /// Returns the dip direction in degrees (strike + 90, wrapped to 0..360).
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let sd = StrikeDip { strike_deg: 300.0, dip_deg: 60.0 };
    /// assert!((sd.dip_direction() - 30.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn dip_direction(&self) -> f64 {
        (self.strike_deg + 90.0) % 360.0
    }
}

/// A 2D spatial grid of optional geologic units.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mut grid = GeologicGrid::new(3, 3, 10.0);
/// assert!(grid.get(0, 0).is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeologicGrid {
    pub nx: usize,
    pub ny: usize,
    pub cell_size_m: f64,
    cells: Vec<Option<GeologicUnit>>,
}

impl GeologicGrid {
    /// Creates a new grid with the given dimensions and cell size.
    /// All cells are initially `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let grid = GeologicGrid::new(4, 3, 10.0);
    /// assert_eq!(grid.nx, 4);
    /// assert_eq!(grid.ny, 3);
    /// ```
    #[must_use]
    pub fn new(nx: usize, ny: usize, cell_size_m: f64) -> Self {
        Self {
            nx,
            ny,
            cell_size_m,
            cells: vec![None; nx * ny],
        }
    }

    /// Sets the geologic unit at grid position (x, y).
    ///
    /// # Panics
    ///
    /// Panics if x >= nx or y >= ny.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let mut grid = GeologicGrid::new(3, 3, 1.0);
    /// let unit = GeologicUnit { name: "Granite".into(), rock_type: "igneous".into(), age_ma: 300.0 };
    /// grid.set(1, 1, unit.clone());
    /// assert_eq!(grid.get(1, 1).unwrap(), &unit);
    /// ```
    pub fn set(&mut self, x: usize, y: usize, unit: GeologicUnit) {
        assert!(x < self.nx, "x index {x} out of bounds (nx={})", self.nx);
        assert!(y < self.ny, "y index {y} out of bounds (ny={})", self.ny);
        self.cells[y * self.nx + x] = Some(unit);
    }

    /// Returns a reference to the geologic unit at grid position (x, y), if any.
    ///
    /// # Panics
    ///
    /// Panics if x >= nx or y >= ny.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let grid = GeologicGrid::new(3, 3, 5.0);
    /// assert!(grid.get(0, 0).is_none());
    /// ```
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&GeologicUnit> {
        assert!(x < self.nx, "x index {x} out of bounds (nx={})", self.nx);
        assert!(y < self.ny, "y index {y} out of bounds (ny={})", self.ny);
        self.cells[y * self.nx + x].as_ref()
    }

    /// Fills every cell in the grid with the given geologic unit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let mut grid = GeologicGrid::new(2, 2, 1.0);
    /// let unit = GeologicUnit { name: "Shale".into(), rock_type: "sedimentary".into(), age_ma: 100.0 };
    /// grid.fill(unit);
    /// assert!(grid.get(0, 0).is_some());
    /// assert!(grid.get(1, 1).is_some());
    /// ```
    pub fn fill(&mut self, unit: GeologicUnit) {
        for cell in &mut self.cells {
            *cell = Some(unit.clone());
        }
    }
}

/// A horizontal geologic layer within a stratigraphic column.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mut col = StratigraphicColumn::new();
/// col.add_layer("Limestone".into(), 10.0, "carbonate".into());
/// let layer = col.layer_at_depth(5.0).unwrap();
/// assert_eq!(layer.name, "Limestone");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeologicLayer {
    pub name: String,
    pub thickness_m: f64,
    pub rock_type: String,
    pub depth_top_m: f64,
}

/// An ordered sequence of geologic layers forming a stratigraphic column.
///
/// # Examples
///
/// ```
/// # use khanij::*;
/// let mut col = StratigraphicColumn::new();
/// col.add_layer("Topsoil".into(), 2.0, "soil".into());
/// col.add_layer("Sandstone".into(), 10.0, "clastic".into());
/// assert!((col.total_thickness() - 12.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratigraphicColumn {
    layers: Vec<GeologicLayer>,
}

impl StratigraphicColumn {
    /// Creates a new empty stratigraphic column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let col = StratigraphicColumn::new();
    /// assert!((col.total_thickness()).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Adds a layer to the column. The layer's `depth_top_m` is set automatically
    /// based on the current total thickness.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let mut col = StratigraphicColumn::new();
    /// col.add_layer("Shale".into(), 8.0, "mudrock".into());
    /// assert!((col.total_thickness() - 8.0).abs() < 1e-6);
    /// ```
    pub fn add_layer(&mut self, name: String, thickness_m: f64, rock_type: String) {
        let depth_top_m = self.total_thickness();
        self.layers.push(GeologicLayer {
            name,
            thickness_m,
            rock_type,
            depth_top_m,
        });
    }

    /// Returns the total thickness of all layers in the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let mut col = StratigraphicColumn::new();
    /// col.add_layer("A".into(), 5.0, "sand".into());
    /// col.add_layer("B".into(), 10.0, "clay".into());
    /// assert!((col.total_thickness() - 15.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn total_thickness(&self) -> f64 {
        self.layers.iter().map(|l| l.thickness_m).sum()
    }

    /// Returns a reference to the layer at the given depth, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// # use khanij::*;
    /// let mut col = StratigraphicColumn::new();
    /// col.add_layer("Topsoil".into(), 2.0, "soil".into());
    /// col.add_layer("Bedrock".into(), 20.0, "granite".into());
    /// assert_eq!(col.layer_at_depth(1.0).unwrap().name, "Topsoil");
    /// assert_eq!(col.layer_at_depth(10.0).unwrap().name, "Bedrock");
    /// assert!(col.layer_at_depth(25.0).is_none());
    /// ```
    #[must_use]
    pub fn layer_at_depth(&self, depth_m: f64) -> Option<&GeologicLayer> {
        self.layers
            .iter()
            .find(|l| depth_m >= l.depth_top_m && depth_m < l.depth_top_m + l.thickness_m)
    }
}

impl Default for StratigraphicColumn {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_unit(name: &str) -> GeologicUnit {
        GeologicUnit {
            name: name.to_string(),
            rock_type: "granite".to_string(),
            age_ma: 250.0,
        }
    }

    #[test]
    fn grid_creation() {
        let grid = GeologicGrid::new(4, 3, 10.0);
        assert_eq!(grid.nx, 4);
        assert_eq!(grid.ny, 3);
        assert_eq!(grid.cell_size_m, 10.0);
        assert!(grid.get(0, 0).is_none());
    }

    #[test]
    fn grid_set_and_get() {
        let mut grid = GeologicGrid::new(5, 5, 1.0);
        let unit = sample_unit("Basalt Flow");
        grid.set(2, 3, unit.clone());
        let retrieved = grid.get(2, 3).unwrap();
        assert_eq!(retrieved, &unit);
    }

    #[test]
    fn grid_get_empty_cell() {
        let grid = GeologicGrid::new(3, 3, 5.0);
        assert!(grid.get(1, 1).is_none());
    }

    #[test]
    fn grid_fill() {
        let mut grid = GeologicGrid::new(2, 2, 1.0);
        let unit = sample_unit("Sandstone");
        grid.fill(unit.clone());
        for y in 0..2 {
            for x in 0..2 {
                assert_eq!(grid.get(x, y).unwrap(), &unit);
            }
        }
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn grid_set_out_of_bounds() {
        let mut grid = GeologicGrid::new(3, 3, 1.0);
        grid.set(5, 0, sample_unit("oob"));
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn grid_get_out_of_bounds() {
        let grid = GeologicGrid::new(3, 3, 1.0);
        let _ = grid.get(0, 10);
    }

    #[test]
    fn strike_dip_direction() {
        let sd = StrikeDip {
            strike_deg: 45.0,
            dip_deg: 30.0,
        };
        assert!((sd.dip_direction() - 135.0).abs() < f64::EPSILON);
    }

    #[test]
    fn strike_dip_wraps_around() {
        let sd = StrikeDip {
            strike_deg: 300.0,
            dip_deg: 60.0,
        };
        assert!((sd.dip_direction() - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn column_construction_and_thickness() {
        let mut col = StratigraphicColumn::new();
        col.add_layer("Topsoil".into(), 2.0, "soil".into());
        col.add_layer("Limestone".into(), 15.0, "carbonate".into());
        col.add_layer("Shale".into(), 8.0, "mudrock".into());
        assert!((col.total_thickness() - 25.0).abs() < f64::EPSILON);
        assert_eq!(col.layers.len(), 3);
    }

    #[test]
    fn column_layer_at_depth() {
        let mut col = StratigraphicColumn::new();
        col.add_layer("Topsoil".into(), 2.0, "soil".into());
        col.add_layer("Limestone".into(), 15.0, "carbonate".into());
        col.add_layer("Shale".into(), 8.0, "mudrock".into());

        let layer = col.layer_at_depth(0.5).unwrap();
        assert_eq!(layer.name, "Topsoil");

        let layer = col.layer_at_depth(10.0).unwrap();
        assert_eq!(layer.name, "Limestone");

        let layer = col.layer_at_depth(20.0).unwrap();
        assert_eq!(layer.name, "Shale");

        assert!(col.layer_at_depth(25.0).is_none());
    }

    #[test]
    fn column_default() {
        let col = StratigraphicColumn::default();
        assert!((col.total_thickness() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn geologic_unit_serde_roundtrip() {
        let unit = sample_unit("Gneiss");
        let json = serde_json::to_string(&unit).unwrap();
        let deserialized: GeologicUnit = serde_json::from_str(&json).unwrap();
        assert_eq!(unit, deserialized);
    }
}
