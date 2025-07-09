pub mod surface;
pub mod region;

// Only include Python-specific code when the python feature is enabled
#[cfg(feature = "python")]
pub mod surface_python;
#[cfg(feature = "python")]
pub mod region_python;

// Re-export the public API for Rust users
pub use surface::Surface;
pub use region::{Region, RegionExpr, HalfspaceType};

// Only export the Python module when the python feature is enabled
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn mycsg(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add the classes with their Python-friendly names
    m.add_class::<region_python::PyRegion>()?;
    m.add_class::<region_python::PyHalfspace>()?;
    m.add_class::<surface_python::PySurface>()?;
    Ok(())
}
