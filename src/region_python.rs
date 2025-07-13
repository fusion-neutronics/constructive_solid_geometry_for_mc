use pyo3::prelude::*;
use std::collections::HashMap;

use crate::region::{Region, RegionExpr, HalfspaceType};
use crate::surface::Surface;
use crate::surface_python::PySurface;

#[pyclass(name = "Region")]
#[derive(Clone)]
pub struct PyRegion {
    pub expr: PyRegionExpr,
}

#[derive(Clone)]
pub enum PyRegionExpr {
    Halfspace(PyHalfspace),
    Union(Box<PyRegionExpr>, Box<PyRegionExpr>),
    Intersection(Box<PyRegionExpr>, Box<PyRegionExpr>),
    Complement(Box<PyRegionExpr>),
}

#[pymethods]
impl PyRegion {
    fn __invert__(self_: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyRegion {
            expr: PyRegionExpr::Complement(Box::new(self_.expr.clone())),
        })
    }

    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        self.expr.evaluate_contains(point)
    }

    pub fn bounding_box(&self) -> PyBoundingBox {
        self.expr.bounding_box()
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Intersection(Box::new(self.expr.clone()), Box::new(other_region.expr.clone())),
            })
        } else if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Intersection(Box::new(self.expr.clone()), Box::new(PyRegionExpr::Halfspace(other_halfspace.clone()))),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Union(Box::new(self.expr.clone()), Box::new(other_region.expr.clone())),
            })
        } else if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Union(Box::new(self.expr.clone()), Box::new(PyRegionExpr::Halfspace(other_halfspace.clone()))),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }
}

#[pyclass]
pub struct PyBoundingBox {
    #[pyo3(get)]
    pub lower_left: [f64; 3],
    #[pyo3(get)]
    pub upper_right: [f64; 3],
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
            "BoundingBox(lower_left={:?}, upper_right={:?}, center={:?}, width={:?})",
            self.lower_left, self.upper_right, self.center, self.width
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyHalfspace {
    pub surface: Py<PySurface>,
    pub is_above: bool,
}

#[pymethods]
impl PyHalfspace {
    #[staticmethod]
    pub fn new_above(surface: Py<PySurface>) -> Self {
        PyHalfspace { surface, is_above: true }
    }
    #[staticmethod]
    pub fn new_below(surface: Py<PySurface>) -> Self {
        PyHalfspace { surface, is_above: false }
    }
    fn __neg__(slf: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyHalfspace { surface: slf.surface.clone(), is_above: false })
    }
    fn __pos__(slf: PyRef<'_, Self>) -> PyResult<Self> {
        Ok(PyHalfspace { surface: slf.surface.clone(), is_above: true })
    }
    fn __invert__(slf: PyRef<'_, Self>) -> PyResult<PyRegion> {
        Ok(PyRegion {
            expr: PyRegionExpr::Complement(Box::new(PyRegionExpr::Halfspace(slf.clone())))
        })
    }
    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        Python::with_gil(|py| {
            let surface = self.surface.as_ref(py);
            if self.is_above {
                surface.borrow().evaluate(point) > 0.0
            } else {
                surface.borrow().evaluate(point) < 0.0
            }
        })
    }
    pub fn bounding_box(&self) -> PyBoundingBox {
        Python::with_gil(|py| {
            let surface = self.surface.as_ref(py);
            match &surface.borrow().inner.kind {
                crate::surface::SurfaceKind::Plane { a, b, c, d } => {
                    let mut lower = [f64::NEG_INFINITY; 3];
                    let mut upper = [f64::INFINITY; 3];
                    if *a == 1.0 && *b == 0.0 && *c == 0.0 {
                        if self.is_above {
                            lower[0] = *d;
                        } else {
                            upper[0] = *d;
                        }
                    } else if *a == 0.0 && *b == 1.0 && *c == 0.0 {
                        if self.is_above {
                            lower[1] = *d;
                        } else {
                            upper[1] = *d;
                        }
                    } else if *a == 0.0 && *b == 0.0 && *c == 1.0 {
                        if self.is_above {
                            lower[2] = *d;
                        } else {
                            upper[2] = *d;
                        }
                    }
                    PyBoundingBox {
                        lower_left: lower,
                        upper_right: upper,
                        center: [0.0, 0.0, 0.0],
                        width: [0.0, 0.0, 0.0],
                    }
                }
                crate::surface::SurfaceKind::Sphere { x0, y0, z0, radius } => {
                    PyBoundingBox {
                        lower_left: [*x0 - *radius, *y0 - *radius, *z0 - *radius],
                        upper_right: [*x0 + *radius, *y0 + *radius, *z0 + *radius],
                        center: [*x0, *y0, *z0],
                        width: [2.0 * *radius, 2.0 * *radius, 2.0 * *radius],
                    }
                }
                _ => PyBoundingBox {
                    lower_left: [f64::NEG_INFINITY; 3],
                    upper_right: [f64::INFINITY; 3],
                    center: [0.0, 0.0, 0.0],
                    width: [0.0, 0.0, 0.0],
                },
            }
        })
    }
    fn __and__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Intersection(Box::new(PyRegionExpr::Halfspace(self.clone())), Box::new(PyRegionExpr::Halfspace(other_halfspace.clone()))),
            })
        } else if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Intersection(Box::new(PyRegionExpr::Halfspace(self.clone())), Box::new(other_region.expr.clone())),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }
    fn __or__(&self, other: &PyAny) -> PyResult<PyRegion> {
        if let Ok(other_halfspace) = other.extract::<PyRef<PyHalfspace>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Union(Box::new(PyRegionExpr::Halfspace(self.clone())), Box::new(PyRegionExpr::Halfspace(other_halfspace.clone()))),
            })
        } else if let Ok(other_region) = other.extract::<PyRef<PyRegion>>() {
            Ok(PyRegion {
                expr: PyRegionExpr::Union(Box::new(PyRegionExpr::Halfspace(self.clone())), Box::new(other_region.expr.clone())),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Operand must be PyRegion or PyHalfspace"))
        }
    }
}

