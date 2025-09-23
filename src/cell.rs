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

    #[test]
    fn test_complex_region_cell() {
        // Create multiple surfaces for a box-like region
        let x_min = Surface {
            surface_id: 1,
            kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: -1.0 },
            boundary_type: BoundaryType::default(),
        };
        let x_max = Surface {
            surface_id: 2,
            kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: 1.0 },
            boundary_type: BoundaryType::default(),
        };
        let y_min = Surface {
            surface_id: 3,
            kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: -1.0 },
            boundary_type: BoundaryType::default(),
        };
        let y_max = Surface {
            surface_id: 4,
            kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: 1.0 },
            boundary_type: BoundaryType::default(),
        };

        // Create box region: -1 < x < 1 AND -1 < y < 1
        let box_region = Region::new_from_halfspace(HalfspaceType::Above(Arc::new(x_min)))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Below(Arc::new(x_max))))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Above(Arc::new(y_min))))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Below(Arc::new(y_max))));

        let box_cell = Cell::new_with_material(10, box_region, 202)
            .with_name("box_cell".to_string());

        // Test points
        assert!(box_cell.contains((0.0, 0.0, 0.0))); // Inside box
        assert!(box_cell.contains((0.5, 0.5, 100.0))); // Inside box (z unconstrained)
        assert!(!box_cell.contains((1.5, 0.0, 0.0))); // Outside box (x too large)
        assert!(!box_cell.contains((0.0, 1.5, 0.0))); // Outside box (y too large)

        // Test cell properties
        assert_eq!(box_cell.cell_id, 10);
        assert_eq!(box_cell.name.as_ref().unwrap(), "box_cell");
        assert_eq!(box_cell.material_id(), Some(202));
        assert!(!box_cell.is_void());
    }

    #[test]
    fn test_union_region_cell() {
        // Create two spheres
        let sphere1 = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: -1.0, y0: 0.0, z0: 0.0, radius: 1.0 },
            boundary_type: BoundaryType::default(),
        };
        let sphere2 = Surface {
            surface_id: 2,
            kind: SurfaceKind::Sphere { x0: 1.0, y0: 0.0, z0: 0.0, radius: 1.0 },
            boundary_type: BoundaryType::default(),
        };

        // Create union of two spheres
        let region1 = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere1)));
        let region2 = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere2)));
        let union_region = region1.union(&region2);

        let union_cell = Cell::new_with_material(20, union_region, 303);

        // Test points
        assert!(union_cell.contains((-1.0, 0.0, 0.0))); // Center of first sphere
        assert!(union_cell.contains((1.0, 0.0, 0.0))); // Center of second sphere
        assert!(union_cell.contains((0.0, 0.0, 0.0))); // Between spheres (overlapping region)
        assert!(!union_cell.contains((0.0, 2.0, 0.0))); // Outside both spheres
        assert!(!union_cell.contains((3.0, 0.0, 0.0))); // Far from both spheres
    }

    #[test]
    fn test_cell_bounding_box() {
        // Create a sphere
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 1.0, y0: 2.0, z0: 3.0, radius: 1.5 },
            boundary_type: BoundaryType::default(),
        };

        let sphere_region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));
        let sphere_cell = Cell::new_with_material(100, sphere_region, 400);

        let bbox = sphere_cell.bounding_box();
        
        // Check bounding box
        assert_eq!(bbox.lower_left, [-0.5, 0.5, 1.5]); // center - radius
        assert_eq!(bbox.upper_right, [2.5, 3.5, 4.5]); // center + radius
        assert_eq!(bbox.center, [1.0, 2.0, 3.0]); // sphere center
        assert_eq!(bbox.width, [3.0, 3.0, 3.0]); // 2 * radius
    }

    #[test]
    fn test_universe_multiple_cells() {
        // Create concentric spheres
        let inner_sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 1.0 },
            boundary_type: BoundaryType::default(),
        };
        let outer_sphere = Surface {
            surface_id: 2,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };

        // Create regions
        let inner_region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(inner_sphere.clone())));
        let shell_region = Region::new_from_halfspace(HalfspaceType::Above(Arc::new(inner_sphere)))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Below(Arc::new(outer_sphere.clone()))));
        let outer_region = Region::new_from_halfspace(HalfspaceType::Above(Arc::new(outer_sphere)));

        // Create cells
        let fuel_cell = Cell::new_with_material(1, inner_region, 235).with_name("fuel".to_string());
        let moderator_cell = Cell::new_with_material(2, shell_region, 1001).with_name("moderator".to_string());
        let void_cell = Cell::new_void(3, outer_region).with_name("void".to_string());

        // Create universe
        let mut universe = Universe::new(1).with_name("reactor_cell".to_string());
        universe.add_cell(fuel_cell);
        universe.add_cell(moderator_cell);
        universe.add_cell(void_cell);

        // Test various points
        let test_cases = vec![
            ((0.0, 0.0, 0.0), 1, Some(235), "fuel"), // Center - fuel
            ((0.5, 0.0, 0.0), 1, Some(235), "fuel"), // Inside fuel
            ((1.5, 0.0, 0.0), 2, Some(1001), "moderator"), // In shell
            ((2.5, 0.0, 0.0), 3, None, "void"), // Outside
        ];

        for (point, expected_cell_id, expected_material, description) in test_cases {
            let cell = universe.find_cell(point).expect(&format!("No cell found for point {:?} ({})", point, description));
            assert_eq!(cell.cell_id, expected_cell_id, "Wrong cell ID for {} at {:?}", description, point);
            assert_eq!(cell.material_id(), expected_material, "Wrong material for {} at {:?}", description, point);
        }

        // Test universe properties
        assert_eq!(universe.universe_id, 1);
        assert_eq!(universe.name.as_ref().unwrap(), "reactor_cell");
        assert_eq!(universe.cells.len(), 3);

        // Test get_cell
        let fuel = universe.get_cell(1).unwrap();
        assert_eq!(fuel.name.as_ref().unwrap(), "fuel");
        assert_eq!(fuel.material_id(), Some(235));

        assert!(universe.get_cell(999).is_none()); // Non-existent cell
    }

    #[test]
    fn test_cell_fill_types() {
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 1.0 },
            boundary_type: BoundaryType::default(),
        };
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));

        // Test material cell
        let material_cell = Cell::new_with_material(1, region.clone(), 123);
        assert!(!material_cell.is_void());
        assert_eq!(material_cell.material_id(), Some(123));

        // Test void cell
        let void_cell = Cell::new_void(2, region);
        assert!(void_cell.is_void());
        assert_eq!(void_cell.material_id(), None);
    }
}