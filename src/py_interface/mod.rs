/* Python interfaces

Each struct requires a wrapper that is readable by python.
For the wrapper we implement various methods that enable read and coonstruct
access from python.
*/
pub mod cell_state;
pub mod example_run;
pub mod examples;
pub mod global_state;
pub mod run;

/// Generic Type wrapper base
///
/// Generic types that inherit this should identify the inner type
///
/// It should also implement new as a pymethod
/// and the PyObjectProtocol
pub trait PyWrapperBase<T> {
    fn get_inner(&self) -> T;
    fn from_inner(inner: &T) -> Self;
}
