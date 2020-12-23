use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::state::CellState;

pub fn example_process(cell_state: &CellState, _neighbours: &Vec<&CellState>) -> Vec<CellUpdate> {
    vec![CellUpdate {
        target_cell: cell_state.id,
        value: cell_state.population / 10,
        action: Action::ADD,
    }]
}

pub fn population_migration(
    cell_state: &CellState,
    neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate> {
    let mut movement = 0;
    for n in neighbours.iter() {
        movement += n.population / 10;
    }
    vec![CellUpdate {
        target_cell: cell_state.id,
        value: movement,
        action: Action::ADD,
    }]
}
