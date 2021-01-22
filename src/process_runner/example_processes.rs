use crate::process_runner::example_state::CellState;
use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;
use crate::process_runner::process::Value;

pub fn example_process(cell_state: &CellState, _neighbours: &[&CellState]) -> Vec<CellUpdate> {
    vec![CellUpdate {
        target_cell: cell_state.id,
        target_field: String::from("population"),
        value: Value::NumberI((cell_state.population / 10) as i32),
        action: Action::ADD,
    }]
}

pub fn population_migration(cell_state: &CellState, neighbours: &[&CellState]) -> Vec<CellUpdate> {
    let mut movement = 0;
    for n in neighbours.iter() {
        movement += n.population / 10;
    }
    vec![CellUpdate {
        target_cell: cell_state.id,
        target_field: String::from("population"),
        value: Value::NumberI(movement as i32),
        action: Action::ADD,
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
