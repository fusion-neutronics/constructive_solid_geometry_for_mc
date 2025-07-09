use crate::region::{RegionExpr, HalfspaceType, Region};

#[derive(Clone)]
pub struct Surface {
    pub id: usize,
    pub kind: SurfaceKind,
}

#[derive(Clone)]
pub enum SurfaceKind {
    Plane { a: f64, b: f64, c: f64, d: f64 },
    Sphere { center: [f64; 3], radius: f64 },
    Cylinder { axis: [f64; 3], origin: [f64; 3], radius: f64 },
}

// Regular Rust implementation
impl Surface {
    pub fn new_plane(a: f64, b: f64, c: f64, d: f64, id: usize) -> Self {
        Surface {
            id,
            kind: SurfaceKind::Plane { a, b, c, d },
        }
    }

    pub fn new_sphere(center: [f64; 3], radius: f64, id: usize) -> Self {
        Surface {
            id,
            kind: SurfaceKind::Sphere { center, radius },
        }
    }

    pub fn new_cylinder(axis: [f64; 3], origin: [f64; 3], radius: f64, id: usize) -> Self {
        Surface {
            id,
            kind: SurfaceKind::Cylinder { axis, origin, radius },
        }
    }
    
    pub fn evaluate(&self, point: (f64, f64, f64)) -> f64 {
        match &self.kind {
            SurfaceKind::Plane { a, b, c, d } => {
                a * point.0 + b * point.1 + c * point.2 - d
            }
            SurfaceKind::Sphere { center, radius } => {
                let dx = point.0 - center[0];
                let dy = point.1 - center[1];
                let dz = point.2 - center[2];
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
    pub fn new_above(id: usize) -> Self {
        Halfspace {
            expr: RegionExpr::Halfspace(HalfspaceType::Above(id)),
        }
    }

    pub fn new_below(id: usize) -> Self {
        Halfspace {
            expr: RegionExpr::Halfspace(HalfspaceType::Below(id)),
        }
    }
}
