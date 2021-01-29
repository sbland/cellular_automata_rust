use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::cells::state::CellStateBase;
use crate::process_runner::global::state::GlobalStateBase;

#[derive(Clone)]
pub struct IterationState<C: CellStateBase, G: GlobalStateBase> {
    // pub: iteration_counter: u32,
    pub global_state: G,
    pub cells: Vec<C>,
    pub network: Vec<Vec<CellIndex>>,
}
