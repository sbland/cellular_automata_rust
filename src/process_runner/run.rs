use crate::process_runner::network::get_network_map;
use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;
use crate::process_runner::state::CellState;
use crate::process_runner::state::GlobalState;
use crate::process_runner::state::IterationState;

pub fn run_process(
    cell: &CellState,
    process: &Process,
    neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate> {
    // let i = process.id;
    // println!("Running process {} on cell {}", i, cell.id);
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
    network: &Vec<Vec<u32>>,
    processes: &Vec<Process>,
) -> Vec<CellUpdate> {
    let mut cell_updates: Vec<CellUpdate> = Vec::new();
    for cell in cells.iter() {
        let cell_id = cell.id as usize;
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .into_iter()
            .map(|id| &cells[*id as usize])
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
        let id = cell_action.target_cell as usize;
        match cell_action.action {
            Action::ADD => modified_cells[id].population += cell_action.value,
            Action::SET => modified_cells[id].population = cell_action.value,
        }
    }
    modified_cells
}

pub fn run_iteration(processes: &Vec<Process>, input_state: IterationState) -> IterationState {
    let mut new_state = input_state;
    let network: Vec<Vec<u32>> = get_network_map(&new_state.cells);
    let cell_updates = run_processes(&new_state.cells, &network, &processes);
    let updated_cells = run_cell_updates(new_state.cells, cell_updates);
    let updated_global_state = get_next_global_state(&new_state.global_state);

    // Update state
    new_state.global_state = updated_global_state;
    new_state.cells = updated_cells;
    new_state
}
