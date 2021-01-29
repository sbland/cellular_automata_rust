// PYTHON INTERFACE EXAMPLE
use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::examples::example_state::CellState;
use crate::process_runner::examples::example_state::GlobalState;
use crate::py_interface::cell_state::CellStatePyBase;
use crate::py_interface::global_state::GlobalStatePyBase;
use crate::py_interface::PyWrapperBase;
use geo::point;

use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

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
impl CellStatePyBase<CellState> for CellStatePy {}
// Note: This now includes cloning so we could in theory end up with duplicate clones
impl PyWrapperBase<CellState> for CellStatePy {
    fn get_inner(&self) -> CellState {
        self.inner.clone()
    }

    fn from_inner(inner: &CellState) -> Self {
        CellStatePy {
            inner: inner.clone(),
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

// Global State

#[pyclass]
#[derive(Clone, Default)]
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

impl GlobalStatePy {
    pub fn inner(input_state: GlobalState) -> Self {
        GlobalStatePy { inner: input_state }
    }
}

#[pyproto]
impl PyObjectProtocol for GlobalStatePy {
    fn __str__(&self) -> PyResult<&'static str> {
        Ok("GlobalState")
    }

    fn __repr__<'a>(&'a self) -> PyResult<String> {
        Ok("GlobalStateObj".to_owned())
    }

    fn __getattr__(&'a self, name: &str) -> PyResult<String> {
        let out: String = match name {
            "iterations" => format!("{}", self.inner.iterations),
            // TODO: Should return missing attribute error here
            &_ => "INVALID".to_owned(),
        };
        Ok(out)
    }
}

impl PyWrapperBase<GlobalState> for GlobalStatePy {
    fn get_inner(&self) -> GlobalState {
        self.inner
    }

    fn from_inner(inner: &GlobalState) -> Self {
        GlobalStatePy { inner: *inner }
    }
}

impl GlobalStatePyBase<GlobalState> for GlobalStatePy {}
