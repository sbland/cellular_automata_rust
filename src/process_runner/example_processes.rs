use crate::process_runner::example_state::CellState;
use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::process::Process;
use crate::process_runner::process::Value;

pub fn example_process<'a>(
    cell_state: &CellState,
    _neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate<'a>> {
    vec![CellUpdate {
        target_cell: cell_state.id,
        target_field: String::from("population"),
        value: Value::NumberI((cell_state.population / 10) as i32),
        action: Action::ADD,
    }]
}

pub fn population_migration<'a>(
    cell_state: &CellState,
    neighbours: &Vec<&CellState>,
) -> Vec<CellUpdate<'a>> {
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
pub fn default_processes<'a>() -> Vec<Process<'a, CellState>> {
    vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ]
}
