extern crate pyo3;
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
            "population" => format!("{}", self.inner.population),
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

#[pyfunction]
fn run_iteration_py(
    // cell: CellStatePy,
    cell_data: Vec<CellStatePy>,
    // global_data: GlobalData,
    // network_map: Vec<Vec<u32>>,
    // ) -> PyResult<Vec<CellStatePy>> {
) -> PyResult<Vec<CellStatePy>> {
    // ) -> PyResult<(Vec<CellState>, GlobalData, Vec<Vec<u32>>)> {
    let cell_data_inner: Vec<CellState> = cell_data.iter().map(|c| c.inner).collect::<Vec<_>>();
    let initial_state = IterationState {
        global_data: GlobalData { iterations: 0 },
        cells: cell_data_inner,
    };
    let processes = vec![
        Process::new(0, Box::new(example_process)),
        Process::new(1, Box::new(population_migration)),
    ];
    let final_state = run_iteration(&processes, initial_state);
    let cell_data_outer: Vec<CellStatePy> = final_state
        .cells
        .iter()
        .map(|c| CellStatePy { inner: *c })
        .collect::<Vec<_>>();
    Ok(cell_data_outer)
}

#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("run_iteration", wrap_pyfunction!(run_iteration_py, m)?)?;
    m.add_class::<CellStatePy>()?;
    Ok(())
}
