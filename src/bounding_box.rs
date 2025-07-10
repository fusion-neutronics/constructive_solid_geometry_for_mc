#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub lower_left_corner: [f64; 3],
    pub upper_right_corner: [f64; 3],
    pub center: [f64; 3],
    pub width: [f64; 3],
}

impl BoundingBox {
    pub fn new(lower_left_corner: [f64; 3], upper_right_corner: [f64; 3]) -> Self {
        let center = [
            0.5 * (lower_left_corner[0] + upper_right_corner[0]),
            0.5 * (lower_left_corner[1] + upper_right_corner[1]),
            0.5 * (lower_left_corner[2] + upper_right_corner[2]),
        ];
        let width = [
            upper_right_corner[0] - lower_left_corner[0],
            upper_right_corner[1] - lower_left_corner[1],
            upper_right_corner[2] - lower_left_corner[2],
        ];
        BoundingBox {
            lower_left_corner,
            upper_right_corner,
            center,
            width,
        }
    }
}
