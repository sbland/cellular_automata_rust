extern crate pyo3;
use super::run::run_iteration_py_wrap;
use crate::process_runner::examples::example_processes::default_cell_processes;
use crate::py_interface::examples::CellStatePy;
use crate::py_interface::examples::GlobalStatePy;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// TODO: Move these to an example library
/// The python facing run_iteration wrapper function
///
/// EXAMPLE IMPLEMENTATION
/// Replace this in library
///
/// This wraps the run_iteration function for python.
/// It can only take python arguments and must return a PyResult object.
///
/// To create a new model with different processes we need to make a copy of this function.
#[pyfunction]
pub fn run_iteration_py(
    cell_data: Vec<CellStatePy>,
    global_state: GlobalStatePy,
) -> PyResult<(Vec<CellStatePy>, GlobalStatePy, Vec<Vec<u32>>)> {
    // TODO: Add global processes
    let global_processes = vec![];
    run_iteration_py_wrap(
        cell_data,
        global_state,
        default_cell_processes(),
        global_processes,
        true,
    )
}

pub fn run_submodule(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "run")?;
    submod.add("run_iteration", wrap_pyfunction!(run_iteration_py, submod)?)?;
    Ok(submod)
}
