// ...existing code...
use crate::surface::Surface;
// ...existing code...
use std::sync::Arc;

#[derive(Clone)]
pub struct Region {
    pub expr: RegionExpr,
}

#[derive(Clone)]
pub enum HalfspaceType {
    Above(Arc<Surface>),
    Below(Arc<Surface>),
}

#[derive(Clone)]
pub enum RegionExpr {
    Halfspace(HalfspaceType),
    Union(Box<RegionExpr>, Box<RegionExpr>),
    Intersection(Box<RegionExpr>, Box<RegionExpr>),
    Complement(Box<RegionExpr>),
}

// Regular Rust implementation
impl Region {
    pub fn new_from_halfspace(halfspace_type: HalfspaceType) -> Self {
        Region {
            expr: RegionExpr::Halfspace(halfspace_type),
        }
    }
    
    pub fn intersection(&self, other: &Self) -> Self {
        Region {
            expr: RegionExpr::Intersection(Box::new(self.expr.clone()), Box::new(other.expr.clone())),
        }
    }
    
    pub fn union(&self, other: &Self) -> Self {
        Region {
            expr: RegionExpr::Union(Box::new(self.expr.clone()), Box::new(other.expr.clone())),
        }
    }
    
    pub fn complement(&self) -> Self {
        Region {
            expr: RegionExpr::Complement(Box::new(self.expr.clone())),
        }
    }
    
