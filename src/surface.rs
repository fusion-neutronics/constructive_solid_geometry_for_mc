use crate::region::{RegionExpr, HalfspaceType, Region};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum BoundaryType {
    Transmission,
    Vacuum,
}

impl Default for BoundaryType {
    fn default() -> Self {
        BoundaryType::Vacuum
    }
}

#[derive(Clone)]
pub struct Surface {
    pub surface_id: usize,
    pub kind: SurfaceKind,
    pub boundary_type: BoundaryType,
}

#[derive(Clone)]
pub enum SurfaceKind {
    Plane { a: f64, b: f64, c: f64, d: f64 },
    Sphere { x0: f64, y0: f64, z0: f64, radius: f64 },
    Cylinder { axis: [f64; 3], origin: [f64; 3], radius: f64 },
}

// Regular Rust implementation
impl Surface {
    pub fn new_plane(a: f64, b: f64, c: f64, d: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a, b, c, d },
            boundary_type: BoundaryType::default(),
        }
    }

    pub fn new_plane_with_boundary(a: f64, b: f64, c: f64, d: f64, surface_id: usize, boundary_type: BoundaryType) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a, b, c, d },
            boundary_type,
        }
    }

    pub fn new_sphere(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Sphere { x0, y0, z0, radius },
            boundary_type: BoundaryType::default(),
        }
    }

    pub fn new_sphere_with_boundary(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize, boundary_type: BoundaryType) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Sphere { x0, y0, z0, radius },
            boundary_type,
        }
    }

    pub fn new_cylinder(axis: [f64; 3], origin: [f64; 3], radius: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Cylinder { axis, origin, radius },
            boundary_type: BoundaryType::default(),
        }
    }

    pub fn new_cylinder_with_boundary(axis: [f64; 3], origin: [f64; 3], radius: f64, surface_id: usize, boundary_type: BoundaryType) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Cylinder { axis, origin, radius },
            boundary_type,
        }
    }
    
    pub fn x_plane(x0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: x0 },
            boundary_type: BoundaryType::default(),
        }
    }

    pub fn y_plane(y0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: y0 },
            boundary_type: BoundaryType::default(),
        }
    }


    pub fn z_plane(z0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: z0 },
            boundary_type: BoundaryType::default(),
        }
    }

    /// Create a cylinder oriented along the Z axis, centered at (x0, y0), with given radius and surface_id
    pub fn z_cylinder(x0: f64, y0: f64, radius: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Cylinder {
                axis: [0.0, 0.0, 1.0],
                origin: [x0, y0, 0.0],
                radius,
            },
            boundary_type: BoundaryType::default(),
        }
    }

    /// Get the boundary type of the surface
    pub fn boundary_type(&self) -> &BoundaryType {
        &self.boundary_type
    }

    /// Set the boundary type of the surface
    pub fn set_boundary_type(&mut self, boundary_type: BoundaryType) {
        self.boundary_type = boundary_type;
    }

    pub fn evaluate(&self, point: (f64, f64, f64)) -> f64 {
        match &self.kind {
            SurfaceKind::Plane { a, b, c, d } => {
                a * point.0 + b * point.1 + c * point.2 - d
            }
            SurfaceKind::Sphere { x0, y0, z0, radius } => {
                let dx = point.0 - x0;
                let dy = point.1 - y0;
                let dz = point.2 - z0;
                (dx * dx + dy * dy + dz * dz).sqrt() - radius
            }
            SurfaceKind::Cylinder { axis, origin, radius } => {
                let v = [point.0 - origin[0], point.1 - origin[1], point.2 - origin[2]];
                let dot = v[0] * axis[0] + v[1] * axis[1] + v[2] * axis[2];
                let d = [
                    v[0] - dot * axis[0],
                    v[1] - dot * axis[1],
                    v[2] - dot * axis[2],
                ];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() - radius
            }
        }
    }
}

#[derive(Clone)]
pub struct Halfspace {
    pub expr: RegionExpr,
}

impl Halfspace {
    pub fn new_above(surface: Arc<Surface>) -> Self {
        Halfspace {
            expr: RegionExpr::Halfspace(HalfspaceType::Above(surface)),
        }
    }

    pub fn new_below(surface: Arc<Surface>) -> Self {
        Halfspace {
            expr: RegionExpr::Halfspace(HalfspaceType::Below(surface)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_creation() {
        let plane = Surface::new_plane(1.0, 0.0, 0.0, 2.0, 42);
        match plane.kind {
            SurfaceKind::Plane { a, b, c, d } => {
                assert_eq!(a, 1.0);
                assert_eq!(b, 0.0);
                assert_eq!(c, 0.0);
                assert_eq!(d, 2.0);
            }
            _ => panic!("Not a plane"),
        }
        assert_eq!(plane.surface_id, 42);
    }

    #[test]
    fn test_sphere_creation() {
        let sphere = Surface::new_sphere(1.0, 2.0, 3.0, 5.0, 7);
        match sphere.kind {
            SurfaceKind::Sphere { x0, y0, z0, radius } => {
                assert_eq!(x0, 1.0);
                assert_eq!(y0, 2.0);
                assert_eq!(z0, 3.0);
                assert_eq!(radius, 5.0);
            }
            _ => panic!("Not a sphere"),
        }
        assert_eq!(sphere.surface_id, 7);
    }

    #[test]
    fn test_cylinder_creation() {
        let axis = [0.0, 1.0, 0.0];
        let origin = [1.0, 2.0, 3.0];
        let cylinder = Surface::new_cylinder(axis, origin, 2.0, 99);
        match cylinder.kind {
            SurfaceKind::Cylinder { axis: a, origin: o, radius } => {
                assert_eq!(a, axis);
                assert_eq!(o, origin);
                assert_eq!(radius, 2.0);
            }
            _ => panic!("Not a cylinder"),
        }
        assert_eq!(cylinder.surface_id, 99);
    }

    #[test]
    fn test_z_cylinder_creation() {
        let zcyl = Surface::z_cylinder(1.0, 2.0, 3.0, 123);
        match zcyl.kind {
            SurfaceKind::Cylinder { axis, origin, radius } => {
                assert_eq!(axis, [0.0, 0.0, 1.0]);
                assert_eq!(origin, [1.0, 2.0, 0.0]);
                assert_eq!(radius, 3.0);
            }
            _ => panic!("Not a Z cylinder"),
        }
        assert_eq!(zcyl.surface_id, 123);
    }

    #[test]
    fn test_boundary_type_default() {
        let plane = Surface::new_plane(1.0, 0.0, 0.0, 2.0, 42);
        assert_eq!(*plane.boundary_type(), BoundaryType::Vacuum);
    }

    #[test]
    fn test_boundary_type_vacuum() {
        let sphere = Surface::new_sphere_with_boundary(0.0, 0.0, 0.0, 1.0, 1, BoundaryType::Vacuum);
        assert_eq!(*sphere.boundary_type(), BoundaryType::Vacuum);
    }

    #[test]
    fn test_set_boundary_type() {
        let mut cylinder = Surface::new_cylinder([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], 1.0, 2);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Vacuum);
        
        cylinder.set_boundary_type(BoundaryType::Transmission);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Transmission);
        
        cylinder.set_boundary_type(BoundaryType::Vacuum);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Vacuum);
    }
}
