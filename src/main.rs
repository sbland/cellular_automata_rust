/*
// TODO: Implement Gui Interface
// TODO: Build
*/

mod process_runner;
use geo::point;
use process_runner::examples::example_processes::default_cell_processes;
use process_runner::examples::example_processes::default_global_processes;
use process_runner::run::run_iteration;

use process_runner::examples::example_state::CellState;
use process_runner::examples::example_state::GlobalState;
use process_runner::state::IterationState;

fn main() {
    let cells = (0..99)
        .map(|i| CellState::new(i, point!(x: 0.0, y: i as f64/100.0), 5))
        .collect::<Vec<_>>();
    let initial_state = IterationState {
        global_state: GlobalState { iterations: 0 },
        cells: cells,
        network: vec![vec![]],
    };
    let cell_processes = default_cell_processes();
    let global_processes = default_global_processes();
    let final_state = run_iteration(&cell_processes, &global_processes, initial_state, true);

    println!("Cell 0 pop! {}", final_state.cells[0].population);
    println!("Cell 1 pop! {}", final_state.cells[1].population);
}
