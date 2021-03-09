/// Run Module
use super::cells;
use super::cells::run::apply_cell_updates;
use super::cells::run::Process as CellProcess;
use super::cells::state::CellIndex;
use super::cells::state::CellStateBase;
use super::global;
use super::global::run::Process as GlobalProcess;
use super::global::state::GlobalStateBase;
use super::network::get_network_map;
use super::state::IterationState;

/// Run a single iteration of the model
///
/// Can run either in full parallel, or update per process
/// In full parallel each process is run on each cell before performing any updates
/// in update_per_process
pub fn run_iteration<C: CellStateBase, G: GlobalStateBase>(
    cell_processes: &Vec<CellProcess<C, G>>,
    global_processes: &Vec<GlobalProcess<C, G>>,
    input_state: IterationState<C, G>,
    update_per_process: bool,
) -> IterationState<C, G> {
    let network: Vec<Vec<CellIndex>> = get_network_map::<C>(&input_state.cells);
    let mut current_state = input_state;

    let mut updated_cells = current_state.cells;
    if update_per_process {
        // For process in cell_processes run
        for process in cell_processes.iter() {
            let cell_updates = cells::run::run_process_on_cells::<C, G>(
                &updated_cells,
                &network,
                process,
                &current_state.global_state,
            );
            updated_cells = apply_cell_updates::<C>(updated_cells, cell_updates);
        }
    } else {
        let cell_updates = cells::run::run_processes::<C, G>(
            &updated_cells,
            &network,
            &cell_processes.iter().collect(),
            &current_state.global_state,
        );
        updated_cells = apply_cell_updates::<C>(updated_cells, cell_updates);
    }
    let updated_global_state = global::run::run_processes::<C, G>(
        &updated_cells.iter().collect(),
        &network,
        &global_processes,
        current_state.global_state,
    );

    // Update state
    current_state.global_state = updated_global_state;
    current_state.cells = updated_cells;
    current_state.network = network;
    current_state
}
