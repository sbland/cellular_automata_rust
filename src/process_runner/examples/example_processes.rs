use super::example_state::CellState;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;

pub fn example_process(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate<CellState>> {
    vec![CellUpdate {
        target_cell: cell_state.id,
        action: Box::new(|mut cell_state: CellState| -> CellState {
            cell_state.population += (cell_state.population as f64 / 10.0).floor() as u32;
            cell_state
        }),
    }]
}

pub fn population_migration(
    cell_state: &CellState,
    neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate<CellState>> {
    let mut movement = 0;
    for n in neighbours.iter() {
        movement += (n.population as f64 / 10.0).ceil() as u32;
    }
    vec![CellUpdate {
        target_cell: cell_state.id,
        // Note: We use the move keyword here to allow external variables to be captured by the closure
        action: Box::new(move |mut cell_state: CellState| -> CellState {
            cell_state.population += movement as u32;
            cell_state
        }),
    }]
}

// Default example processes
#[allow(dead_code)]
pub fn default_processes() -> Vec<Process<CellState>> {
    vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ]
}
