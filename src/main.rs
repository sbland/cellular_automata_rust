/*
// TODO: Implement Gui Interface
// TODO: Build
*/

mod process_runner;
use geo::point;
use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::Process;

use process_runner::state::CellState;
use process_runner::state::GlobalState;
use process_runner::state::IterationState;

fn main() {
    let cells = (0..99)
        .map(|i| CellState {
            id: i,
            position: point!(x: 0.0, y: i as f64/100.0),
            population: 5,
        })
        .collect::<Vec<_>>();
    let initial_state = IterationState {
        global_state: GlobalState { iterations: 0 },
        cells: cells,
        // cells: vec![
        //     CellState::new(0, point!(x:5.54, y:-0.19), 12),
        //     CellState::new(1, point!(x:5.77, y:-0.02), 40),
        // ],
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
