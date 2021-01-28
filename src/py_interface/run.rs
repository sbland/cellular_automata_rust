extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::process_runner::state::CellStateBase;
use crate::py_interface::cell_state::CellStatePyBase;
use crate::py_interface::examples::CellStatePy;
use crate::py_interface::global_state::GlobalStatePy;

use crate::process_runner::example_processes::default_processes;
use crate::process_runner::process::Process;
use crate::process_runner::run::run_iteration;
use crate::process_runner::state::CellIndex;
use crate::process_runner::state::IterationState;

/// Wrap the run_iteration function so we can perform python object conversions
///
/// This is the API endpoint when using this library in other rust libraries
///
/// This function contains all the logic that takes the inputs from the python
/// interface function and passes it to the rust `run_iteration` function.
///
/// By seperating this from the `run_iteration_py` function we can allow non python arguments
///
pub fn run_iteration_wrap<T: CellStateBase + Copy + std::fmt::Debug, S: CellStatePyBase<T>>(
    cell_data: Vec<S>,
    global_state: GlobalStatePy,
    processes: Vec<Process<T>>,
) -> PyResult<(Vec<S>, GlobalStatePy, Vec<Vec<u32>>)> {
    // 1. Get the processes that are to be used.
    // let processes = processes_in.into().unwrap_or(default_processes());

    // 2. Extract the CellState from the CellStatePy wrapper
    let cell_data_inner = cell_data.iter().map(|c| c.get_inner()).collect::<Vec<_>>();

    // 3. Setup the full iteration state to pass to the run iteration function
    let initial_state = IterationState {
        global_state: global_state.inner,
        cells: cell_data_inner,
        // TODO: Network currently reset before each iteration then recalculated inside run_iteration
        network: vec![vec![]],
    };

    // 4. Run the iteration
    let out_state: IterationState<T> = run_iteration(&processes, initial_state);

    // 5. Wrap the cells state back up in the CellStatePy wrapper
    let cell_data_outer: Vec<S> = out_state
        .cells
        .iter()
        .map(|c| S::from_inner(c))
        .collect::<Vec<_>>();

    let network_converted: Vec<Vec<u32>> = out_state
        .network
        .iter()
        .map(|c| c.iter().map(|ci: &CellIndex| u32::from(*ci)).collect())
        .collect::<Vec<Vec<u32>>>();

    // 6. Wrap the global state in the GlobalStatePy wrapper
    let global_state_output = GlobalStatePy::inner(out_state.global_state);
    Ok((cell_data_outer, global_state_output, network_converted))
}

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
    run_iteration_wrap(cell_data, global_state, default_processes())
}

pub fn run_submodule(py: Python) -> PyResult<&PyModule> {
    let submod = PyModule::new(py, "run")?;
    submod.add("run_iteration", wrap_pyfunction!(run_iteration_py, submod)?)?;
    Ok(submod)
}
