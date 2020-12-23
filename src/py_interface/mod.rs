/* Python interfaces

Each struct requires a wrapper that is readable by python.
For the wrapper we implement various methods that enable read and coonstruct
access from python.
*/
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::process_runner::state::CellState;
use crate::process_runner::state::GlobalState;
// use crate::process_runner::state::IterationState;

use geo::point;

#[pyclass]
#[derive(Clone)]
pub struct CellStatePy {
    pub inner: CellState,
}

#[pymethods]
impl CellStatePy {
    #[new]
    pub fn new(id: u32, pos: (f64, f64), population: u32) -> Self {
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
            "population" => format!("{}", self.inner.population),
            // TODO: Should return missing attribute error here
            &_ => format!("World"),
        };
        Ok(out)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct GlobalStatePy {
    pub inner: GlobalState,
}

#[pymethods]
impl GlobalStatePy {
    #[new]
    pub fn new() -> Self {
        GlobalStatePy {
            inner: GlobalState { iterations: 0 },
        }
    }
}
