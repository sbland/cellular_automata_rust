use super::example_state::CellState;
use super::example_state::GlobalState;
use crate::process_runner::cells::run::CellUpdate;
use crate::process_runner::cells::run::Process as CellProcess;
use crate::process_runner::global::run::GlobalUpdate;
use crate::process_runner::global::run::Process as GlobalProcess;

pub type CellProcessT = CellProcess<CellState, GlobalState>;
pub type GlobalProcessT = GlobalProcess<CellState, GlobalState>;

pub fn increase_population_by_10_percent(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
    _global_state: &GlobalState,
) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
    (
        vec![CellUpdate {
            target_cell: cell_state.id,
            action: Box::new(|mut cell_state: CellState| -> CellState {
                // cell_state.population = 10;
                cell_state.population += (cell_state.population as f64 / 10.0).floor() as u32;
                cell_state
            }),
        }],
        vec![],
    )
}

#[allow(dead_code)]
pub fn set_population_to_100(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
    _global_state: &GlobalState,
) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
    (
        vec![CellUpdate {
            target_cell: cell_state.id,
            action: Box::new(|mut cell_state: CellState| -> CellState {
                cell_state.population = 100;
                cell_state
            }),
        }],
        vec![],
    )
}

#[allow(dead_code)]
/// An example process that needs to be run in series
pub fn conditional_pop_reset(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
    _global_state: &GlobalState,
) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
    let new_population: u32 = if cell_state.population > 5 {
        cell_state.population
    } else {
        0
    };
    (
        vec![CellUpdate {
            target_cell: cell_state.id,
            action: Box::new(move |mut cell_state: CellState| -> CellState {
                cell_state.population = new_population;
                cell_state
            }),
        }],
        vec![],
    )
}

pub fn population_migration(
    cell_state: &CellState,
    neighbours: &Vec<&CellState>,
    _global_state: &GlobalState,
) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
    let mut movement = 0;
    for n in neighbours.iter() {
        movement += (n.population as f64 / 10.0).ceil() as u32;
    }
    (
        vec![CellUpdate {
            target_cell: cell_state.id,
            // Note: We use the move keyword here to allow external variables to be captured by the closure
            action: Box::new(move |mut cell_state: CellState| -> CellState {
                cell_state.population += movement as u32;
                cell_state
            }),
        }],
        vec![],
    )
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
pub fn default_cell_processes() -> Vec<CellProcessT> {
    vec![
        CellProcess::new(0, Box::new(increase_population_by_10_percent)),
        CellProcess::new(1, Box::new(population_migration)),
    ]
}

// Default example processes
#[allow(dead_code)]
pub fn default_global_processes() -> Vec<GlobalProcessT> {
    vec![
        GlobalProcess::new(0, Box::new(example_global_process)),
        GlobalProcess::new(0, Box::new(example_global_process_iter)),
    ]
}
