use crate::region::{RegionExpr, HalfspaceType, Region};
use std::sync::Arc;

#[derive(Clone)]
pub struct Surface {
    pub surface_id: usize,
    pub kind: SurfaceKind,
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
        }
    }

    pub fn new_sphere(x0: f64, y0: f64, z0: f64, radius: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Sphere { x0, y0, z0, radius },
        }
    }

    pub fn new_cylinder(axis: [f64; 3], origin: [f64; 3], radius: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Cylinder { axis, origin, radius },
        }
    }
    
    pub fn x_plane(x0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: x0 },
        }
    }

    pub fn y_plane(y0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: y0 },
        }
    }

    pub fn z_plane(z0: f64, surface_id: usize) -> Self {
        Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: z0 },
        }
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
