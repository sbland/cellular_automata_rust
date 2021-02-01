/// Cell Process Module
///
/// 'a lifetime represents a single iteration
/// Generic C represents the cell type
///
use super::state::CellIndex;
use super::state::CellStateBase;
use crate::process_runner::global::state::GlobalStateBase;

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
type ProcessFuncT<C, G> = Box<dyn Fn(&C, &Vec<&C>, &G) -> Vec<CellUpdate<C>>>;

pub struct Process<C: CellStateBase, G: GlobalStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<C, G>,
}

impl<C: CellStateBase, G: GlobalStateBase> std::fmt::Debug for Process<C, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process")
            .field("id", &self.id)
            .finish()
    }
}


#[allow(dead_code)]
impl<C: CellStateBase, G: GlobalStateBase> Process<C, G> {
    pub fn new(id: u32, func: ProcessFuncT<C, G>) -> Process<C, G> {
        Process { id, func }
    }
}

/// Apply all queued cell updates to the cells
pub fn apply_cell_updates<T: CellStateBase + Clone>(
    cells_in: Vec<T>,
    cell_updates: Vec<CellUpdate<T>>,
) -> Vec<T> {
    let mut modified_cells = cells_in;
    for cell_action in cell_updates.iter() {
        let id = cell_action.target_cell.0 as usize;
        modified_cells[id] = (cell_action.action)(modified_cells[id].clone())
    }
    modified_cells
}

/// Run a single process on a single cell
pub fn run_process<C: CellStateBase, G: GlobalStateBase>(
    cell: &C,
    process: &Process<C, G>,
    neighbours: &Vec<&C>, // A list of the neighbours states
    global_state: &G,
) -> Vec<CellUpdate<C>> {
    let func = &process.func;
    let cell_updates: Vec<CellUpdate<C>> = func(&cell, &neighbours, &global_state);
    cell_updates
}

/// Run all processes on all cells
pub fn run_processes<C: CellStateBase, G: GlobalStateBase>(
    cells: &Vec<C>,
    network: &Vec<Vec<CellIndex>>,
    processes: &Vec<Process<C, G>>,
    global_state: &G,
) -> Vec<CellUpdate<C>> {
    let mut cell_updates: Vec<CellUpdate<C>> = Vec::new();
    for cell in cells.iter() {
        let cell_id: usize = cell.id().into();
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .iter()
            // Note we use tuple struct destructuring here
            .map(|CellIndex(id)| &cells[*id as usize])
            .collect::<Vec<_>>();
        for process in processes.iter() {
            let mut more_cell_updates =
                run_process::<C, G>(&cell, &process, &neighbours, &global_state);
            cell_updates.append(&mut more_cell_updates);
        }
    }
    cell_updates
}

#[cfg(test)]
mod tests {
    // TODO: Implement tests
}
