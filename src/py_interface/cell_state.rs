/* Python interfaces

Each struct requires a wrapper that is readable by python.
For the wrapper we implement various methods that enable read and coonstruct
access from python.
*/
extern crate pyo3;

use geo::point;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellState;

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
                id: CellIndex(id),
                position: point!(x: pos.0, y: pos.1),
                population,
                ..Default::default()
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
        Ok(format!("CellStateObj id: {:?}", self.inner.id))
    }

    fn __getattr__(&'a self, name: &str) -> PyResult<String> {
        // TODO: Work out how to return value other than string here
        let out: String = match name {
            "id" => self.inner.id.into(),
            "position" => format!("{},{}", self.inner.position.x(), self.inner.position.y()),
            "population" => format!("{}", self.inner.population),
            // TODO: Should return missing attribute error here
            &_ => "INVALID FIELD".to_owned(),
        };
        Ok(out)
    }
}
