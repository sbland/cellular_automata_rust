extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod process_runner;
mod py_interface;

use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::state::IterationState;
use process_runner::Process;

use py_interface::CellStatePy;
use py_interface::GlobalStatePy;

//// A wrapper interface for the run_iteration function
///
/// This wraps the run_iteration function.
#[pyfunction]
fn run_iteration_py(
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

#[pymodule]
fn cellular_automata(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("run_iteration", wrap_pyfunction!(run_iteration_py, m)?)?;
    m.add_class::<CellStatePy>()?;
    m.add_class::<GlobalStatePy>()?;
    Ok(())
}
