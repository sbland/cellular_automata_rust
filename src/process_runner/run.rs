use crate::process_runner::network::get_network_map;
use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;
use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellState;
use crate::process_runner::state::GlobalState;
use crate::process_runner::state::IterationState;

pub fn run_process(
    cell: &CellState,
    process: &Process,
    neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate> {
    let func = &process.func;
    let cell_updates: Vec<CellUpdate> = func(&cell, &neighbours);
    cell_updates
}

pub fn get_next_global_state(global_state: &GlobalState) -> GlobalState {
    let new_global_state = GlobalState {
        iterations: global_state.iterations + 1,
    };
    new_global_state
}

pub fn run_processes(
    cells: &Vec<CellState>,
    network: &Vec<Vec<CellIndex>>,
    processes: &Vec<Process>,
) -> Vec<CellUpdate> {
    let mut cell_updates: Vec<CellUpdate> = Vec::new();
    for cell in cells.iter() {
        let cell_id = cell.id.0 as usize;
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .into_iter()
            // Note we use tuple struct destructuring here
            .map(|CellIndex(id)| &cells[*id as usize])
            .collect::<Vec<_>>();
        for process in processes.iter() {
            let mut cell_update = run_process(&cell, &process, &neighbours);
            cell_updates.append(&mut cell_update);
        }
    }
    cell_updates
}

pub fn run_cell_updates(cells_in: Vec<CellState>, cell_updates: Vec<CellUpdate>) -> Vec<CellState> {
    let mut modified_cells = cells_in;
    for cell_action in cell_updates.iter() {
        let id = cell_action.target_cell.0 as usize;
        match cell_action.action {
            // TODO: implement for all cell fields
            // TODO: Implement default case
            Action::ADD => match cell_action.target_field.as_str() {
                "population" => modified_cells[id].population += cell_action.value,
                &_ => (),
            },
            Action::SET => match cell_action.target_field.as_str() {
                "population" => modified_cells[id].population = cell_action.value.to::<u32>(),
                &_ => (),
            },
        }
    }
    modified_cells
}

pub fn run_iteration(processes: &Vec<Process>, input_state: IterationState) -> IterationState {
    let mut new_state = input_state;
    let network: Vec<Vec<CellIndex>> = get_network_map(&new_state.cells);
    let cell_updates = run_processes(&new_state.cells, &network, &processes);
    let updated_cells = run_cell_updates(new_state.cells, cell_updates);
    let updated_global_state = get_next_global_state(&new_state.global_state);

    // Update state
    new_state.global_state = updated_global_state;
    new_state.cells = updated_cells;
    new_state
}
