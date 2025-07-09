use pyo3::prelude::*;

use crate::surface::{Surface, SurfaceKind};
use crate::region_python::{PyRegion, PyHalfspace, PySurface};

#[pymethods]
impl PySurface {
    #[new]
    pub fn new(a: f64, b: f64, c: f64, d: f64, id: Option<usize>) -> PyResult<Self> {
        Ok(PySurface {
            inner: Surface {
                id: id.unwrap_or(0),
                kind: SurfaceKind::Plane { a, b, c, d },
            }
        })
    }

    #[staticmethod]
    pub fn sphere(center: (f64, f64, f64), radius: f64, id: Option<usize>) -> PyResult<Self> {
        Ok(PySurface {
            inner: Surface {
                id: id.unwrap_or(0),
                kind: SurfaceKind::Sphere {
                    center: [center.0, center.1, center.2],
                    radius,
                }
            }
        })
    }

    #[staticmethod]
    pub fn cylinder(axis: (f64, f64, f64), origin: (f64, f64, f64), radius: f64, id: Option<usize>) -> PyResult<Self> {
        Ok(PySurface {
            inner: Surface {
                id: id.unwrap_or(0),
                kind: SurfaceKind::Cylinder {
                    axis: [axis.0, axis.1, axis.2],
                    origin: [origin.0, origin.1, origin.2],
                    radius,
                }
            }
        })
    }

    pub fn evaluate(&self, point: (f64, f64, f64)) -> f64 {
        // Call the core Rust implementation
        self.inner.evaluate(point)
    }

    fn __neg__(slf: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace::new_below(slf.inner.id))
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace::new_above(slf.inner.id))
    }
}


