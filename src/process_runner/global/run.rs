use super::state::GlobalStateBase;
use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::cells::state::CellStateBase;

// A function that takes the cells and global state and returns an updated global state

/// A boxed function to modify the global state
pub type GlobalUpdateFn<G> = Box<dyn Fn(G) -> G>;

pub struct GlobalUpdate<T: GlobalStateBase> {
    pub id: String,
    pub action: GlobalUpdateFn<T>,
}
type ProcessFuncT<C, G> = Box<dyn Fn(&Vec<&C>, &G) -> Vec<GlobalUpdate<G>>>;

impl<T: GlobalStateBase> std::fmt::Debug for GlobalUpdate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalUpdate")
            .field("id", &self.id.to_owned())
            .finish()
    }
}

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

/// Apply all queued global updates to the global state
pub fn apply_global_updates<T: GlobalStateBase + Clone>(
    global_state_in: T,
    global_updates: Vec<GlobalUpdate<T>>,
) -> T {
    let mut modified_global_state = global_state_in;
    for global_action in global_updates.iter() {
        modified_global_state = (global_action.action)(modified_global_state.clone())
    }
    modified_global_state
}

/// Run all processes sequentially on global state
pub fn run_processes<C: CellStateBase, G: GlobalStateBase>(
    cells: &Vec<&C>,
    _network: &Vec<Vec<CellIndex>>,
    processes: &Vec<&Process<C, G>>,
    global_state: &G,
) -> Vec<GlobalUpdate<G>> {
    processes
        .iter()
        .fold(Vec::new(), move |mut updates, process| {
            let mut new_updates = (process.func)(cells, global_state);
            updates.append(&mut new_updates);
            updates
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::examples::example_processes::*;
    use crate::process_runner::examples::example_state::*;
    use geo::point;

    macro_rules! action_add_population_global {
        ($amount: literal) => {{
            fn add_population(mut global_state: GlobalState) -> GlobalState {
                global_state.population += $amount;
                global_state
            }
            add_population
        }};
    }

    // fn demo_process_fn(
    //     cell_state: &CellState,
    //     _neighbours: &Vec<&CellState>,
    //     global_state: &GlobalState,
    // ) -> (Vec<CellUpdate<CellState>>, Vec<GlobalUpdate<GlobalState>>) {
    //     let a = global_state.population;
    //     (
    //         vec![CellUpdate {
    //             target_cell: cell_state.id,
    //             action: Box::new(move |mut cell_state_loc: CellState| -> CellState {
    //                 cell_state_loc.population += a;
    //                 cell_state_loc
    //             }),
    //         }],
    //         vec![GlobalUpdate {
    //             id: format!("Global act for {}", cell_state.id),
    //             action: Box::new(|mut global_state_loc: GlobalState| -> GlobalState {
    //                 global_state_loc.population += 1;
    //                 global_state_loc
    //             }),
    //         }],
    //     )
    // }

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

    fn demo_processes() -> Vec<Process<CellState, GlobalState>> {
        let p1 = Process::new(0, Box::new(example_global_process));
        let p2 = Process::new(1, Box::new(example_global_process_iter));
        vec![p1, p2]
    }

    fn demo_global_updates() -> Vec<GlobalUpdate<GlobalState>> {
        vec![
            GlobalUpdate::<GlobalState> {
                id: "Example global process".to_owned(),
                action: Box::new(action_add_population_global!(11)),
            },
            GlobalUpdate::<GlobalState> {
                id: "Example global process iter".to_owned(),
                action: Box::new(action_add_population_global!(11)),
            },
        ]
    }

    mod test_run_processes {
        use super::*;

        fn run_demo_processes() -> Vec<GlobalUpdate<GlobalState>> {
            let cells = demo_cells();
            let network = demo_network(cells.iter().collect());
            let processes: Vec<Process<CellState, GlobalState>> = demo_processes();
            let global_state: GlobalState = GlobalState::new(0);
            run_processes::<CellState, GlobalState>(
                &cells.iter().collect(),
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
        fn can_get_global_state_updates_from_process() {
            let updates = run_demo_processes();
            let example_updates = demo_global_updates();
            assert_eq!(updates.len(), example_updates.len());
            assert_eq!(updates, example_updates);
        }
    }

    mod test_apply_global_updates {
        use super::*;
        #[test]
        fn should_apply_population_addition_update_to_global_and_increase_population() {
            let global_data = GlobalState::new(0);
            let updates = vec![GlobalUpdate::<GlobalState> {
                id: "Global act for 0".to_owned(),
                action: Box::new(action_add_population_global!(11)),
            }];
            let updated_state = apply_global_updates(global_data, updates);
            assert_eq!(updated_state.population, 11);
        }
        #[test]
        fn should_get_example_updates_and_apply_to_globalstate_changing_population() {
            let global_data = GlobalState::new(0);
            let updates = demo_global_updates();
            let updated_state = apply_global_updates(global_data, updates);
            assert_eq!(updated_state.population, 22);
        }
    }
}
