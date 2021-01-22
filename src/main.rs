/*
// TODO: Implement Gui Interface
// TODO: Build
*/

mod process_runner;
use geo::point;
use process_runner::example_processes::example_process;
use process_runner::example_processes::population_migration;
use process_runner::process::Process;
use process_runner::run::run_iteration;

use process_runner::example_state::CellState;
use process_runner::state::GlobalState;
use process_runner::state::IterationState;

fn main() {
    let cells = (0..99)
        .map(|i| {
            CellState::new(
                i,
                point!(x: 0.0, y: i as f64/100.0),
                5,
                None,
                None,
                None,
                None,
            )
        })
        .collect::<Vec<_>>();
    let initial_state = IterationState {
        global_state: GlobalState { iterations: 0 },
        cells: cells,
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
    let final_state = run_iteration(&processes, initial_state);

    println!("Cell 0 pop! {}", final_state.cells[0].population);
    println!("Cell 1 pop! {}", final_state.cells[1].population);
}
