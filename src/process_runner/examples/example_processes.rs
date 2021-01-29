use super::example_state::CellState;
use super::example_state::GlobalState;
use crate::process_runner::cells::run::CellUpdate;
use crate::process_runner::cells::run::Process as CellProcess;
use crate::process_runner::global::run::Process as GlobalProcess;

pub fn example_process(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
    _global_state: &GlobalState,
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
    _global_state: &GlobalState,
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

/// Example global process that just returns the global state
pub fn example_global_process(_cells: &Vec<&CellState>, global_state: GlobalState) -> GlobalState {
    global_state
}

/// Example global process that just returns the global state
pub fn example_global_process_iter(
    _cells: &Vec<&CellState>,
    global_state: GlobalState,
) -> GlobalState {
    let mut new_global_state = global_state;
    new_global_state.iterations += 1;
    new_global_state
}

// Default example processes
#[allow(dead_code)]
pub fn default_cell_processes() -> Vec<CellProcess<CellState, GlobalState>> {
    vec![
        CellProcess::new(0, Box::new(example_process)),
        CellProcess::new(1, Box::new(population_migration)),
    ]
}

// Default example processes
#[allow(dead_code)]
pub fn default_global_processes() -> Vec<GlobalProcess<CellState, GlobalState>> {
    vec![
        GlobalProcess::new(0, Box::new(example_global_process)),
        GlobalProcess::new(0, Box::new(example_global_process_iter)),
    ]
}
