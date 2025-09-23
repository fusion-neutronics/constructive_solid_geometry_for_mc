use crate::region::Region;
use std::sync::Arc;

/// A Cell represents a geometric region
/// This follows OpenMC's approach where cells are defined by:
/// - A region (combination of surfaces using boolean operations)
/// - A name for identification
#[derive(Clone)]
pub struct Cell {
    pub cell_id: u32,
    pub name: Option<String>,
    pub region: Region,
}

impl Cell {
    /// Create a new cell with a regio
    pub fn new(cell_id: u32, region: Region, name: Option<String>) -> Self {
        Cell {
            cell_id,
            name,
            region,
        }
    }


    /// Check if a point is inside this cell's region
    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        self.region.contains(point)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::{Surface, SurfaceKind, BoundaryType};
    use crate::region::{Region, HalfspaceType};
    use std::sync::Arc;

    #[test]
    fn test_cell_contains_simple() {
        // Sphere of radius 2 at (0,0,0)
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));
    let cell = Cell::new(1, region, Some(101), None);
        assert!(cell.contains((0.0, 0.0, 0.0)));
        assert!(!cell.contains((3.0, 0.0, 0.0)));
    }

    #[test]
    fn test_cell_union_intersection_complement() {
        // Two spheres
        let s1 = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };
        let s2 = Surface {
            surface_id: 2,
            kind: SurfaceKind::Sphere { x0: 2.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };
        let region1 = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s1.clone())));
        let region2 = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s2.clone())));
        // Union
    let union_cell = Cell::new(2, region1.clone().union(&region2.clone()), Some(102), None);
        assert!(union_cell.contains((0.0, 0.0, 0.0)));
        assert!(union_cell.contains((2.0, 0.0, 0.0)));
        assert!(!union_cell.contains((5.0, 0.0, 0.0)));
        // Intersection
    let intersection_cell = Cell::new(3, region1.clone().intersection(&region2.clone()), Some(103), None);
        assert!(!intersection_cell.contains((0.0, 0.0, 0.0)));
        assert!(intersection_cell.contains((1.0, 0.0, 0.0)));
        // Complement
    let complement_cell = Cell::new(4, region1.complement(), None, None);
        assert!(!complement_cell.contains((0.0, 0.0, 0.0)));
        assert!(complement_cell.contains((5.0, 0.0, 0.0)));
    }

    #[test]
    fn test_cell_naming() {
        let sphere = Surface {
            surface_id: 1,
            kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 },
            boundary_type: BoundaryType::default(),
        };
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(sphere)));
    let cell = Cell::new(1, region, Some(101), Some("fuel".to_string()));
        assert_eq!(cell.name, Some("fuel".to_string()));
        assert!(!cell.is_void());
    }
}
