use feos_pets::python::feos_pets;
use pyo3::prelude::*;

#[pymodule]
pub fn build_wheel(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    feos_pets(py, m)
}