impl PyRegionExpr {
    pub fn evaluate_contains(&self, point: (f64, f64, f64)) -> bool {
        match self {
            PyRegionExpr::Halfspace(hs) => {
                Python::with_gil(|py| {
                    let surface = hs.surface.as_ref(py);
                    if hs.is_above {
                        surface.borrow().evaluate(point) > 0.0
                    } else {
                        surface.borrow().evaluate(point) < 0.0
                    }
                })
            }
            PyRegionExpr::Union(a, b) => a.evaluate_contains(point) || b.evaluate_contains(point),
            PyRegionExpr::Intersection(a, b) => a.evaluate_contains(point) && b.evaluate_contains(point),
            PyRegionExpr::Complement(inner) => !inner.evaluate_contains(point),
        }
    }
    pub fn bounding_box(&self) -> PyBoundingBox {
        match self {
            PyRegionExpr::Halfspace(hs) => hs.bounding_box(),
            PyRegionExpr::Intersection(a, b) => {
                let bbox_a = a.bounding_box();
                let bbox_b = b.bounding_box();
                let lower_left = [
                    bbox_a.lower_left[0].max(bbox_b.lower_left[0]),
                    bbox_a.lower_left[1].max(bbox_b.lower_left[1]),
                    bbox_a.lower_left[2].max(bbox_b.lower_left[2]),
                ];
                let upper_right = [
                    bbox_a.upper_right[0].min(bbox_b.upper_right[0]),
                    bbox_a.upper_right[1].min(bbox_b.upper_right[1]),
                    bbox_a.upper_right[2].min(bbox_b.upper_right[2]),
                ];
                PyBoundingBox {
                    lower_left,
                    upper_right,
                    center: [
                        (lower_left[0] + upper_right[0]) / 2.0,
                        (lower_left[1] + upper_right[1]) / 2.0,
                        (lower_left[2] + upper_right[2]) / 2.0,
                    ],
                    width: [
                        upper_right[0] - lower_left[0],
                        upper_right[1] - lower_left[1],
                        upper_right[2] - lower_left[2],
                    ],
                }
            }
            PyRegionExpr::Union(a, b) => {
                let bbox_a = a.bounding_box();
                let bbox_b = b.bounding_box();
                let lower_left = [
                    bbox_a.lower_left[0].min(bbox_b.lower_left[0]),
                    bbox_a.lower_left[1].min(bbox_b.lower_left[1]),
                    bbox_a.lower_left[2].min(bbox_b.lower_left[2]),
                ];
                let upper_right = [
                    bbox_a.upper_right[0].max(bbox_b.upper_right[0]),
                    bbox_a.upper_right[1].max(bbox_b.upper_right[1]),
                    bbox_a.upper_right[2].max(bbox_b.upper_right[2]),
                ];
                PyBoundingBox {
                    lower_left,
                    upper_right,
                    center: [
                        (lower_left[0] + upper_right[0]) / 2.0,
                        (lower_left[1] + upper_right[1]) / 2.0,
                        (lower_left[2] + upper_right[2]) / 2.0,
                    ],
                    width: [
                        upper_right[0] - lower_left[0],
                        upper_right[1] - lower_left[1],
                        upper_right[2] - lower_left[2],
                    ],
                }
            }
            PyRegionExpr::Complement(_) => {
                PyBoundingBox {
                    lower_left: [f64::NEG_INFINITY; 3],
                    upper_right: [f64::INFINITY; 3],
                    center: [0.0, 0.0, 0.0],
                    width: [f64::INFINITY; 3],
                }
            }
        }
    }
}
