mod surface;
mod region;

// Only include Python-specific code when the python feature is enabled
#[cfg(feature = "python")]
mod surface_python;
#[cfg(feature = "python")]
mod region_python;
#[cfg(feature = "python")]
use pyo3::prelude::*;

// Only export the Python module when the python feature is enabled
#[cfg(feature = "python")]
#[pymodule]
fn mycsg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<region_python::PyRegion>()?;
    m.add_class::<region_python::PyHalfspace>()?;
    m.add_class::<region_python::PySurface>()?;
    Ok(())
}
