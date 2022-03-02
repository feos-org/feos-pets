use crate::parameters::{PetsBinaryRecord, PetsParameters, PetsRecord};
use feos_core::joback::JobackRecord;
use feos_core::parameter::{Identifier, IdentifierOption, Parameter, ParameterError, PureRecord};
use feos_core::python::joback::PyJobackRecord;
use feos_core::python::parameter::PyIdentifier;
use feos_core::*;
use ndarray::Array2;

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
    // Create a set of PeTS parameters from lists.
    ///
    /// Parameters
    /// ----------
    /// sigma : List[float]
    ///     PeTS segment diameter in units of Angstrom.
    /// epsilon_k : List[float]
    ///     PeTS energy parameter in units of Kelvin.
    /// k_ij: numpy.ndarray[float]
    ///     matrix of binary interaction parameters.
    /// molarweight: List[float], optional
    ///     molar weight in units of Gram per Mol.
    /// viscosity: List[List[float]], optional
    ///     entropy scaling parameters for viscosity.
    /// diffusion: List[List[float]], optional
    ///     entropy scaling parameters for self-diffusion.
    /// thermal_conductivity: List[List[float]], optional
    ///     entropy scaling parameters for thermal conductivity.
    /// Returns
    /// -------
    /// PetsParameters
    #[pyo3(
        text_signature = "(sigma, epsilon_k, k_ij=None, molarweight=None, viscosity=None, diffusion=None, thermal_conductivity=None)"
    )]
    #[staticmethod]
    fn from_lists(
        sigma: Vec<f64>,
        epsilon_k: Vec<f64>,
        k_ij: Option<&PyArray2<f64>>,
        molarweight: Option<Vec<f64>>,
        viscosity: Option<Vec<[f64; 4]>>,
        diffusion: Option<Vec<[f64; 5]>>,
        thermal_conductivity: Option<Vec<[f64; 4]>>,
    ) -> Self {
        let n = sigma.len();
        let pure_records = (0..n)
            .map(|i| {
                let identifier =
                    Identifier::new(format!("{}", i).as_str(), None, None, None, None, None);
                let model_record = PetsRecord::new(
                    sigma[i],
                    epsilon_k[i],
                    viscosity.as_ref().map_or(None, |v| Some(v[i])),
                    diffusion.as_ref().map_or(None, |v| Some(v[i])),
                    thermal_conductivity.as_ref().map_or(None, |v| Some(v[i])),
                );
                PureRecord::new(
                    identifier,
                    molarweight.as_ref().map_or(1.0, |v| v[i]),
                    model_record,
                    None,
                )
                // Hier Ideal Gas anstatt None???
            })
            .collect();

        let binary = match k_ij {
            Some(v) => v.to_owned_array().mapv(f64::into),
            None => Array2::from_shape_fn((n, n), |(_, _)| PetsBinaryRecord::from(0.0)),
        };

        Self(Rc::new(PetsParameters::from_records(pure_records, binary)))
    }

    // Create a set of PeTS parameters from values.
    ///
    /// Parameters
    /// ----------
    /// sigma : float
    ///     PeTS segment diameter in units of Angstrom.
    /// epsilon_k : float
    ///     PeTS energy parameter in units of Kelvin.
    /// molarweight: float, optional
    ///     molar weight in units of Gram per Mol.
    /// viscosity: List[float], optional
    ///     entropy scaling parameters for viscosity.
    /// diffusion: List[float], optional
    ///     entropy scaling parameters for self-diffusion.
    /// thermal_conductivity: List[float], optional
    ///     entropy scaling parameters for thermal conductivity.
    /// Returns
    /// -------
    /// PetsParameters
    #[pyo3(
        text_signature = "(sigma, epsilon_k, molarweight=None, viscosity=None, diffusion=None, thermal_conductivity=None)"
    )]
    #[staticmethod]
    fn from_lists_pure(
        sigma: f64,
        epsilon_k: f64,
        molarweight: Option<f64>,
        viscosity: Option<[f64; 4]>,
        diffusion: Option<[f64; 5]>,
        thermal_conductivity: Option<[f64; 4]>,
    ) -> Self {
        let pure_records = vec![PureRecord::new(
            Identifier::new(format!("{}", 1).as_str(), None, None, None, None, None),
            molarweight.map_or(1.0, |v| v),
            PetsRecord::new(
                sigma,
                epsilon_k,
                viscosity.map_or(None, |v| Some(v)),
                diffusion.map_or(None, |v| Some(v)),
                thermal_conductivity.map_or(None, |v| Some(v)),
            ),
            None,
        )];

        let binary = Array2::from_shape_fn((1, 1), |(_, _)| PetsBinaryRecord::from(0.0));

        Self(Rc::new(PetsParameters::from_records(pure_records, binary)))
    }

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
