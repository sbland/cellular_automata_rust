/// Run Module
use super::cells;
use super::cells::run::apply_cell_updates;
use super::cells::run::Process as CellProcess;
use super::cells::state::CellIndex;
use super::cells::state::CellStateBase;
use super::global;
use super::global::run::apply_global_updates;
use super::global::run::Process as GlobalProcess;
use super::global::state::GlobalStateBase;
use super::network::get_network_map;
use super::state::IterationState;

#[allow(dead_code)]
pub fn setup_initial_state<C: CellStateBase, G: GlobalStateBase>(
    cell_setup_processes: Option<Vec<&CellProcess<C, G>>>,
    global_setup_processes: Option<Vec<&GlobalProcess<C, G>>>,
    cells_data: impl Into<Option<Vec<C>>>,
    global_state: impl Into<Option<G>>,
    randomize: impl Into<Option<bool>>,
) -> IterationState<C, G> {
    let initial_cells: Vec<C> = match randomize.into().unwrap_or(false) {
        true => cells_data
            .into()
            .unwrap_or_default()
            .iter()
            .map(|cell| cell.randomize())
            .collect(),
        false => cells_data.into().unwrap_or_default(),
    };
    let initial_network: Vec<Vec<CellIndex>> = get_network_map::<C>(&initial_cells);
    let initial_global_state = global_state.into().unwrap_or_default();
    let (cell_updates, global_updates) = cells::run::run_processes::<C, G>(
        &initial_cells,
        &initial_network,
        &cell_setup_processes.unwrap_or_default(),
        &initial_global_state,
    );
    let updated_cells = apply_cell_updates::<C>(initial_cells, cell_updates);
    let updated_global_state = apply_global_updates::<G>(initial_global_state, global_updates);

    // TODO: Run setup processes
    let final_network: Vec<Vec<CellIndex>> = get_network_map::<C>(&updated_cells);

    let update_global_actions = global::run::run_processes::<C, G>(
        &updated_cells.iter().collect(),
        &final_network,
        &global_setup_processes.unwrap_or_default(),
        &updated_global_state,
    );
    let updated_global_state = apply_global_updates(updated_global_state, update_global_actions);

    IterationState {
        cells: updated_cells,
        global_state: updated_global_state,
        network: final_network,
    }
}
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
    let mut updated_global_state = current_state.global_state;
    if update_per_process {
        // For process in cell_processes run
        for process in cell_processes.iter() {
            let (cell_updates, global_updates) = cells::run::run_process_on_cells::<C, G>(
                &updated_cells,
                &network,
                process,
                &updated_global_state,
            );
            updated_cells = apply_cell_updates::<C>(updated_cells, cell_updates);
            updated_global_state = apply_global_updates::<G>(updated_global_state, global_updates)
        }
    } else {
        let (cell_updates, global_updates) = cells::run::run_processes::<C, G>(
            &updated_cells,
            &network,
            &cell_processes.iter().collect(),
            &updated_global_state,
        );
        updated_cells = apply_cell_updates::<C>(updated_cells, cell_updates);
        updated_global_state = apply_global_updates::<G>(updated_global_state, global_updates)
    }

    if update_per_process {
        // For process in cell_processes run
        for process in global_processes.iter() {
            let update_global_actions = global::run::run_processes::<C, G>(
                &updated_cells.iter().collect(),
                &network,
                &vec![process],
                &updated_global_state,
            );
            updated_global_state =
                apply_global_updates(updated_global_state, update_global_actions);
        }
    } else {
        let update_global_actions = global::run::run_processes::<C, G>(
            &updated_cells.iter().collect(),
            &network,
            &global_processes.iter().collect(),
            &updated_global_state,
        );
        updated_global_state = apply_global_updates(updated_global_state, update_global_actions);
    }

    // Update state
    current_state.global_state = updated_global_state;
    current_state.cells = updated_cells;
    current_state.network = network;
    current_state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::examples::example_processes::*;
    use crate::process_runner::examples::example_state::*;
    use crate::process_runner::network::*;

    fn get_demo_data() -> (Vec<CellState>, GlobalState, CellNetwork) {
        let example_cell_data = (0..10)
            .map(|i| CellState {
                id: CellIndex(i),
                population: 100,
                ..Default::default()
            })
            .collect();
        let example_global_state = GlobalState::default();
        let example_network = vec![vec![]];
        (example_cell_data, example_global_state, example_network)
    }
    mod test_run_iteration {
        use super::*;
        fn get_demo_processes() -> (Vec<CellProcessT>, Vec<GlobalProcessT>) {
            // TODO: Add example cell setup processes
            let example_cell_processes = vec![CellProcessT {
                id: 0,
                func: Box::new(increase_population_by_10_percent),
            }];
            // TODO: Add example global setup processes
            let example_global_processes = vec![GlobalProcessT {
                id: 0,
                func: Box::new(example_global_process),
            }];
            (example_cell_processes, example_global_processes)
        }
        #[test]
        fn should_run_iteration() {
            let (cells, global_state, network) = get_demo_data();
            let (cell_processes, global_processes) = get_demo_processes();
            let initial_state = IterationState {
                cells,
                global_state,
                network,
            };
            let state_out = run_iteration(
                &cell_processes,
                &global_processes,
                initial_state.clone(),
                false,
            );
            assert_ne!(initial_state.cells, state_out.cells);
        }
    }

    mod test_setup_initial_state {
        use super::*;

        fn get_demo_processes() -> (Vec<CellProcessT>, Vec<GlobalProcessT>) {
            // TODO: Add example cell setup processes
            let example_cell_setup_processes = vec![];
            // TODO: Add example global setup processes
            let example_global_setup_processes = vec![];
            (example_cell_setup_processes, example_global_setup_processes)
        }
        #[test]
        fn should_return_initial_state() {
            let (cells, _global_state, _network) = get_demo_data();
            let (cell_setup_processes, global_setup_processes) = get_demo_processes();
            let initial_state = setup_initial_state(
                Some(cell_setup_processes.iter().collect()),
                Some(global_setup_processes.iter().collect()),
                cells.clone(),
                None,
                false,
            );
            assert_eq!(initial_state.cells.len(), cells.len());
        }

        #[test]
        fn should_allow_random_cell_state() {
            let (cells, _global_state, _network) = get_demo_data();
            let (cell_setup_processes, global_setup_processes) = get_demo_processes();
            let initial_state = setup_initial_state(
                Some(cell_setup_processes.iter().collect()),
                Some(global_setup_processes.iter().collect()),
                cells.clone(),
                None,
                true,
            );
            assert_eq!(initial_state.cells.len(), cells.len());
            assert_ne!(initial_state.cells, cells);
        }
    }
}
