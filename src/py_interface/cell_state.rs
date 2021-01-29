/* Python interfaces

Each struct requires a wrapper that is readable by python.
For the wrapper we implement various methods that enable read and coonstruct
access from python.
*/
extern crate pyo3;

use crate::process_runner::cells::state::CellStateBase;
use crate::py_interface::PyWrapperBase;

/// Generic Type CellStatePy base
///
/// Generic types that inherit this should identify the inner type
/// that must inherit CellStateBase
///
/// It should also implement new as a pymethod
/// and the PyObjectProtocol
pub trait CellStatePyBase<T: CellStateBase>: PyWrapperBase<T> {}
