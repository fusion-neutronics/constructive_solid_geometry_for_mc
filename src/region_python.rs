use pyo3::prelude::*;
use std::collections::HashMap;

use crate::region::{Region, RegionExpr, HalfspaceType};
use crate::surface::Surface;
use crate::surface_python::PySurface;

#[pyclass(name = "Region")]
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

    pub fn bounding_box(&self, surfaces: &PyAny) -> PyResult<PyBoundingBox> {
        let mut surf_map = HashMap::new();
        let items = surfaces.call_method0("items")?;
        let iter = items.iter()?;
        for item_result in iter {
            let item = item_result?;
            let key: usize = item.get_item(0)?.extract()?;
            let value: PyRef<PySurface> = item.get_item(1)?.extract()?;
            surf_map.insert(key, value.inner.clone());
        }
        let bbox = self.inner.bounding_box(&surf_map);
        Ok(PyBoundingBox {
            lower_left_corner: bbox.lower_left_corner,
            upper_right_corner: bbox.upper_right_corner,
            center: bbox.center,
            width: bbox.width,
        })
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Intersection(Box::new(self.inner.expr.clone()), Box::new(other_region.inner.expr.clone())),
                }
            })
        } else if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Intersection(Box::new(self.inner.expr.clone()), Box::new(other_halfspace.inner.expr.clone())),
                }
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Union(Box::new(self.inner.expr.clone()), Box::new(other_region.inner.expr.clone())),
                }
            })
        } else if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Union(Box::new(self.inner.expr.clone()), Box::new(other_halfspace.inner.expr.clone())),
                }
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }
}

#[pyclass]
pub struct PyBoundingBox {
    #[pyo3(get)]
    pub lower_left_corner: [f64; 3],
    #[pyo3(get)]
    pub upper_right_corner: [f64; 3],
    #[pyo3(get)]
    pub center: [f64; 3],
    #[pyo3(get)]
    pub width: [f64; 3],
}

#[pymethods]
impl PyBoundingBox {
    // Optionally, add __repr__ for pretty printing
    fn __repr__(&self) -> String {
        format!(
            "BoundingBox(lower_left_corner={:?}, upper_right_corner={:?}, center={:?}, width={:?})",
            self.lower_left_corner, self.upper_right_corner, self.center, self.width
        )
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

    fn __invert__(self_: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace {
            inner: Region {
                expr: RegionExpr::Complement(Box::new(self_.inner.expr.clone())),
            }
        })
    }

    pub fn contains(&self, point: (f64, f64, f64), surfaces: &PyAny) -> PyResult<bool> {
        let mut surf_map = std::collections::HashMap::new();
        let items = surfaces.call_method0("items")?;
        let iter = items.iter()?;
        for item_result in iter {
            let item = item_result?;
            let key: usize = item.get_item(0)?.extract()?;
            let value: PyRef<crate::surface_python::PySurface> = item.get_item(1)?.extract()?;
            surf_map.insert(key, value.inner.clone());
        }
        Ok(self.inner.expr.evaluate_contains(point, &surf_map))
    }

    pub fn bounding_box(&self, surfaces: &PyAny) -> PyResult<PyBoundingBox> {
        let mut surf_map = HashMap::new();
        let items = surfaces.call_method0("items")?;
        let iter = items.iter()?;
        for item_result in iter {
            let item = item_result?;
            let key: usize = item.get_item(0)?.extract()?;
            let value: PyRef<crate::surface_python::PySurface> = item.get_item(1)?.extract()?;
            surf_map.insert(key, value.inner.clone());
        }
        let bbox = self.inner.bounding_box(&surf_map);
        Ok(PyBoundingBox {
            lower_left_corner: bbox.lower_left_corner,
            upper_right_corner: bbox.upper_right_corner,
            center: bbox.center,
            width: bbox.width,
        })
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Intersection(Box::new(self.inner.expr.clone()), Box::new(other_halfspace.inner.expr.clone())),
                }
            })
        } else if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Intersection(Box::new(self.inner.expr.clone()), Box::new(other_region.inner.expr.clone())),
                }
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Union(Box::new(self.inner.expr.clone()), Box::new(other_halfspace.inner.expr.clone())),
                }
            })
        } else if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                inner: Region {
                    expr: RegionExpr::Union(Box::new(self.inner.expr.clone()), Box::new(other_region.inner.expr.clone())),
                }
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }
}
