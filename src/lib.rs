extern crate pyo3;
use pyo3::prelude::*;

mod process_runner;
mod py_interface;

use py_interface::cell_state::CellStatePy;
use py_interface::global_state::GlobalStatePy;
use py_interface::run::run_submodule;

#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(run_submodule(py)?)?;
    m.add_class::<CellStatePy>()?;
    m.add_class::<GlobalStatePy>()?;
    Ok(())
}
