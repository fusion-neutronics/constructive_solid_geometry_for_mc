use crate::geometry::Geometry;
use crate::cell_python::PyCell;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyclass(name = "Geometry"))]
#[derive(Clone)]
pub struct PyGeometry {
    pub inner: Geometry,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl PyGeometry {
    #[new]
    pub fn new(cells: Vec<PyCell>) -> Self {
        let rust_cells = cells.into_iter().map(|pycell| pycell.inner).collect();
        PyGeometry { inner: Geometry { cells: rust_cells } }
    }

    pub fn find_cell(&self, x: f64, y: f64, z: f64) -> Option<PyCell> {
        self.inner.find_cell((x, y, z)).cloned().map(|cell| PyCell { inner: cell })
    }
}
