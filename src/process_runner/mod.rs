/*
// TODO: implement find closest cells for get neighbours
// TODO: Allow multiple cell actions from process
// TODO: Implement apply actions properly

*/

pub mod network;
pub mod state;

use state::CellState;
use state::GlobalData;
use state::IterationState;

use network::get_network_map;

pub enum Action {
    ADD,
}

pub struct CellUpdate {
    pub action: Action,
    pub target_cell: u32,
    pub value: u32, // could be int or float
}

pub struct Process {
    pub id: u32,
    pub func: Box<dyn Fn(&CellState, &Vec<&CellState>) -> CellUpdate>,
}

fn run_process(cell: &CellState, process: &Process, neighbours: &Vec<&CellState>) -> CellUpdate {
    let i = process.id;
    println!("Running process {} on cell {}", i, cell.id);
    let func = &process.func;
    let cell_update: CellUpdate = func(&cell, &neighbours);
    cell_update
}

pub fn get_next_global_state(global_state: &GlobalData) -> GlobalData {
    let new_global_state = GlobalData {
        iterations: global_state.iterations + 1,
    };
    new_global_state
}

fn run_processes(
    cells: &Vec<CellState>,
    network: Vec<Vec<u32>>,
    processes: Vec<Process>,
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
            let cell_update = run_process(&cell, &process, &neighbours);
            cell_updates.push(cell_update);
        }
    }
    cell_updates
}

fn run_cell_updates(cells_in: Vec<CellState>, cell_updates: Vec<CellUpdate>) -> Vec<CellState> {
    let mut modified_cells = cells_in;
    for cell_action in cell_updates.iter() {
        let id = cell_action.target_cell as usize;
        modified_cells[id].population += cell_action.value;
    }
    modified_cells
}

pub fn run_iteration(processes: Vec<Process>, input_state: IterationState) -> IterationState {
    let mut new_state = input_state;
    let network: Vec<Vec<u32>> = get_network_map(&new_state.cells);
    let cell_updates = run_processes(&new_state.cells, network, processes);
    let updated_cells = run_cell_updates(new_state.cells, cell_updates);
    let updated_global_data = get_next_global_state(&new_state.global_data);

    // Update state
    new_state.global_data = updated_global_data;
    new_state.cells = updated_cells;
    new_state
}

pub fn example_process(cell_state: &CellState, _neighbours: &Vec<&CellState>) -> CellUpdate {
    CellUpdate {
        target_cell: cell_state.id,
        value: cell_state.population / 10,
        action: Action::ADD,
    }
}

pub fn population_migration(cell_state: &CellState, neighbours: &Vec<&CellState>) -> CellUpdate {
    let mut movement = 0;
    for n in neighbours.iter() {
        movement += n.population / 10;
    }
    CellUpdate {
        target_cell: cell_state.id,
        value: movement,
        action: Action::ADD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    #[test]
    fn test_run_iteration() {
        let initial_state = IterationState {
            global_data: GlobalData { iterations: 0 },
            cells: vec![
                CellState::new(0, point!(x:5.54, y:-0.19), 12),
                CellState::new(1, point!(x:5.77, y:-0.02), 40),
            ],
        };
        let processes = vec![
            Process {
                id: 0,
                func: Box::new(example_process),
            },
            Process {
                id: 1,
                func: Box::new(population_migration),
            },
        ];
        let final_state = run_iteration(processes, initial_state);

        assert_eq!(final_state.cells[0].population, 17);
        assert_eq!(final_state.cells[1].population, 45);
    }
}
