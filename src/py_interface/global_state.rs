/* Global State Interface */
extern crate pyo3;

use pyo3::prelude::*;

use crate::process_runner::state::GlobalState;

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

impl GlobalStatePy {
    pub fn inner(input_state: GlobalState) -> Self {
        GlobalStatePy { inner: input_state }
    }
}
