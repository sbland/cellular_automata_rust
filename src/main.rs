mod process_runner;
use geo::point;
use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::Process;

use process_runner::state::CellState;
use process_runner::state::GlobalData;
use process_runner::state::IterationState;

fn main() {
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
        cells: vec![
            CellState::new(0, point!(x:0, y:0), 12),
            CellState::new(1, point!(x:1, y:0), 40),
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

    println!("Cell 0 pop! {}", final_state.cells[0].population);
    println!("Cell 1 pop! {}", final_state.cells[1].population);
}
