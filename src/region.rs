use std::collections::HashMap;
use crate::surface::Surface;

#[derive(Clone)]
pub struct Region {
    pub expr: RegionExpr,
}

#[derive(Clone)]
pub enum HalfspaceType {
    Above(usize),
    Below(usize),
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
    
    // Regular Rust version of contains that takes a HashMap directly
    pub fn contains(&self, point: (f64, f64, f64), surfaces: &HashMap<usize, Surface>) -> bool {
        self.expr.evaluate_contains(point, surfaces)
    }
    
    // Make this method available regardless of features for internal use
    pub fn evaluate_contains(&self, point: (f64, f64, f64), surfaces: &HashMap<usize, Surface>) -> bool {
        self.expr.evaluate_contains(point, surfaces)
    }
}

impl RegionExpr {
    pub fn evaluate_contains(&self, point: (f64, f64, f64), surfaces: &HashMap<usize, Surface>) -> bool {
        match self {
            RegionExpr::Halfspace(hs) => match hs {
                HalfspaceType::Above(id) => {
                    if let Some(s) = surfaces.get(id) {
                        s.evaluate(point) > 0.0
                    } else {
                        false
                    }
                }
                HalfspaceType::Below(id) => {
                    if let Some(s) = surfaces.get(id) {
                        s.evaluate(point) < 0.0
                    } else {
                        false
                    }
                }
            },
            RegionExpr::Union(a, b) => a.evaluate_contains(point, surfaces) || b.evaluate_contains(point, surfaces),
            RegionExpr::Intersection(a, b) => a.evaluate_contains(point, surfaces) && b.evaluate_contains(point, surfaces),
            RegionExpr::Complement(inner) => !inner.evaluate_contains(point, surfaces),
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
        let s1 = Surface { id: 1, kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: -5.0 } };
        let s2 = Surface { id: 2, kind: SurfaceKind::Sphere { center: [0.0, 0.0, 0.0], radius: 3.0 } };

        // Map of surfaces by id
        let mut surfaces = HashMap::new();
        surfaces.insert(s1.id, s1.clone());
        surfaces.insert(s2.id, s2.clone());

        // Build a region: inside s2 AND above s1
        let region = Region::new_from_halfspace(crate::region::HalfspaceType::Above(s1.id))
            .intersection(&Region::new_from_halfspace(crate::region::HalfspaceType::Below(s2.id)));

        // Test a point inside both
        let point = (0.0, 0.0, 0.0);
        assert!(region.contains(point, &surfaces));

        // Test a point outside the sphere
        let point = (0.0, 0.0, 4.0);
        assert!(!region.contains(point, &surfaces));
    }
}
