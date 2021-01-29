use super::cells;
use super::cells::run::apply_cell_updates;
use super::cells::run::Process as CellProcess;
use super::cells::state::CellIndex;
use super::cells::state::CellStateBase;
use super::global;
use super::global::run::Process as GlobalProcess;
use super::global::state::GlobalStateBase;
/// Run Module
///
/// 'a lifetime represents a single iteration
///
use super::network::get_network_map;
use super::state::IterationState;

/// Run a single iteration of the model
pub fn run_iteration<C: CellStateBase, G: GlobalStateBase>(
    cell_processes: &Vec<CellProcess<C, G>>,
    global_processes: &Vec<GlobalProcess<C, G>>,
    input_state: IterationState<C, G>,
) -> IterationState<C, G> {
    let network: Vec<Vec<CellIndex>> = get_network_map::<C>(&input_state.cells);
    let mut current_state = input_state;
    let updated_global_state = global::run::run_processes::<C, G>(
        &current_state.cells.iter().collect(),
        &network,
        &global_processes,
        current_state.global_state,
    );

    let cell_updates = cells::run::run_processes::<C, G>(
        &current_state.cells,
        &network,
        &cell_processes,
        &updated_global_state,
    );

    let updated_cells = apply_cell_updates::<C>(current_state.cells, cell_updates);

    // Update state
    current_state.global_state = updated_global_state;
    current_state.cells = updated_cells;
    current_state.network = network;
    current_state
}