    // Updated contains method: no surface dictionary needed
    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        self.expr.evaluate_contains(point)
    }

    // Updated evaluate_contains method: no surface dictionary needed
    pub fn evaluate_contains(&self, point: (f64, f64, f64)) -> bool {
        self.expr.evaluate_contains(point)
    }

    pub fn bounding_box(&self) -> crate::bounding_box::BoundingBox {
        use crate::surface::SurfaceKind;
        let mut x_bounds = (f64::NEG_INFINITY, f64::INFINITY);
        let mut y_bounds = (f64::NEG_INFINITY, f64::INFINITY);
        let mut z_bounds = (f64::NEG_INFINITY, f64::INFINITY);

        // Collect axis-aligned plane bounds with correct sign convention
        fn collect_axis_bounds(expr: &RegionExpr,
                              x_bounds: &mut (f64, f64), y_bounds: &mut (f64, f64), z_bounds: &mut (f64, f64)) {
            match expr {
                RegionExpr::Intersection(a, b) => {
                    collect_axis_bounds(a, x_bounds, y_bounds, z_bounds);
                    collect_axis_bounds(b, x_bounds, y_bounds, z_bounds);
                }
                RegionExpr::Halfspace(hs) => {
                    match hs {
                        HalfspaceType::Below(surf) => {
                            match &surf.kind {
                                SurfaceKind::Plane { a, b, c, d } => {
                                    if *a == 1.0 && *b == 0.0 && *c == 0.0 {
                                        x_bounds.1 = x_bounds.1.min(*d); // x < d
                                    } else if *a == 0.0 && *b == 1.0 && *c == 0.0 {
                                        y_bounds.1 = y_bounds.1.min(*d); // y < d
                                    } else if *a == 0.0 && *b == 0.0 && *c == 1.0 {
                                        z_bounds.1 = z_bounds.1.min(*d); // z < d
                                    }
                                }
                                _ => {}
                            }
                        }
                        HalfspaceType::Above(surf) => {
                            match &surf.kind {
                                SurfaceKind::Plane { a, b, c, d } => {
                                    if *a == 1.0 && *b == 0.0 && *c == 0.0 {
                                        x_bounds.0 = x_bounds.0.max(*d); // x > d
                                    } else if *a == 0.0 && *b == 1.0 && *c == 0.0 {
                                        y_bounds.0 = y_bounds.0.max(*d); // y > d
                                    } else if *a == 0.0 && *b == 0.0 && *c == 1.0 {
                                        z_bounds.0 = z_bounds.0.max(*d); // z > d
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        collect_axis_bounds(&self.expr, &mut x_bounds, &mut y_bounds, &mut z_bounds);

        // Intersect with sphere bounds if present
        fn find_sphere_bounds(expr: &RegionExpr) -> Option<([f64; 3], [f64; 3])> {
            match expr {
                RegionExpr::Halfspace(hs) => {
                    match hs {
                        HalfspaceType::Above(surf) | HalfspaceType::Below(surf) => {
                            if let SurfaceKind::Sphere { x0, y0, z0, radius } = &surf.kind {
                                return Some((
                                    [*x0 - *radius, *y0 - *radius, *z0 - *radius],
                                    [*x0 + *radius, *y0 + *radius, *z0 + *radius],
                                ));
                            } else {
                                None
                            }
                        }
                    }
                }
                RegionExpr::Intersection(a, b) | RegionExpr::Union(a, b) => {
                    find_sphere_bounds(a).or_else(|| find_sphere_bounds(b))
                }
                RegionExpr::Complement(inner) => find_sphere_bounds(inner),
            }
        }
    let sphere_bounds = find_sphere_bounds(&self.expr);

        let lower = [
            sphere_bounds.map_or(x_bounds.0, |b| x_bounds.0.max(b.0[0])),
            sphere_bounds.map_or(y_bounds.0, |b| y_bounds.0.max(b.0[1])),
            sphere_bounds.map_or(z_bounds.0, |b| z_bounds.0.max(b.0[2])),
        ];
        let upper = [
            sphere_bounds.map_or(x_bounds.1, |b| x_bounds.1.min(b.1[0])),
            sphere_bounds.map_or(y_bounds.1, |b| y_bounds.1.min(b.1[1])),
            sphere_bounds.map_or(z_bounds.1, |b| z_bounds.1.min(b.1[2])),
        ];

        // If any min > max, region is empty: return empty bounding box
        if lower[0] > upper[0] || lower[1] > upper[1] || lower[2] > upper[2] {
            return crate::bounding_box::BoundingBox::new(
                [f64::INFINITY; 3],
                [f64::NEG_INFINITY; 3],
            );
        }

        crate::bounding_box::BoundingBox::new(lower, upper)
    }
}

impl RegionExpr {
    pub fn evaluate_contains(&self, point: (f64, f64, f64)) -> bool {
        match self {
            RegionExpr::Halfspace(hs) => match hs {
                HalfspaceType::Above(surf) => surf.evaluate(point) > 0.0,
                HalfspaceType::Below(surf) => surf.evaluate(point) < 0.0,
            },
            RegionExpr::Union(a, b) => a.evaluate_contains(point) || b.evaluate_contains(point),
            RegionExpr::Intersection(a, b) => a.evaluate_contains(point) && b.evaluate_contains(point),
            RegionExpr::Complement(inner) => !inner.evaluate_contains(point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::{Surface, SurfaceKind};
    use std::collections::HashMap;

    #[test]
    fn test_region_contains() {
        // Create two surfaces
        let s1 = Surface { surface_id: 1, kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: -5.0 }, boundary_type: crate::surface::BoundaryType::default() };
        let s2 = Surface { surface_id: 2, kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 3.0 }, boundary_type: crate::surface::BoundaryType::default() };

        // Map of surfaces by surface_id
        let mut surfaces = HashMap::new();
        surfaces.insert(s1.surface_id, s1.clone());
        surfaces.insert(s2.surface_id, s2.clone());

        // Build a region: inside s2 AND above s1
        let region = Region::new_from_halfspace(crate::region::HalfspaceType::Above(Arc::new(s1.clone())))
            .intersection(&Region::new_from_halfspace(crate::region::HalfspaceType::Below(Arc::new(s2.clone()))));

        // Test a point inside both
        let point = (0.0, 0.0, 0.0);
        assert!(region.contains(point));

        // Test a point outside the sphere
        let point = (0.0, 0.0, 4.0);
        assert!(!region.contains(point));
    }

    #[test]
    fn test_sphere_bounding_box() {
        // Sphere of radius 2 at (0,0,0)
        let s = Surface { surface_id: 1, kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 2.0 }, boundary_type: crate::surface::BoundaryType::default() };
        let mut surfaces = HashMap::new();
        surfaces.insert(s.surface_id, s.clone());
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s.clone())));
        let bbox = region.bounding_box();
        assert_eq!(bbox.lower_left, [-2.0, -2.0, -2.0]);
        assert_eq!(bbox.upper_right, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_box_and_sphere_bounding_box() {
        // XPlanes at x=2.1 and x=-2.1, sphere at origin with radius 4.2
        let s1 = Surface { surface_id: 1, kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: 2.1 }, boundary_type: crate::surface::BoundaryType::default() };
        let s2 = Surface { surface_id: 2, kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: -2.1 }, boundary_type: crate::surface::BoundaryType::default() };
        let s3 = Surface { surface_id: 3, kind: SurfaceKind::Sphere { x0: 0.0, y0: 0.0, z0: 0.0, radius: 4.2 }, boundary_type: crate::surface::BoundaryType::default() };
        let mut surfaces = HashMap::new();
        surfaces.insert(s1.surface_id, s1.clone());
        surfaces.insert(s2.surface_id, s2.clone());
        surfaces.insert(s3.surface_id, s3.clone());
        // Region: x >= -2.1 & x <= 2.1 & inside sphere
        let region = Region::new_from_halfspace(HalfspaceType::Above(Arc::new(s2.clone())))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s1.clone()))))
            .intersection(&Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s3.clone()))));
        let bbox = region.bounding_box();
        assert_eq!(bbox.lower_left, [-2.1, -4.2, -4.2]);
        assert_eq!(bbox.upper_right, [2.1, 4.2, 4.2]);
    }

    #[test]
    fn test_zplane_bounding_box() {
        // ZPlane at z=3.5
        let s = Surface { surface_id: 1, kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: 3.5 }, boundary_type: crate::surface::BoundaryType::default() };
        let mut surfaces = HashMap::new();
        surfaces.insert(s.surface_id, s.clone());
        // Region: z < 3.5 (Below ZPlane)
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s.clone())));
        let bbox = region.bounding_box();
        assert_eq!(bbox.lower_left[2], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[2], 3.5);
        assert_eq!(bbox.lower_left[0], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[0], f64::INFINITY);
        assert_eq!(bbox.lower_left[1], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[1], f64::INFINITY);
    }

    #[test]
    fn test_xplane_bounding_box() {
        // XPlane at x=1.5
        let s = Surface { surface_id: 1, kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: 1.5 }, boundary_type: crate::surface::BoundaryType::default() };
        let mut surfaces = HashMap::new();
        surfaces.insert(s.surface_id, s.clone());
        // Region: x < 1.5 (Below XPlane)
        let region = Region::new_from_halfspace(HalfspaceType::Below(Arc::new(s.clone())));
        let bbox = region.bounding_box();
        assert_eq!(bbox.lower_left[0], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[0], 1.5);
        assert_eq!(bbox.lower_left[1], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[1], f64::INFINITY);
        assert_eq!(bbox.lower_left[2], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[2], f64::INFINITY);
    }

    #[test]
    fn test_yplane_bounding_box() {
        // YPlane at y=-2.0
        let s = Surface { surface_id: 1, kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: -2.0 }, boundary_type: crate::surface::BoundaryType::default() };
        let mut surfaces = HashMap::new();
        surfaces.insert(s.surface_id, s.clone());
        // Region: y > -2.0 (Above YPlane)
        let region = Region::new_from_halfspace(HalfspaceType::Above(Arc::new(s.clone())));
        let bbox = region.bounding_box();
        assert_eq!(bbox.lower_left[1], -2.0);
        assert_eq!(bbox.upper_right[1], f64::INFINITY);
        assert_eq!(bbox.lower_left[0], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[0], f64::INFINITY);
        assert_eq!(bbox.lower_left[2], f64::NEG_INFINITY);
        assert_eq!(bbox.upper_right[2], f64::INFINITY);
    }
}
