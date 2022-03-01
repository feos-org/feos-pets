use feos_core::python::joback::PyJobackRecord;
use feos_core::python::parameter::*;
use feos_core::python::*;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use quantity::python::PyInit_quantity;

mod eos;
use eos::*;
mod dft;
use dft::*;
mod parameters;
use parameters::*;

#[pymodule]
pub fn feos_pets(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIdentifier>()?;
    m.add_class::<PyVerbosity>()?;
    m.add_class::<PyContributions>()?;
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
sys.modules['feos_pets.eos.utils'] = eos.utils
sys.modules['feos_pets.dft'] = dft
sys.modules['feos_pets.dft.utils'] = dft.utils
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
