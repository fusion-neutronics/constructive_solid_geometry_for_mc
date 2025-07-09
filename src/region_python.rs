use pyo3::prelude::*;
use std::collections::HashMap;

use crate::region::{Region, RegionExpr, HalfspaceType};
use crate::surface::Surface;

#[pyclass]
#[derive(Clone)]
pub struct PyRegion {
    pub inner: Region,
}

#[pymethods]
impl PyRegion {
    #[new]
    pub fn new() -> Self {
        // Create an empty region for now
        PyRegion {
            inner: Region::new_from_halfspace(HalfspaceType::Above(0)),
        }
    }

    fn __and__(self_: PyRef<'_, Self>, other: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyRegion {
            inner: Region {
                expr: RegionExpr::Intersection(Box::new(self_.inner.expr.clone()), Box::new(other.inner.expr.clone())),
            }
        })
    }

    fn __or__(self_: PyRef<'_, Self>, other: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyRegion {
            inner: Region {
                expr: RegionExpr::Union(Box::new(self_.inner.expr.clone()), Box::new(other.inner.expr.clone())),
            }
        })
    }

    fn __invert__(self_: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyRegion {
            inner: Region {
                expr: RegionExpr::Complement(Box::new(self_.inner.expr.clone())),
            }
        })
    }

    pub fn contains(&self, point: (f64, f64, f64), surfaces: &PyAny) -> PyResult<bool> {
        // Extract Rust HashMap<usize, Surface> from Python dict-like `surfaces`
        let mut surf_map = HashMap::new();
        
        // Python dictionaries are iterated as key-value pairs
        let items = surfaces.call_method0("items")?;
        let iter = items.iter()?;
        
        for item_result in iter {
            let item = item_result?;
            let key: usize = item.get_item(0)?.extract()?;
            let value: PyRef<PySurface> = item.get_item(1)?.extract()?;
            surf_map.insert(key, value.inner.clone());
        }

        Ok(self.inner.expr.evaluate_contains(point, &surf_map))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyHalfspace {
    pub inner: Region,
}

#[pymethods]
impl PyHalfspace {
    #[staticmethod]
    pub fn new_above(id: usize) -> Self {
        PyHalfspace {
            inner: Region::new_from_halfspace(HalfspaceType::Above(id)),
        }
    }

    #[staticmethod]
    pub fn new_below(id: usize) -> Self {
        PyHalfspace {
            inner: Region::new_from_halfspace(HalfspaceType::Below(id)),
        }
    }

    fn __and__(self_: PyRef<'_, Self>, other: PyRef<'_, Self>) -> PyResult<PyRegion> {
        Ok(PyRegion {
            inner: Region {
                expr: RegionExpr::Intersection(Box::new(self_.inner.expr.clone()), Box::new(other.inner.expr.clone())),
            }
        })
    }

    fn __or__(self_: PyRef<'_, Self>, other: PyRef<'_, Self>) -> PyResult<PyRegion> {
        Ok(PyRegion {
            inner: Region {
                expr: RegionExpr::Union(Box::new(self_.inner.expr.clone()), Box::new(other.inner.expr.clone())),
            }
        })
    }

    fn __invert__(self_: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace {
            inner: Region {
                expr: RegionExpr::Complement(Box::new(self_.inner.expr.clone())),
            }
        })
    }
}

// This is used by surface_python.rs
#[pyclass]
#[derive(Clone)]
pub struct PySurface {
    pub inner: Surface,
}
