
use crate::cell::Cell;
use crate::region_python::PyRegion;
use materials_for_mc::Material;
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(name = "Material")]
#[derive(Clone)]
pub struct PyMaterial {
    pub inner: Material,
}

#[pymethods]
impl PyMaterial {
    #[new]
    pub fn new(name: String) -> Self {
        PyMaterial {
            inner: Material {
                name: Some(name),
                nuclides: std::collections::HashMap::new(),
                density: None,
                density_units: "g/cm3".to_string(),
                volume: None,
                temperature: "293.6".to_string(),
                nuclide_data: std::collections::HashMap::new(),
                macroscopic_xs_neutron: std::collections::HashMap::new(),
                unified_energy_grid_neutron: Vec::new(),
                macroscopic_xs_neutron_total_by_nuclide: None,
            }
        }
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }
}

#[pyclass(name = "Cell")]
pub struct PyCell {
    pub inner: Cell,
}

#[pymethods]
impl PyCell {
    #[new]
    #[args(cell_id = "0", name = "None", fill = "None")]
    pub fn new(cell_id: u32, region: PyRegion, name: Option<String>, fill: Option<PyMaterial>) -> Self {
        let material = fill.map(|m| m.inner);
        PyCell {
            inner: Cell::new(cell_id, region.region, name, material),
        }
    }

    #[getter]
    pub fn cell_id(&self) -> u32 {
        self.inner.cell_id
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[getter]
    pub fn fill(&self) -> Option<PyMaterial> {
        self.inner.material.clone().map(|m| PyMaterial { inner: m })
    }

    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        self.inner.contains((x, y, z))
    }
}
