use crate::parameters::{PetsParameters, PetsRecord};
use feos_core::joback::JobackRecord;
use feos_core::parameter::{IdentifierOption, Parameter, ParameterError, PureRecord};
use feos_core::python::joback::PyJobackRecord;
use feos_core::python::parameter::PyIdentifier;
use feos_core::*;
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;
use std::convert::TryFrom;
use std::rc::Rc;

/// Create a set of PeTS parameters from records.
#[pyclass(name = "PetsRecord", unsendable)]
#[pyo3(
    text_signature = "(sigma, epsilon_k, viscosity=None, diffusion=None, thermal_conductivity=None)"
)]
#[derive(Clone)]
pub struct PyPetsRecord(PetsRecord);

#[pymethods]
impl PyPetsRecord {
    #[new]
    fn new(
        sigma: f64,
        epsilon_k: f64,
        viscosity: Option<[f64; 4]>,
        diffusion: Option<[f64; 5]>,
        thermal_conductivity: Option<[f64; 4]>,
    ) -> Self {
        Self(PetsRecord::new(
            sigma,
            epsilon_k,
            viscosity,
            diffusion,
            thermal_conductivity,
        ))
    }

    #[getter]
    fn get_sigma(&self) -> f64 {
        self.0.sigma
    }

    #[getter]
    fn get_epsilon_k(&self) -> f64 {
        self.0.epsilon_k
    }

    #[getter]
    fn get_viscosity(&self) -> Option<[f64; 4]> {
        self.0.viscosity
    }

    #[getter]
    fn get_diffusion(&self) -> Option<[f64; 5]> {
        self.0.diffusion
    }

    #[getter]
    fn get_thermal_conductivity(&self) -> Option<[f64; 4]> {
        self.0.thermal_conductivity
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PyPetsRecord {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

impl_json_handling!(PyPetsRecord);

impl_pure_record!(PetsRecord, PyPetsRecord, JobackRecord, PyJobackRecord);

/// Create a set of PeTS parameters from records.
///
/// Parameters
/// ----------
/// pure_records : List[PureRecord]
///     pure substance records.
/// binary_records : List[BinarySubstanceRecord], optional
///     binary PeTS parameter records
/// substances : List[str], optional
///     The substances to use. Filters substances from `pure_records` according to
///     `search_option`.
///     When not provided, all entries of `pure_records` are used.
/// search_option : {'Name', 'Cas', 'Inchi', 'IupacName', 'Formula', 'Smiles'}, optional, defaults to 'Name'.
///     Identifier that is used to search substance.
#[pyclass(name = "PetsParameters", unsendable)]
#[pyo3(
    text_signature = "(pure_records, binary_records=None, substances=None, search_option='Name')"
)]
#[derive(Clone)]
pub struct PyPetsParameters(pub Rc<PetsParameters>);

impl_parameter!(PetsParameters, PyPetsParameters);

#[pymethods]
impl PyPetsParameters {
    #[getter]
    fn get_pure_records(&self) -> Vec<PyPureRecord> {
        self.0
            .pure_records
            .iter()
            .map(|r| PyPureRecord(r.clone()))
            .collect()
    }

    #[getter]
    fn get_k_ij<'py>(&self, py: Python<'py>) -> &'py PyArray2<f64> {
        self.0.k_ij.view().to_pyarray(py)
    }

    fn _repr_markdown_(&self) -> String {
        self.0.to_markdown()
    }
}

#[pyproto]
impl pyo3::class::basic::PyObjectProtocol for PyPetsParameters {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}
