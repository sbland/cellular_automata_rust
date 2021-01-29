extern crate pyo3;
use pyo3::prelude::*;

use crate::process_runner::cells::run::Process as CellProcess;
use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::cells::state::CellStateBase;
use crate::process_runner::global::run::Process as GlobalProcess;
use crate::process_runner::global::state::GlobalStateBase;
use crate::process_runner::run::run_iteration;
use crate::process_runner::state::IterationState;
use crate::py_interface::cell_state::CellStatePyBase;
use crate::py_interface::global_state::GlobalStatePyBase;

/// Wrap the run_iteration function so we can perform python object conversions
///
/// This is the API endpoint when using this library in other rust libraries
///
/// This function contains all the logic that takes the inputs from the python
/// interface function and passes it to the rust `run_iteration` function.
///
/// By seperating this from the `run_iteration_py` function we can allow non python arguments
///
pub fn run_iteration_wrap<
    T: CellStateBase,
    S: CellStatePyBase<T>,
    G: GlobalStateBase,
    GW: GlobalStatePyBase<G>,
>(
    cell_data: Vec<S>,
    global_state: GW,
    cell_processes: Vec<CellProcess<T, G>>,
    global_processes: Vec<GlobalProcess<T, G>>,
) -> PyResult<(Vec<S>, GW, Vec<Vec<u32>>)> {
    // 1. Get the processes that are to be used.
    // let processes = processes_in.into().unwrap_or(default_processes());

    // 2. Extract the CellState from the CellStatePy wrapper
    let cell_data_inner = cell_data.iter().map(|c| c.get_inner()).collect::<Vec<_>>();

    // 3. Setup the full iteration state to pass to the run iteration function
    let initial_state = IterationState {
        global_state: global_state.get_inner(),
        cells: cell_data_inner,
        // TODO: Network currently reset before each iteration then recalculated inside run_iteration
        network: vec![vec![]],
    };

    // 4. Run the iteration
    let out_state: IterationState<T, G> =
        run_iteration(&cell_processes, &global_processes, initial_state);

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
    let global_state_output = GW::from_inner(&out_state.global_state);
    Ok((cell_data_outer, global_state_output, network_converted))
}
