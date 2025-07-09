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
    #[cfg(not(feature = "python"))]
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
