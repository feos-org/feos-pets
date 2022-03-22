use feos_core::*;
use feos_pets::{Pets, PetsOptions};
use feos_pets::python::PyPetsParameters;
use numpy::convert::ToPyArray;
use numpy::{PyArray1, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use quantity::python::*;
use quantity::si::*;
use std::collections::HashMap;
use std::rc::Rc;

/// Initialize PeTS equation of state.
///
/// Parameters
/// ----------
/// parameters : PetsParameters
///     The parameters of the PeTS equation of state to use.
/// max_eta : float, optional
///     Maximum packing fraction. Defaults to 0.5.
///
/// Returns
/// -------
/// Pets
///     The PeTS equation of state that can be used to compute thermodynamic
///     states.
#[pyclass(name = "Pets", unsendable)]
#[pyo3(text_signature = "(parameters, max_eta)")]
#[derive(Clone)]
pub struct PyPets(pub Rc<Pets>);

#[pymethods]
impl PyPets {
    #[new]
    #[args(max_eta = "0.5")]
    fn new(parameters: PyPetsParameters, max_eta: f64) -> Self {
        let options = PetsOptions { max_eta };
        Self(Rc::new(Pets::with_options(parameters.0.clone(), options)))
    }
}

impl_equation_of_state!(PyPets);
impl_virial_coefficients!(PyPets);

impl_state!(Pets, PyPets);
impl_state_molarweight!(Pets, PyPets);
impl_state_entropy_scaling!(Pets, PyPets);
impl_vle_state!(Pets, PyPets);

#[pymodule]
pub fn eos(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPets>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyPhaseDiagramPure>()?;
    m.add_class::<PyPhaseDiagramBinary>()?;
    m.add_class::<PyPhaseDiagramHetero>()?;
    m.add_class::<PyPhaseEquilibrium>()?;
    Ok(())
}
