use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::surface::{Surface, SurfaceKind};
use crate::region_python::{PyRegion, PyHalfspace};

#[pyclass(name = "Surface")]
#[derive(Clone)]
pub struct PySurface {
    pub inner: Surface,
}

#[pymethods]
impl PySurface {
    pub fn evaluate(&self, point: (f64, f64, f64)) -> f64 {
        // Call the core Rust implementation
        self.inner.evaluate(point)
    }

    #[getter]
    pub fn id(&self) -> usize {
        self.inner.surface_id
    }

    fn __neg__(slf: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace::new_below(slf.inner.surface_id))
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        Ok(PyHalfspace::new_above(slf.inner.surface_id))
    }
}

#[pyfunction]
pub fn XPlane(x0: f64, surface_id: usize) -> PySurface {
    PySurface { inner: crate::surface::Surface::x_plane(x0, surface_id) }
}
#[pyfunction]
pub fn YPlane(y0: f64, surface_id: usize) -> PySurface {
    PySurface { inner: crate::surface::Surface::y_plane(y0, surface_id) }
}
#[pyfunction]
pub fn ZPlane(z0: f64, surface_id: usize) -> PySurface {
    PySurface { inner: crate::surface::Surface::z_plane(z0, surface_id) }
}
#[pyfunction]
pub fn Sphere(center: (f64, f64, f64), radius: f64, surface_id: Option<usize>) -> PySurface {
    PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Sphere {
                center: [center.0, center.1, center.2],
                radius,
            },
        },
    }
}
#[pyfunction]
pub fn Cylinder(axis: (f64, f64, f64), origin: (f64, f64, f64), radius: f64, surface_id: Option<usize>) -> PySurface {
    PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Cylinder {
                axis: [axis.0, axis.1, axis.2],
                origin: [origin.0, origin.1, origin.2],
                radius,
            },
        },
    }
}
#[pyfunction]
pub fn Plane(a: f64, b: f64, c: f64, d: f64, surface_id: Option<usize>) -> PySurface {
    PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Plane { a, b, c, d },
        },
    }
}


