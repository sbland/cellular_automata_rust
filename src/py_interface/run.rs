extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::py_interface::cell_state::CellStatePy;
use crate::py_interface::global_state::GlobalStatePy;

use crate::process_runner::example_processes::example_process;
use crate::process_runner::example_processes::population_migration;
use crate::process_runner::process::Process;
use crate::process_runner::run::run_iteration;
use crate::process_runner::state::IterationState;

// Default example processes
pub fn default_processes() -> Vec<Process> {
    vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ]
}

/// Wrap the run_iteration function so we can perform python object conversions
///
/// This is the API endpoint when using this library in other rust libraries
///
/// This function contains all the logic that takes the inputs from the python
/// interface function and passes it to the rust `run_iteration` function.
///
/// By seperating this from the `run_iteration_py` function we can allow non python arguments
///
pub fn run_iteration_wrap(
    cell_data: Vec<CellStatePy>,
    global_state: GlobalStatePy,
    processes_in: Option<Vec<Process>>,
) -> PyResult<(Vec<CellStatePy>, GlobalStatePy, Vec<Vec<u32>>)> {
    // 1. Get the processes that are to be used.
    let processes = processes_in.unwrap_or(default_processes());

    // 2. Extract the CellState from the CellStatePy wrapper
    let cell_data_inner = cell_data.iter().map(|c| c.inner).collect::<Vec<_>>();

    // 3. Setup the full iteration state to pass to the run iteration function
    let initial_state = IterationState {
        global_state: global_state.inner,
        cells: cell_data_inner,
    };

    // 4. Run the iteration
    let out_state: IterationState = run_iteration(&processes, initial_state);

    // 5. Wrap the cells state back up in the CellStatePy wrapper
    let cell_data_outer: Vec<CellStatePy> = out_state
        .cells
        .iter()
        .map(|c| CellStatePy { inner: *c })
        .collect::<Vec<_>>();

    // 6. Wrap the global state in the GlobalStatePy wrapper
    let global_state_output = GlobalStatePy::inner(out_state.global_state);
    Ok((cell_data_outer, global_state_output, vec![vec![1u32]]))
}

/// The python facing run_iteration wrapper function
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
    run_iteration_wrap(cell_data, global_state, None)
}

pub fn run_submodule(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "run")?;
    submod.add("run_iteration", wrap_pyfunction!(run_iteration_py, submod)?)?;
    Ok(submod)
}
