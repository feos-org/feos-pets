use feos_core::*;
use feos_dft::adsorption::*;
use feos_dft::fundamental_measure_theory::FMTVersion;
use feos_dft::interface::*;
use feos_dft::python::*;
use feos_dft::solvation::*;
use feos_dft::*;
use feos_pets::python::*;
use feos_pets::PetsFunctional;
use numpy::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use quantity::python::*;
use quantity::si::*;
use std::collections::HashMap;
use std::rc::Rc;

/// PeTS Helmholtz energy functional.
///
/// Parameters
/// ----------
/// parameters: PetsParameters
///     The set of PeTS parameters.
///
/// Returns
/// -------
/// PetsFunctional
#[pyclass(name = "PetsFunctional", unsendable)]
#[pyo3(text_signature = "(parameters)")]
#[derive(Clone)]
pub struct PyPetsFunctional(pub Rc<DFT<PetsFunctional>>);

#[pymethods]
impl PyPetsFunctional {
    #[new]
    fn new(parameters: PyPetsParameters) -> Self {
        Self(Rc::new(PetsFunctional::new(parameters.0)))
    }

    /// PeTS Helmholtz energy functional without simplifications
    /// for pure components.
    ///
    /// Parameters
    /// ----------
    /// parameters: PetsParameters
    ///     The set of PeTS parameters.
    /// fmt_version: FMTVersion
    ///     Specify the FMT term.
    ///
    /// Returns
    /// -------
    /// PetsFunctional
    #[staticmethod]
    #[pyo3(text_signature = "(parameters, fmt_version)")]
    fn new_full(parameters: PyPetsParameters, fmt_version: FMTVersion) -> Self {
        Self(Rc::new(PetsFunctional::new_full(parameters.0, fmt_version)))
    }
}

impl_equation_of_state!(PyPetsFunctional);

impl_state!(DFT<PetsFunctional>, PyPetsFunctional);
impl_state_molarweight!(DFT<PetsFunctional>, PyPetsFunctional);
impl_phase_equilibrium!(DFT<PetsFunctional>, PyPetsFunctional);

impl_planar_interface!(PetsFunctional);
impl_surface_tension_diagram!(PetsFunctional);

impl_pore!(PetsFunctional, PyPetsFunctional);
impl_adsorption!(PetsFunctional, PyPetsFunctional);

impl_pair_correlation!(PetsFunctional);
impl_solvation_profile!(PetsFunctional);

#[pymodule]
pub fn dft(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPetsFunctional>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyPhaseDiagram>()?;
    m.add_class::<PyPhaseEquilibrium>()?;
    m.add_class::<PyPlanarInterface>()?;
    m.add_class::<Geometry>()?;
    m.add_class::<PyPore1D>()?;
    m.add_class::<PyPore3D>()?;
    m.add_class::<PyPairCorrelation>()?;
    m.add_class::<PyExternalPotential>()?;
    m.add_class::<PyAdsorption1D>()?;
    m.add_class::<PyAdsorption3D>()?;
    m.add_class::<PySurfaceTensionDiagram>()?;
    m.add_class::<PyDFTSolver>()?;
    m.add_class::<PySolvationProfile>()?;
    m.add_class::<FMTVersion>()?;
    Ok(())
}
