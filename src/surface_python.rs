use pyo3::prelude::*;
// ...existing code...

use crate::surface::{Surface, BoundaryType};
use crate::region_python::{PyHalfspace};

#[pyclass(name = "BoundaryType")]
#[derive(Clone)]
pub struct PyBoundaryType {
    pub inner: BoundaryType,
}

#[pymethods]
impl PyBoundaryType {
    #[new]
    fn new(boundary_type: &str) -> PyResult<Self> {
        let boundary = BoundaryType::from_str_option(boundary_type)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            ))?;
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
        let boundary = BoundaryType::from_str_option(boundary_type)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "boundary_type must be 'transmission' or 'vacuum'"
            ))?;
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
    let surface = Surface::x_plane_str(x0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn YPlane(y0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::y_plane_str(y0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn ZPlane(z0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::z_plane_str(z0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn ZCylinder(x0: f64, y0: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::z_cylinder_str(x0, y0, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn Sphere(x0: f64, y0: f64, z0: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::sphere_str(x0, y0, z0, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn Cylinder(x0: f64, y0: f64, z0: f64, axis_x: f64, axis_y: f64, axis_z: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::cylinder_str(x0, y0, z0, axis_x, axis_y, axis_z, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
pub fn Plane(a: f64, b: f64, c: f64, d: f64, surface_id: Option<usize>, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface_id = surface_id.unwrap_or(0);
    let surface = Surface::plane_str(a, b, c, d, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}


