/// Cell Process Module
///
/// 'a lifetime represents a single iteration
/// Generic C represents the cell type
///
use super::state::CellIndex;
use super::state::CellStateBase;
use crate::process_runner::global::run::GlobalUpdate;
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
type ProcessFuncT<C, G> =
    Box<dyn Fn(&C, &Vec<&C>, &G) -> (Vec<CellUpdate<C>>, Vec<GlobalUpdate<G>>)>;

pub struct Process<C: CellStateBase, G: GlobalStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<C, G>,
}

impl<C: CellStateBase, G: GlobalStateBase> std::fmt::Debug for Process<C, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").field("id", &self.id).finish()
    }
}

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
) -> (Vec<CellUpdate<C>>, Vec<GlobalUpdate<G>>) {
    let func = &process.func;
    let updates = func(&cell, &neighbours, &global_state);
    let cell_updates: Vec<CellUpdate<C>> = updates.0;
    let global_updates: Vec<GlobalUpdate<G>> = updates.1;
    // TODO: implement global updates
    (cell_updates, global_updates)
}

/// Run all processes on all cells
pub fn run_processes<C: CellStateBase, G: GlobalStateBase>(
    cells: &Vec<C>,
    network: &Vec<Vec<CellIndex>>,
    processes: &Vec<&Process<C, G>>,
    global_state: &G,
) -> (Vec<CellUpdate<C>>, Vec<GlobalUpdate<G>>) {
    let mut cell_updates: Vec<CellUpdate<C>> = Vec::new();
    let mut global_updates: Vec<GlobalUpdate<G>> = Vec::new();
    for cell in cells.iter() {
        let cell_id: usize = cell.id().into();
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .iter()
            // Note we use tuple struct destructuring here
            .map(|CellIndex(id)| &cells[*id as usize])
            .collect::<Vec<_>>();
        for process in processes.iter() {
            let mut updates = run_process::<C, G>(&cell, &process, &neighbours, &global_state);
            cell_updates.append(&mut updates.0);
            global_updates.append(&mut updates.1);
        }
    }
    (cell_updates, global_updates)
}

/// Run all processes on all cells
pub fn run_process_on_cells<C: CellStateBase, G: GlobalStateBase>(
    cells: &Vec<C>,
    network: &Vec<Vec<CellIndex>>,
    process: &Process<C, G>,
    global_state: &G,
) -> (Vec<CellUpdate<C>>, Vec<GlobalUpdate<G>>) {
    let mut cell_updates: Vec<CellUpdate<C>> = Vec::new();
    let mut global_updates: Vec<GlobalUpdate<G>> = Vec::new();
    for cell in cells.iter() {
        let cell_id: usize = cell.id().into();
        let cell_network = &network[cell_id];
        let neighbours = cell_network
            .iter()
            // Note we use tuple struct destructuring here
            .map(|CellIndex(id)| &cells[*id as usize])
            .collect::<Vec<_>>();
        let mut updates = run_process::<C, G>(&cell, &process, &neighbours, &global_state);
        cell_updates.append(&mut updates.0);
        global_updates.append(&mut updates.1);
    }
    (cell_updates, global_updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::examples::example_state::*;
    use geo::point;

    fn demo_process_fn(
        cell_state: &CellState,
        _neighbours: &Vec<&CellState>,
        global_state: &GlobalState,
    ) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
        let a = global_state.iterations;
        (
            vec![CellUpdate {
                target_cell: cell_state.id,
                action: Box::new(move |mut cell_state_loc: CellState| -> CellState {
                    cell_state_loc.population += a;
                    cell_state_loc
                }),
            }],
            vec![GlobalUpdate {
                id: format!("Global act for {}", cell_state.id),
                action: Box::new(|mut global_state_loc: GlobalState| -> GlobalState {
                    global_state_loc.iterations += 1;
                    global_state_loc
                }),
            }],
        )
    }

    fn demo_cells() -> Vec<CellState> {
        vec![
            CellState::new(0, point!(x: 0.0, y: 0.0), 100),
            CellState::new(1, point!(x: 0.0, y: 0.0), 100),
            CellState::new(2, point!(x: 0.0, y: 0.0), 100),
        ]
    }

    fn demo_network(cells: Vec<&CellState>) -> Vec<Vec<CellIndex>> {
        vec![
            vec![cells[1].id, cells[2].id],
            vec![cells[0].id, cells[2].id],
            vec![cells[0].id, cells[1].id],
        ]
    }

    fn demo_neigbours(cells: Vec<&CellState>) -> Vec<&CellState> {
        vec![&cells[1], &cells[2]]
    }

    fn demo_processes() -> Vec<Process<CellState, GlobalState>> {
        let p1 = Process::new(0, Box::new(demo_process_fn));
        let p2 = Process::new(1, Box::new(demo_process_fn));
        vec![p1, p2]
    }

    mod test_run_process {
        use super::*;

        fn run_demo_process() -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
            let cells = demo_cells();
            let neighbours = demo_neigbours(cells.iter().collect());
            let processes = demo_processes();
            let global_state: GlobalState = GlobalState::new();
            run_process::<CellState, GlobalState>(
                &cells[0],
                &processes[0],
                &neighbours,
                &global_state,
            )
        }

        #[test]
        fn can_run_process() {
            run_demo_process();
        }
        #[test]
        fn can_get_cell_state_updates_from_process() {
            let updates = run_demo_process();
            assert_eq!(updates.0.len(), 1);
        }

        #[test]
        fn can_get_global_state_updates_from_process() {
            let updates = run_demo_process();
            assert_eq!(updates.1.len(), 1);
        }
    }

    mod test_run_processes {
        use super::*;
        fn run_demo_processes() -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
            let cells = demo_cells();
            let network = demo_network(cells.iter().collect());
            let processes: Vec<Process<CellState, GlobalState>> = demo_processes();
            let global_state: GlobalState = GlobalState::new();
            run_processes::<CellState, GlobalState>(
                &cells,
                &network,
                &processes.iter().collect(),
                &global_state,
            )
        }

        #[test]
        fn can_run_process() {
            run_demo_processes();
        }
        #[test]
        fn can_get_cell_state_updates_from_process() {
            let updates = run_demo_processes();
            assert_eq!(updates.0.len(), 6);
        }

        #[test]
        fn can_get_global_state_updates_from_process() {
            let updates = run_demo_processes();
            assert_eq!(updates.1.len(), 6);
        }
    }
}
