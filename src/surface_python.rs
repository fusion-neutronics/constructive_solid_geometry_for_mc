use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::surface::{Surface, SurfaceKind, BoundaryType};
use crate::region_python::{PyRegion, PyHalfspace};

#[pyclass(name = "BoundaryType")]
#[derive(Clone)]
pub struct PyBoundaryType {
    pub inner: BoundaryType,
}

#[pymethods]
impl PyBoundaryType {
    #[new]
    fn new(boundary_type: &str) -> PyResult<Self> {
        let boundary = match boundary_type.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        };
        Ok(PyBoundaryType { inner: boundary })
    }

    fn __str__(&self) -> &str {
        match self.inner {
            BoundaryType::Transmission => "transmission",
            BoundaryType::Vacuum => "vacuum",
        }
    }

    fn __repr__(&self) -> String {
        format!("BoundaryType('{}')", self.__str__())
    }
}

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

    #[getter]
    pub fn boundary_type(&self) -> PyBoundaryType {
        PyBoundaryType { inner: self.inner.boundary_type().clone() }
    }

    #[setter(boundary_type)]
    pub fn set_boundary_type(&mut self, boundary_type: &str) -> PyResult<()> {
        let boundary = match boundary_type.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        };
        self.inner.set_boundary_type(boundary);
        Ok(())
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
pub fn XPlane(x0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface { 
        inner: Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 1.0, b: 0.0, c: 0.0, d: x0 },
            boundary_type: boundary,
        }
    })
}

#[pyfunction]
pub fn YPlane(y0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface { 
        inner: Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 1.0, c: 0.0, d: y0 },
            boundary_type: boundary,
        }
    })
}

#[pyfunction]
pub fn ZPlane(z0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface { 
        inner: Surface {
            surface_id,
            kind: SurfaceKind::Plane { a: 0.0, b: 0.0, c: 1.0, d: z0 },
            boundary_type: boundary,
        }
    })
}

#[pyfunction]
pub fn ZCylinder(x0: f64, y0: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface { 
        inner: Surface {
            surface_id,
            kind: SurfaceKind::Cylinder {
                axis: [0.0, 0.0, 1.0],
                origin: [x0, y0, 0.0],
                radius: r,
            },
            boundary_type: boundary,
        }
    })
}

#[pyfunction]
pub fn Sphere(x0: Option<f64>, y0: Option<f64>, z0: Option<f64>, r: f64, surface_id: Option<usize>, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Sphere {
                x0: x0.unwrap_or(0.0),
                y0: y0.unwrap_or(0.0),
                z0: z0.unwrap_or(0.0),
                radius: r,
            },
            boundary_type: boundary,
        },
    })
}

#[pyfunction]
pub fn Cylinder(axis: (f64, f64, f64), origin: (f64, f64, f64), r: f64, surface_id: Option<usize>, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Cylinder {
                axis: [axis.0, axis.1, axis.2],
                origin: [origin.0, origin.1, origin.2],
                radius: r,
            },
            boundary_type: boundary,
        },
    })
}

#[pyfunction]
pub fn Plane(a: f64, b: f64, c: f64, d: f64, surface_id: Option<usize>, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let boundary = match boundary_type {
        Some(bt) => match bt.to_lowercase().as_str() {
            "transmission" => BoundaryType::Transmission,
            "vacuum" => BoundaryType::Vacuum,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            )),
        },
        None => BoundaryType::default(),
    };
    Ok(PySurface {
        inner: Surface {
            surface_id: surface_id.unwrap_or(0),
            kind: SurfaceKind::Plane { a, b, c, d },
            boundary_type: boundary,
        },
    })
}


