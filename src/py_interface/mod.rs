/* Python interfaces

Each struct requires a wrapper that is readable by python.
For the wrapper we implement various methods that enable read and coonstruct
access from python.
*/
pub mod cell_state;
pub mod global_state;
pub mod run;
