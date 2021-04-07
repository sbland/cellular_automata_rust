/* Global State Interface */
extern crate pyo3;

use crate::process_runner::global::state::GlobalStateBase;
use crate::py_interface::PyWrapperBase;

/// Generic Type GlobalStatePy base
///
/// Generic types that inherit this should identify the inner type
/// that must inherit GlobalStateBase
///
/// It should also implement new as a pymethod
/// and the PyObjectProtocol
pub trait GlobalStatePyBase<T: GlobalStateBase>: PyWrapperBase<T> + Default {}
