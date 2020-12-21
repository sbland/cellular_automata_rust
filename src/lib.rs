extern crate pyo3;
use pyo3::callback::IntoPyCallbackOutput;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyObjectProtocol;

mod process_runner;
use geo::point;
use process_runner::example_process;
use process_runner::population_migration;
use process_runner::run_iteration;
use process_runner::Process;

use process_runner::state::CellState;
use process_runner::state::GlobalData;
use process_runner::state::IterationState;

// impl IntoPyCallbackOutput<String> for CellState {
//     fn convert(self, py: Python) -> PyResult<String> {
//         Ok(format!("hello"))
//     }
// }

#[pyclass]
#[derive(Clone)]
pub struct CellStatePy {
    // #[pyo3(get)]
    pub inner: CellState,
}

#[pymethods]
impl CellStatePy {
    #[new]
    fn new(id: u32, pos: (f64, f64), population: u32) -> Self {
        CellStatePy {
            inner: CellState {
                id,
                position: point!(x: pos.0, y: pos.1),
                population,
            },
        }
    }
}

#[pyproto]
impl PyObjectProtocol for CellStatePy {
    fn __str__(&self) -> PyResult<&'static str> {
        Ok("CellStatePy")
    }

    fn __repr__<'a>(&'a self) -> PyResult<String> {
        Ok(format!("CellStateObj id: {}", self.inner.id))
    }

    fn __getattr__(&'a self, name: &str) -> PyResult<String> {
        let out: String = match name {
            "a" => format!("hello"),
            &_ => format!("World"),
        };
        Ok(out)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct GlobalDataPy {
    pub inner: GlobalData,
}

fn demo_run() -> Vec<CellState> {
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
    final_state.cells
}

#[pyfunction]
fn demo_run_py() -> PyResult<Vec<CellStatePy>> {
    let out = demo_run();
    let cellpy = out
        .iter()
        .map(|c| CellStatePy { inner: *c })
        .collect::<Vec<_>>();
    Ok(cellpy)
}

// fn run_iteration_i(
//     cell_data: &Vec<CellState>,
//     // global_data: GlobalData,
//     // network_map: Vec<Vec<u32>>,
// ) -> (Vec<CellState>, GlobalData, Vec<Vec<u32>>) {
//     let global_data = GlobalData { iterations: 0 };
//     let network_map = vec![vec![0]];
//     (cell_data.to_vec(), global_data, network_map)
// }

// #[pyfunction]
// fn run_iteration_i_py(
//     // cell: CellStatePy,
//     cell_data: Vec<CellStatePy>,
//     // global_data: GlobalData,
//     // network_map: Vec<Vec<u32>>,
//     // ) -> PyResult<Vec<CellStatePy>> {
// ) -> PyResult<(Vec<CellState>, GlobalData, Vec<Vec<u32>>)> {
//     Ok(cell_data)
//     // let cell_data: Vec<CellStatePy> = vec![];
//     // let cell_data_inner: Vec<CellState> = cell_data.iter().map(|c| c.inner).collect::<Vec<_>>();
//     // let out = run_iteration_i(&cell_data_inner);
//     // let out = run_iteration_i(cell_data, global_data, network_map);
//     // Ok(out)
// }

#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(demo_run_py, m)?)?;
    m.add("demo_run", wrap_pyfunction!(demo_run_py, m)?)?;
    // m.add("run_iteration", wrap_pyfunction!(run_iteration_i_py, m)?)?;
    m.add_class::<CellStatePy>()?;
    Ok(())
}
