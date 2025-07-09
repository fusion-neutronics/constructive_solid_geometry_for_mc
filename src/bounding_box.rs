#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub lower_left_corner: [f64; 3],
    pub upper_right_corner: [f64; 3],
}
