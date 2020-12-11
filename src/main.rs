struct GlobalData {
    iterations: u32,
}

struct CellState {
    id: u32,
    population: u32,
}

struct IterationState {
    global_data: GlobalData,
    cells: Vec<CellState>,
}

enum Action {
    ADD,
}

struct CellUpdate {
    action: Action,
    target_cell: u32,
    value: u32, // could be int or float
}

struct Process {
    id: u32,
    pub func: Box<dyn Fn(&CellState) -> CellUpdate>,
}

fn example_process(cell_state: &CellState) -> CellUpdate {
    CellUpdate {
        target_cell: cell_state.id,
        value: cell_state.population / 10,
        action: Action::ADD,
    }
}

fn run_iteration(processes: Vec<Process>, input_state: IterationState) -> IterationState {
    let mut new_state = input_state;
    let new_global_state = GlobalData {
        iterations: new_state.global_data.iterations + 1,
    };
    let mut cell_updates: Vec<CellUpdate> = Vec::new();

    // Run each process on each cell
    for process in processes.iter() {
        for cell in new_state.cells.iter_mut() {
            let i = process.id;
            println!("Running process {} on cell {}", i, cell.id);
            let func = &process.func;
            let cell_update: CellUpdate = func(cell);
            cell_updates.push(cell_update);
        }
    }

    // Run the resulting actions
    for cell_action in cell_updates.iter() {
        let id = cell_action.target_cell as usize;
        new_state.cells[id].population += cell_action.value;
    }

    new_state.global_data = new_global_state;
    new_state
}

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
