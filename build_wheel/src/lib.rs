use feos_core::python::joback::PyJobackRecord;
use feos_core::python::parameter::*;
use feos_core::{Contributions, Verbosity};
use feos_pets::python::*;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use quantity::python::__PYO3_PYMODULE_DEF_QUANTITY;

mod dft;
mod eos;
use dft::__PYO3_PYMODULE_DEF_DFT;
use eos::__PYO3_PYMODULE_DEF_EOS;

#[pymodule]
pub fn feos_pcsaft(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIdentifier>()?;
    m.add_class::<Verbosity>()?;
    m.add_class::<Contributions>()?;
    m.add_class::<PyChemicalRecord>()?;
    m.add_class::<PyJobackRecord>()?;

    m.add_class::<PyPetsRecord>()?;
    m.add_class::<PyPureRecord>()?;
    m.add_class::<PyBinaryRecord>()?;
    m.add_class::<PyPetsParameters>()?;

    m.add_wrapped(wrap_pymodule!(eos))?;
    m.add_wrapped(wrap_pymodule!(dft))?;
    m.add_wrapped(wrap_pymodule!(quantity))?;

    py.run(
        "\
import sys
sys.modules['feos_pets.eos'] = eos
sys.modules['feos_pets.dft'] = dft
quantity.SINumber.__module__ = 'feos_pets.si'
quantity.SIArray1.__module__ = 'feos_pets.si'
quantity.SIArray2.__module__ = 'feos_pets.si'
quantity.SIArray3.__module__ = 'feos_pets.si'
quantity.SIArray4.__module__ = 'feos_pets.si'
sys.modules['feos_pets.si'] = quantity
    ",
        None,
        Some(m.dict()),
    )?;
    Ok(())
}
