mod run_model;
use run_model::example_process;
use run_model::run_iteration;
use run_model::CellState;
use run_model::GlobalData;
use run_model::IterationState;
use run_model::Process;

fn main() {
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
        cells: vec![
            CellState {
                population: 12,
                id: 0,
            },
            CellState {
                population: 40,
                id: 1,
            },
        ],
    };
    let processes = vec![Process {
        id: 0,
        func: Box::new(example_process),
    }];
    let final_state = run_iteration(processes, initial_state);

    println!("Cell 0 pop! {}", final_state.cells[0].population);
    println!("Cell 1 pop! {}", final_state.cells[1].population);
}
