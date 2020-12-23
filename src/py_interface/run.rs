extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::py_interface::cell_state::CellStatePy;
use crate::py_interface::global_state::GlobalStatePy;

use crate::process_runner::example_process;
use crate::process_runner::population_migration;
use crate::process_runner::run_iteration;
use crate::process_runner::state::IterationState;
use crate::process_runner::Process;

//// A wrapper interface for the run_iteration function
///
/// This wraps the run_iteration function.
#[pyfunction]
pub fn run_iteration_py(
    cell_data: Vec<CellStatePy>,
    global_state: GlobalStatePy,
    // network_map: Vec<Vec<u32>>,
    // ) -> PyResult<Vec<CellStatePy>> {
    // ) -> PyResult<Vec<CellStatePy>> {
) -> PyResult<(Vec<CellStatePy>, GlobalStatePy, Vec<Vec<u32>>)> {
    let cell_data_inner = cell_data.iter().map(|c| c.inner).collect::<Vec<_>>();
    let initial_state = IterationState {
        global_state: global_state.inner,
        cells: cell_data_inner,
    };
    let processes = vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ];
    let final_state: IterationState = run_iteration(&processes, initial_state);
    let cell_data_outer: Vec<CellStatePy> = final_state
        .cells
        .iter()
        .map(|c| CellStatePy { inner: *c })
        .collect::<Vec<_>>();
    let global_state_output = GlobalStatePy::inner(final_state.global_state);
    Ok((cell_data_outer, global_state_output, vec![vec![1u32]]))
}

pub fn run_submodule(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "run")?;
    submod.add("run_iteration", wrap_pyfunction!(run_iteration_py, submod)?)?;
    Ok(submod)
}
