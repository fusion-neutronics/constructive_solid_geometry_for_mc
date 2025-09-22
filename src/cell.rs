use crate::region::Region;
use std::collections::HashMap;

/// A Cell represents a geometric region that can be filled with material or left as void.
/// This follows OpenMC's approach where cells are defined by:
/// - A region (combination of surfaces using boolean operations)
/// - An optional fill (material or universe)
/// - A name for identification
#[derive(Clone)]
pub struct Cell {
    pub cell_id: u32,
    pub name: Option<String>,
    pub region: Region,
    pub fill: CellFill,
}

/// The fill type for a cell - either material, void, or nested universe
#[derive(Clone)]
pub enum CellFill {
    /// Cell is filled with void (no material)
    Void,
    /// Cell is filled with a material (by ID or reference)
    Material(MaterialRef),
    /// Cell contains a nested universe (for complex geometries)
    Universe(u32), // Universe ID
}

/// Reference to a material - for now just by ID
/// Future versions could integrate with materials_for_mc crate
#[derive(Clone)]
pub enum MaterialRef {
    /// Reference by material ID
    Id(u32),
    // Future: Direct material data integration
    // Data(MaterialStruct), // Where MaterialStruct implements MaterialData
}

// For future integration with materials_for_mc, we could add:
// impl From<materials_for_mc::Material> for MaterialRef {
//     fn from(material: materials_for_mc::Material) -> Self {
//         MaterialRef::Data(material)
//     }
// }

impl Cell {
    /// Create a new cell with void fill
    pub fn new_void(cell_id: u32, region: Region) -> Self {
        Cell {
            cell_id,
            name: None,
            region,
            fill: CellFill::Void,
        }
    }

    /// Create a new cell filled with material
    pub fn new_with_material(cell_id: u32, region: Region, material_id: u32) -> Self {
        Cell {
            cell_id,
            name: None,
            region,
            fill: CellFill::Material(MaterialRef::Id(material_id)),
        }
    }

    /// Set the cell name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Check if a point is inside this cell's region
    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        self.region.contains(point)
    }

    /// Get the material ID if this cell is filled with material
    pub fn material_id(&self) -> Option<u32> {
        match &self.fill {
            CellFill::Material(MaterialRef::Id(id)) => Some(*id),
            _ => None,
        }
    }

    /// Check if this cell is void
    pub fn is_void(&self) -> bool {
        matches!(self.fill, CellFill::Void)
    }

    /// Get bounding box of this cell's region
    pub fn bounding_box(&self) -> crate::bounding_box::BoundingBox {
        self.region.bounding_box()
    }
}

/// A Universe contains a collection of cells that together define a geometric space
/// This follows OpenMC's universe concept for hierarchical geometry
pub struct Universe {
    pub universe_id: u32,
    pub name: Option<String>,
    pub cells: HashMap<u32, Cell>,
}

impl Universe {
    /// Create a new empty universe
    pub fn new(universe_id: u32) -> Self {
        Universe {
            universe_id,
            name: None,
            cells: HashMap::new(),
        }
    }

    /// Add a cell to this universe
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.cell_id, cell);
    }

    /// Find which cell contains a given point
    pub fn find_cell(&self, point: (f64, f64, f64)) -> Option<&Cell> {
        for cell in self.cells.values() {
            if cell.contains(point) {
                return Some(cell);
            }
        }
        None
    }

    /// Get cell by ID
    pub fn get_cell(&self, cell_id: u32) -> Option<&Cell> {
        self.cells.get(&cell_id)
    }

    /// Set universe name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::{Surface, SurfaceKind, BoundaryType};
    use crate::region::{Region, HalfspaceType};
    use std::sync::Arc;

    #[test]
    fn test_cell_creation() {
        // Create a sphere surface
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };

        // Create region inside sphere
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));

        // Create material cell
        let material_cell = Cell::new_with_material(1, region.clone(), 101)
            .with_name("fuel_cell".to_string());

        assert_eq!(material_cell.cell_id, 1);
        assert_eq!(material_cell.name, Some("fuel_cell".to_string()));
        assert_eq!(material_cell.material_id(), Some(101));
        assert!(!material_cell.is_void());

        // Create void cell
        let void_cell = Cell::new_void(2, region.complement());
        assert_eq!(void_cell.cell_id, 2);
        assert_eq!(void_cell.material_id(), None);
        assert!(void_cell.is_void());
    }

    #[test]
    fn test_universe_operations() {
        // Create surfaces for a simple geometry
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };

        let sphere_region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));
        let void_region = sphere_region.complement();

        // Create cells
        let fuel_cell = Cell::new_with_material(1, sphere_region, 101)
            .with_name("fuel".to_string());
        let void_cell = Cell::new_void(2, void_region)
            .with_name("void".to_string());

        // Create universe
        let mut universe = Universe::new(0).with_name("main".to_string());
        universe.add_cell(fuel_cell);
        universe.add_cell(void_cell);

        // Test point location
        let point_inside = (0.0, 0.0, 0.0);
        let cell = universe.find_cell(point_inside).unwrap();
        assert_eq!(cell.cell_id, 1);
        assert_eq!(cell.material_id(), Some(101));

        let point_outside = (0.0, 0.0, 3.0);
        let cell = universe.find_cell(point_outside).unwrap();
        assert_eq!(cell.cell_id, 2);
        assert!(cell.is_void());
    }
}