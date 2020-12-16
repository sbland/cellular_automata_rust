// #[macro_use]
extern crate cpython;
use cpython::{py_fn, py_module_initializer, PyDict, PyResult, Python, ToPyObject};

mod process_runner;
use geo::point;
use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::Process;

use process_runner::state::CellState;
use process_runner::state::GlobalData;
use process_runner::state::IterationState;

impl ToPyObject for CellState {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "id", self.id).unwrap();
        dict.set_item(py, "position", (self.position.x(), self.position.y()))
            .unwrap();
        dict.set_item(py, "population", self.population).unwrap();

        dict
    }
}

fn demo_run() -> String {
    let cells = (0..99)
        .map(|i| CellState {
            id: i,
            position: point!(x: 0.0, y: i as f64/100.0),
            population: 5,
        })
        .collect::<Vec<_>>();
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
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
    format!("Cell 0 population is {}", final_state.cells[0].population).to_string()
}

fn run_iteration_i(// cell_data: Vec<CellState>,
    // global_data: GlobalData,
    // network_map: Vec<Vec<u32>>,
) -> Vec<CellState>
// (
//     Vec<CellState>,
//     // GlobalData,
//     // Vec<Vec<u32>>
// )
{
    vec![CellState {
        id: 0,
        position: point!(x: 0.0, y: 0.0),
        population: 10,
    }]
    // (
    // cell_data,
    //     global_data,
    //     network_map,
    // )
}

fn run_iteration_i_py(
    _: Python,
    // cell_data: Vec<CellState>,
    // global_data: GlobalData,
    // network_map: Vec<Vec<u32>>,
) -> PyResult<
    Vec<CellState>, //     (
                    //     Vec<CellState>,
                    //     // GlobalData,
                    //     // Vec<Vec<u32>>
                    // )
> {
    let out = run_iteration_i();
    // let out = run_iteration_i(cell_data, global_data, network_map);
    Ok(out)
}

fn demo_run_py(_: Python) -> PyResult<String> {
    let out = demo_run();
    Ok(out)
}

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(cellular_automata, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "sum_as_string",
        py_fn!(py, sum_as_string_py(a: i64, b: i64)),
    )?;
    m.add(py, "demo_run", py_fn!(py, demo_run_py()))?;
    m.add(py, "run_iteration", py_fn!(py, run_iteration_i_py()))?;
    Ok(())
});

// logic implemented as a normal rust function
fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b).to_string()
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn sum_as_string_py(_: Python, a: i64, b: i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}
