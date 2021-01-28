/// Cell Process Module
///
/// 'a lifetime represents a single iteration
/// Generic C represents the cell type
///
use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellStateBase;

/// A function that takes a CellState, makes a modification and returns the modified CellState
type CellActionFunc<T> = Box<dyn Fn(T) -> T>;

pub struct CellUpdate<T: CellStateBase> {
    pub action: CellActionFunc<T>,
    pub target_cell: CellIndex,
}

impl<T: CellStateBase> std::fmt::Debug for CellUpdate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellUpdate")
            // .field("action", &*stringify!(&self.action).to_owned())
            .finish()
    }
}

#[allow(unused)]
impl<T: CellStateBase> CellUpdate<T> {
    pub fn new(target_cell: CellIndex, action: CellActionFunc<T>) -> CellUpdate<T> {
        CellUpdate::<T> {
            action,
            target_cell,
        }
    }
}

// A function that takes a cell and its neighbours and returns a CellUpdate instance
type ProcessFuncT<T> = Box<dyn Fn(&T, &Vec<&T>) -> Vec<CellUpdate<T>>>;

pub struct Process<T: CellStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<T>,
}

#[allow(dead_code)]
impl<T: CellStateBase> Process<T> {
    pub fn new(id: u32, func: ProcessFuncT<T>) -> Process<T> {
        Process { id, func }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement tests
}
