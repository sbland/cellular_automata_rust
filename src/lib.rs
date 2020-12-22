extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod process_runner;
mod py_interface;

use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::state::GlobalData;
use process_runner::state::IterationState;
use process_runner::Process;

use py_interface::CellStatePy;

#[pyfunction]
fn run_iteration_py(
    // cell: CellStatePy,
    cell_data: Vec<CellStatePy>,
    // global_data: GlobalData,
    // network_map: Vec<Vec<u32>>,
    // ) -> PyResult<Vec<CellStatePy>> {
) -> PyResult<Vec<CellStatePy>> {
    // ) -> PyResult<(Vec<CellState>, GlobalData, Vec<Vec<u32>>)> {
    let cell_data_inner = cell_data.iter().map(|c| c.inner).collect::<Vec<_>>();
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
        cells: cell_data_inner,
    };
    let processes = vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ];
    let final_state = run_iteration(&processes, initial_state);
    let cell_data_outer: Vec<CellStatePy> = final_state
        .cells
        .iter()
        .map(|c| CellStatePy { inner: *c })
        .collect::<Vec<_>>();
    Ok(cell_data_outer)
}

#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("run_iteration", wrap_pyfunction!(run_iteration_py, m)?)?;
    m.add_class::<CellStatePy>()?;
    Ok(())
}
