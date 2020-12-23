// #[macro_use]
// extern crate cpython;
extern crate pyo3;

// use cpython::{py_fn, py_module_initializer, PyDict, PyResult, Python, ToPyObject};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

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
struct Data {
    a: u32,
    b: u32,
}

#[pymethods]
impl Data {
    #[new]
    fn new(a: u32, b: u32) -> Self {
        Data { a, b }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_data(d: Data) -> PyResult<String> {
    Ok((d.a + d.b).to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_data_b(d: &PyAny) -> PyResult<String> {
    // let dt: Data = Data {
    //     a: d.get_item("a").unwrap().extract().unwrap(),
    //     b: d.get_item("b").unwrap().extract().unwrap(),
    // };
    let dt: Data = Data {
        a: d.get_item("a").unwrap().extract()?,
        b: d.get_item("b").unwrap().extract()?,
    };
    Ok((dt.a + dt.b).to_string())
}

#[pyfunction]
fn sum_as_string_data_c(d: &PyAny) -> PyResult<String> {
    let dt: Data = d.extract()?;
    Ok((dt.a + dt.b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn cellular_automata(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string_data, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string_data_b, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string_data_c, m)?)?;
    m.add_class::<Data>()?;
    Ok(())
}
