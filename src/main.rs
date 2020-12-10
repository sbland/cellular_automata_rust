struct GlobalData {
    iterations: u32,
}

struct CellState {
    population: u32,
}

struct IterationState {
    global_data: GlobalData,
    cells: Vec<CellState>,
}

struct CellUpdate {
    action: u32, // TODO: replace with action type
}

struct Process {
    id: u32,
    pub func: Box<dyn Fn(&CellState) -> CellUpdate>,
}

fn run_iteration(processes: Vec<Process>, input_state: IterationState) -> IterationState {
    let mut new_state = input_state;
    let new_global_state = GlobalData {
        iterations: new_state.global_data.iterations + 1,
    };
    let mut cell_updates: Vec<CellUpdate> = Vec::new();

    // Run for all cells
    for process in processes.iter() {
        let i = process.id;
        println!("Running process {}", i);
        let func = &process.func;
        let cell = &new_state.cells[0];
        let cell_update: CellUpdate = func(cell);
        cell_updates.push(cell_update);
    }
    new_state.cells[0].population += cell_updates[0].action;

    new_state.global_data = new_global_state;
    new_state
}

fn example_process(cell_state: &CellState) -> CellUpdate {
    CellUpdate {
        action: cell_state.population,
    }
}

fn main() {
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
        cells: vec![CellState { population: 12 }, CellState { population: 40 }],
    };
    let processes = vec![Process {
        id: 0,
        func: Box::new(example_process),
    }];
    let final_state = run_iteration(processes, initial_state);

    println!("Hello, world! {}", final_state.cells[0].population);
    // let process = &processes[0].func;
    // let cell = &initial_state.cells[0];
    // let cell_update = process(cell);
    // println!("hello {}", cell_update.action)
}
