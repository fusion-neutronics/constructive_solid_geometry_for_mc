#![allow(non_local_definitions)]

use pyo3::prelude::*;
// ...existing code...

use crate::surface::{Surface, BoundaryType};
use crate::region_python::{PyHalfspace, PyBoundingBox};

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

    /// Compute the distance from a point along a direction to the surface.
    /// Returns float if intersection exists and distance > 0, else None.
    pub fn distance_to_surface(&self, point: (f64, f64, f64), direction: (f64, f64, f64)) -> Option<f64> {
        self.inner.distance_to_surface([point.0, point.1, point.2], [direction.0, direction.1, direction.2])
    }
    pub fn evaluate(&self, point: (f64, f64, f64)) -> f64 {
        // Call the core Rust implementation
        self.inner.evaluate(point)
    }

    /// Get the bounding box for the inside (negative halfspace) of this surface
    pub fn bounding_box_inside(&self) -> Option<PyBoundingBox> {
        self.inner.bounding_box(true).map(|(lower, upper)| PyBoundingBox {
            lower_left: lower,
            upper_right: upper,
            center: [
                (lower[0] + upper[0]) / 2.0,
                (lower[1] + upper[1]) / 2.0,
                (lower[2] + upper[2]) / 2.0,
            ],
            width: [
                upper[0] - lower[0],
                upper[1] - lower[1],
                upper[2] - lower[2],
            ],
        })
    }

    pub fn bounding_box_outside(&self) -> Option<PyBoundingBox> {
        self.inner.bounding_box(false).map(|(lower, upper)| PyBoundingBox {
            lower_left: lower,
            upper_right: upper,
            center: [
                (lower[0] + upper[0]) / 2.0,
                (lower[1] + upper[1]) / 2.0,
                (lower[2] + upper[2]) / 2.0,
            ],
            width: [
                upper[0] - lower[0],
                upper[1] - lower[1],
                upper[2] - lower[2],
            ],
        })
    }

    /// Get axis constraint for this surface when used as a halfspace
    /// Returns (axis_index, is_upper_bound, value) or None
    /// axis_index: 0=X, 1=Y, 2=Z
    /// is_upper_bound: True if this constrains the upper bound, False for lower bound
    pub fn axis_constraint(&self, halfspace_below: bool) -> Option<(usize, bool, f64)> {
        self.inner.axis_constraint(halfspace_below)
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
#[allow(non_snake_case)]
pub fn XPlane(x0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::x_plane_str(x0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn YPlane(y0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::y_plane_str(y0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn ZPlane(z0: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::z_plane_str(z0, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn ZCylinder(x0: f64, y0: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::z_cylinder_str(x0, y0, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn Sphere(x0: f64, y0: f64, z0: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::sphere_str(x0, y0, z0, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn Cylinder(x0: f64, y0: f64, z0: f64, axis_x: f64, axis_y: f64, axis_z: f64, r: f64, surface_id: usize, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface = Surface::cylinder_str(x0, y0, z0, axis_x, axis_y, axis_z, r, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn Plane(a: f64, b: f64, c: f64, d: f64, surface_id: Option<usize>, boundary_type: Option<&str>) -> PyResult<PySurface> {
    let surface_id = surface_id.unwrap_or(0);
    let surface = Surface::plane_str(a, b, c, d, surface_id, boundary_type)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(PySurface { inner: surface })
}


