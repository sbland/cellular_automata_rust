use super::state::GlobalStateBase;
use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::cells::state::CellStateBase;

// A function that takes the cells and global state and returns an updated global state
type ProcessFuncT<C, G> = Box<dyn Fn(&Vec<&C>, G) -> G>;

pub struct Process<C: CellStateBase, G: GlobalStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<C, G>,
}

#[allow(dead_code)]
impl<C: CellStateBase, G: GlobalStateBase> Process<C, G> {
    pub fn new(id: u32, func: ProcessFuncT<C, G>) -> Process<C, G> {
        Process { id, func }
    }
}

/// Run all processes sequentially on global state
pub fn run_processes<C: CellStateBase, G: GlobalStateBase>(
    cells: &Vec<&C>,
    _network: &Vec<Vec<CellIndex>>,
    processes: &Vec<Process<C, G>>,
    global_state: G,
) -> G {
    processes
        .iter()
        .fold(global_state, |current_global_state, process| {
            (process.func)(cells, current_global_state)
        })
}
