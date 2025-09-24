pub mod bounding_box;
pub mod cell;
pub mod region;
pub mod surface;

// Only include Python-specific code when the pyo3 feature is enabled
#[cfg(feature = "pyo3")]
pub mod cell_python;
#[cfg(feature = "pyo3")]
pub mod region_python;
#[cfg(feature = "pyo3")]
pub mod surface_python;

// Re-export the public API for Rust users
pub use cell::Cell;
pub use region::{HalfspaceType, Region, RegionExpr};
pub use surface::{BoundaryType, Surface};

// Only export the Python module when the pyo3 feature is enabled
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
fn constructive_solid_geometry_for_mc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<region_python::PyRegion>()?;
    m.add_class::<region_python::PyHalfspace>()?;
    m.add_class::<cell_python::PyCell>()?;
    m.add_class::<cell_python::PyMaterial>()?;
    m.add_class::<surface_python::PyBoundaryType>()?;
    // Expose surface constructors at top level for OpenMC-style API
    use surface_python::{Cylinder, Plane, Sphere, XPlane, YPlane, ZCylinder, ZPlane};
    m.add_function(wrap_pyfunction!(XPlane, m)?)?;
    m.add_function(wrap_pyfunction!(YPlane, m)?)?;
    m.add_function(wrap_pyfunction!(ZPlane, m)?)?;
    m.add_function(wrap_pyfunction!(Sphere, m)?)?;
    m.add_function(wrap_pyfunction!(Cylinder, m)?)?;
    m.add_function(wrap_pyfunction!(ZCylinder, m)?)?;
    m.add_function(wrap_pyfunction!(Plane, m)?)?;
    Ok(())
}
