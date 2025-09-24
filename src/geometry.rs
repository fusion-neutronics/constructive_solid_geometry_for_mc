use crate::cell::Cell;

/// Geometry is a collection of cells for Monte Carlo transport
pub struct Geometry {
    pub cells: Vec<Cell>,
}

impl Geometry {
    /// Find the first cell containing the given point, or None if not found
    pub fn find_cell(&self, point: (f64, f64, f64)) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.contains(point))
    }
}
