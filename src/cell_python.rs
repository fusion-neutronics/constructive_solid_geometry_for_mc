use pyo3::prelude::*;
use std::collections::HashMap;

use crate::cell::{Cell, Universe, CellFill, MaterialRef};
use crate::region_python::PyRegion;

#[pyclass(name = "Cell")]
#[derive(Clone)]
pub struct PyCell {
    pub inner: Cell,
}

#[pymethods]
impl PyCell {
    #[new]
    #[pyo3(signature = (cell_id, region, material_id=None, name=None))]
    pub fn new(
        cell_id: u32,
        region: PyRegion,
        material_id: Option<u32>,
        name: Option<String>,
    ) -> PyResult<Self> {
        // Convert PyRegion to Region
        let rust_region = region.to_rust_region()?;
        
        let mut cell = if let Some(mat_id) = material_id {
            Cell::new_with_material(cell_id, rust_region, mat_id)
        } else {
            Cell::new_void(cell_id, rust_region)
        };

        if let Some(n) = name {
            cell = cell.with_name(n);
        }

        Ok(PyCell { inner: cell })
    }

    #[getter]
    pub fn cell_id(&self) -> u32 {
        self.inner.cell_id
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.inner.name = Some(name);
    }

    #[getter]
    pub fn material_id(&self) -> Option<u32> {
        self.inner.material_id()
    }

    #[getter]
    pub fn is_void(&self) -> bool {
        self.inner.is_void()
    }

    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        self.inner.contains(point)
    }

    pub fn bounding_box(&self) -> PyBoundingBox {
        let bbox = self.inner.bounding_box();
        PyBoundingBox {
            lower_left: bbox.lower_left,
            upper_right: bbox.upper_right,
            center: bbox.center(),
            width: bbox.width(),
        }
    }

    fn __repr__(&self) -> String {
        match (&self.inner.name, self.inner.material_id()) {
            (Some(name), Some(mat_id)) => {
                format!("Cell(id={}, name='{}', material={})", self.inner.cell_id, name, mat_id)
            }
            (Some(name), None) => {
                format!("Cell(id={}, name='{}', void)", self.inner.cell_id, name)
            }
            (None, Some(mat_id)) => {
                format!("Cell(id={}, material={})", self.inner.cell_id, mat_id)
            }
            (None, None) => {
                format!("Cell(id={}, void)", self.inner.cell_id)
            }
        }
    }
}

#[pyclass(name = "Universe")]
pub struct PyUniverse {
    pub inner: Universe,
}

#[pymethods]
impl PyUniverse {
    #[new]
    #[pyo3(signature = (universe_id, name=None))]
    pub fn new(universe_id: u32, name: Option<String>) -> Self {
        let mut universe = Universe::new(universe_id);
        if let Some(n) = name {
            universe = universe.with_name(n);
        }
        PyUniverse { inner: universe }
    }

    #[getter]
    pub fn universe_id(&self) -> u32 {
        self.inner.universe_id
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.inner.name = Some(name);
    }

    pub fn add_cell(&mut self, cell: PyCell) {
        self.inner.add_cell(cell.inner);
    }

    pub fn find_cell(&self, point: (f64, f64, f64)) -> Option<PyCell> {
        self.inner.find_cell(point).map(|cell| PyCell {
            inner: cell.clone(),
        })
    }

    pub fn get_cell(&self, cell_id: u32) -> Option<PyCell> {
        self.inner.get_cell(cell_id).map(|cell| PyCell {
            inner: cell.clone(),
        })
    }

    #[getter]
    pub fn cells(&self) -> HashMap<u32, PyCell> {
        self.inner
            .cells
            .iter()
            .map(|(id, cell)| (*id, PyCell { inner: cell.clone() }))
            .collect()
    }

    fn __repr__(&self) -> String {
        match &self.inner.name {
            Some(name) => format!(
                "Universe(id={}, name='{}', {} cells)",
                self.inner.universe_id,
                name,
                self.inner.cells.len()
            ),
            None => format!(
                "Universe(id={}, {} cells)",
                self.inner.universe_id,
                self.inner.cells.len()
            ),
        }
    }
}

// Re-export from region_python to avoid duplication
use crate::region_python::PyBoundingBox;

// Implementation to convert PyRegion to Rust Region
impl PyRegion {
    pub fn to_rust_region(&self) -> PyResult<crate::region::Region> {
        use crate::region::{Region, RegionExpr, HalfspaceType};
        use crate::region_python::{PyRegionExpr, PyHalfspace};
        use std::sync::Arc;
        
        fn convert_expr(expr: &PyRegionExpr) -> PyResult<RegionExpr> {
            match expr {
                PyRegionExpr::Halfspace(hs) => {
                    let surface = Python::with_gil(|py| {
                        hs.surface.borrow(py).inner.clone()
                    });
                    let halfspace_type = if hs.is_above {
                        HalfspaceType::Above(Arc::new(surface))
                    } else {
                        HalfspaceType::Below(Arc::new(surface))
                    };
                    Ok(RegionExpr::Halfspace(halfspace_type))
                }
                PyRegionExpr::Union(a, b) => {
                    let a_expr = convert_expr(a)?;
                    let b_expr = convert_expr(b)?;
                    Ok(RegionExpr::Union(Box::new(a_expr), Box::new(b_expr)))
                }
                PyRegionExpr::Intersection(a, b) => {
                    let a_expr = convert_expr(a)?;
                    let b_expr = convert_expr(b)?;
                    Ok(RegionExpr::Intersection(Box::new(a_expr), Box::new(b_expr)))
                }
                PyRegionExpr::Complement(inner) => {
                    let inner_expr = convert_expr(inner)?;
                    Ok(RegionExpr::Complement(Box::new(inner_expr)))
                }
            }
        }

        let rust_expr = convert_expr(&self.expr)?;
        Ok(Region { expr: rust_expr })
    }
}