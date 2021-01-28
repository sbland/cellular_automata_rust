/// Run Module
///
/// 'a lifetime represents a single iteration
///
use crate::process_runner::network::get_network_map;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;
use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellStateBase;
use crate::process_runner::state::GlobalState;
use crate::process_runner::state::IterationState;

/// Run a single process on a single cell
pub fn run_process<T: CellStateBase + std::fmt::Debug>(
    cell: &T,
    process: &Process<T>,
    neighbours: &Vec<&T>, // A list of the neighbours states
) -> Vec<CellUpdate<T>> {
    let func = &process.func;
    let cell_updates: Vec<CellUpdate<T>> = func(&cell, &neighbours);
    cell_updates
}

/// Update the global state
#[allow(clippy::let_and_return)]
pub fn get_next_global_state(global_state: &GlobalState) -> GlobalState {
    let new_global_state = GlobalState {
        iterations: global_state.iterations + 1,
    };
    new_global_state
}

/// Run all processes on all cells
pub fn run_processes<T: CellStateBase + std::fmt::Debug>(
    cells: &Vec<T>,
    network: &Vec<Vec<CellIndex>>,
    processes: &Vec<Process<T>>,
) -> Vec<CellUpdate<T>> {
    let mut cell_updates: Vec<CellUpdate<T>> = Vec::new();
    for cell in cells.iter() {
        let cell_id: usize = cell.id().into();
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .iter()
            // Note we use tuple struct destructuring here
            .map(|CellIndex(id)| &cells[*id as usize])
            .collect::<Vec<_>>();
        for process in processes.iter() {
            let mut more_cell_updates = run_process::<T>(&cell, &process, &neighbours);
            cell_updates.append(&mut more_cell_updates);
        }
    }
    cell_updates
}

/// Apply all queued cell updates to the cells
pub fn apply_cell_updates<T: CellStateBase + Copy>(
    cells_in: Vec<T>,
    cell_updates: Vec<CellUpdate<T>>,
) -> Vec<T> {
    let mut modified_cells = cells_in;
    for cell_action in cell_updates.iter() {
        let id = cell_action.target_cell.0 as usize;
        modified_cells[id] = (cell_action.action)(modified_cells[id])
    }
    modified_cells
}

/// Run a single iteration of the model
pub fn run_iteration<T: CellStateBase + Copy + std::fmt::Debug>(
    processes: &Vec<Process<T>>,
    input_state: IterationState<T>,
) -> IterationState<T> {
    let mut new_state = input_state;
    let network: Vec<Vec<CellIndex>> = get_network_map::<T>(&new_state.cells);
    let cell_updates = run_processes(&new_state.cells, &network, &processes);
    let updated_cells = apply_cell_updates::<T>(new_state.cells, cell_updates);
    let updated_global_state = get_next_global_state(&new_state.global_state);

    // Update state
    new_state.global_state = updated_global_state;
    new_state.cells = updated_cells;
    new_state.network = network;
    new_state
}
