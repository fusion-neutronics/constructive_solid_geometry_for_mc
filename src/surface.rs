use crate::region::{RegionExpr, HalfspaceType};
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

impl BoundaryType {
    /// Parse a boundary type from a string, returning None for invalid strings
    pub fn from_str_option(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "transmission" => Some(BoundaryType::Transmission),
            "vacuum" => Some(BoundaryType::Vacuum),
            _ => None,
        }
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
    pub fn new_plane(a: f64, b: f64, c: f64, d: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a, b, c, d },
            boundary_type: boundary_type.unwrap_or_default(),
        }
    }

    pub fn new_sphere(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Sphere { x0, y0, z0, radius },
            boundary_type: boundary_type.unwrap_or_default(),
        }
    }

    pub fn new_cylinder(axis: [f64; 3], origin: [f64; 3], radius: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Cylinder { axis, origin, radius },
            boundary_type: boundary_type.unwrap_or_default(),
        }
    }
    
    pub fn x_plane(x0: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_plane(1.0, 0.0, 0.0, x0, surface_id, boundary_type)
    }

    pub fn y_plane(y0: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_plane(0.0, 1.0, 0.0, y0, surface_id, boundary_type)
    }

    pub fn z_plane(z0: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_plane(0.0, 0.0, 1.0, z0, surface_id, boundary_type)
    }

    /// Create a cylinder oriented along the Z axis, centered at (x0, y0), with given radius and surface_id
    pub fn z_cylinder(x0: f64, y0: f64, radius: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_cylinder([0.0, 0.0, 1.0], [x0, y0, 0.0], radius, surface_id, boundary_type)
    }

    /// Create a sphere with a specific boundary type
    pub fn sphere(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_sphere(x0, y0, z0, radius, surface_id, boundary_type)
    }

    /// Create a cylinder with individual axis components with a specific boundary type
    pub fn cylinder(x0: f64, y0: f64, z0: f64, axis_x: f64, axis_y: f64, axis_z: f64, radius: f64, surface_id: usize, boundary_type: Option<BoundaryType>) -> Self {
        Self::new_cylinder([axis_x, axis_y, axis_z], [x0, y0, z0], radius, surface_id, boundary_type)
    }

    // Python-friendly functions that accept string boundary types
    pub fn x_plane_str(x0: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::x_plane(x0, surface_id, boundary))
    }

    pub fn y_plane_str(y0: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::y_plane(y0, surface_id, boundary))
    }

    pub fn z_plane_str(z0: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::z_plane(z0, surface_id, boundary))
    }

    pub fn sphere_str(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::sphere(x0, y0, z0, radius, surface_id, boundary))
    }

    pub fn cylinder_str(x0: f64, y0: f64, z0: f64, axis_x: f64, axis_y: f64, axis_z: f64, radius: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::cylinder(x0, y0, z0, axis_x, axis_y, axis_z, radius, surface_id, boundary))
    }

    pub fn z_cylinder_str(x0: f64, y0: f64, radius: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::z_cylinder(x0, y0, radius, surface_id, boundary))
    }

    pub fn plane_str(a: f64, b: f64, c: f64, d: f64, surface_id: usize, boundary_type: Option<&str>) -> Result<Self, String> {
        let boundary = match boundary_type {
            Some(s) => Some(BoundaryType::from_str_option(s).ok_or("boundary_type must be 'transmission' or 'vacuum'")?),
            None => None,
        };
        Ok(Self::new_plane(a, b, c, d, surface_id, boundary))
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
        let plane = Surface::new_plane(1.0, 0.0, 0.0, 2.0, 42, None);
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
        let sphere = Surface::new_sphere(1.0, 2.0, 3.0, 5.0, 7, None);
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
        let cylinder = Surface::new_cylinder(axis, origin, 2.0, 99, None);
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
        let zcyl = Surface::z_cylinder(1.0, 2.0, 3.0, 123, None);
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
        let plane = Surface::new_plane(1.0, 0.0, 0.0, 2.0, 42, None);
        assert_eq!(*plane.boundary_type(), BoundaryType::Vacuum);
    }

    #[test]
    fn test_boundary_type_vacuum() {
        let sphere = Surface::new_sphere(0.0, 0.0, 0.0, 1.0, 1, Some(BoundaryType::Vacuum));
        assert_eq!(*sphere.boundary_type(), BoundaryType::Vacuum);
    }

    #[test]
    fn test_set_boundary_type() {
        let mut cylinder = Surface::new_cylinder([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], 1.0, 2, None);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Vacuum);
        
        cylinder.set_boundary_type(BoundaryType::Transmission);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Transmission);
        
        cylinder.set_boundary_type(BoundaryType::Vacuum);
        assert_eq!(*cylinder.boundary_type(), BoundaryType::Vacuum);
    }
}
