/* Global State Interface */
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::process_runner::state::GlobalState;

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
