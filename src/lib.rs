#![warn(clippy::all)]
#![allow(clippy::ptr_arg)]

extern crate pyo3;
use pyo3::prelude::*;

pub mod process_runner;
pub mod py_interface;

use py_interface::example_run::run_submodule;
use py_interface::examples::CellStatePy;
use py_interface::examples::GlobalStatePy;

#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(run_submodule(py)?)?;
    m.add_class::<CellStatePy>()?;
    m.add_class::<GlobalStatePy>()?;
    Ok(())
}
