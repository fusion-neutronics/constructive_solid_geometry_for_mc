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
        let py = slf.py();
        let py_surface: Py<PySurface> = slf.into_py(py).extract(py).unwrap();
        Ok(PyHalfspace::new_below(py_surface))
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyResult<PyHalfspace> {
        let py = slf.py();
        let py_surface: Py<PySurface> = slf.into_py(py).extract(py).unwrap();
        Ok(PyHalfspace::new_above(py_surface))
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
pub fn ZCylinder(x0: f64, y0: f64, radius: f64, surface_id: usize) -> PySurface {
    PySurface { inner: crate::surface::Surface::z_cylinder(x0, y0, radius, surface_id) }
}
#[pyfunction]
pub fn Sphere(x0: Option<f64>, y0: Option<f64>, z0: Option<f64>, r: f64, surface_id: Option<usize>) -> PySurface {
    PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Sphere {
                x0: x0.unwrap_or(0.0),
                y0: y0.unwrap_or(0.0),
                z0: z0.unwrap_or(0.0),
                radius: r,
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


