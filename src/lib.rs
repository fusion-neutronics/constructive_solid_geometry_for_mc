pub mod surface;
pub mod region;

// Only include Python-specific code when the pyo3 feature is enabled
#[cfg(feature = "pyo3")]
pub mod surface_python;
#[cfg(feature = "pyo3")]
pub mod region_python;

// Re-export the public API for Rust users
pub use surface::Surface;
pub use region::{Region, RegionExpr, HalfspaceType};

// Only export the Python module when the pyo3 feature is enabled
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
fn constructive_solid_geometry_for_mc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<surface_python::PySurface>()?;
    m.add_class::<region_python::PyRegion>()?;
    m.add_class::<region_python::PyHalfspace>()?;
    Ok(())
}
