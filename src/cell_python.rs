use pyo3::prelude::*;
use crate::cell::Cell;
use crate::region_python::PyRegion;

#[pyclass(name = "Cell")]
pub struct PyCell {
    pub inner: Cell,
}

#[pymethods]
impl PyCell {
    #[new]
    pub fn new(cell_id: u32, region: PyRegion, name: Option<String>) -> Self {
        PyCell {
            inner: Cell::new(cell_id, region.region, name),
        }
    }

    #[getter]
    pub fn cell_id(&self) -> u32 {
        self.inner.cell_id
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        self.inner.contains((x, y, z))
    }
}
