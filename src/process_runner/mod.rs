pub mod agents;
pub mod cells;
pub mod examples;
pub mod global;
pub mod network;
pub mod run;
pub mod state;
/* =============== TESTS =============== */

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    use crate::process_runner::cells::run::Process as CellProcess;
    use cells::run::apply_cell_updates;
    use cells::run::run_processes;
    use cells::run::CellUpdate;
    use cells::state::CellIndex;
    use examples::example_processes::default_cell_processes;
    use examples::example_processes::default_global_processes;
    use examples::example_processes::CellProcessT;
    use examples::example_processes::GlobalProcessT;
    use examples::example_state::CellState;
    use examples::example_state::GlobalState;
    use network::CellNetwork;
    use run::run_iteration;
    use state::IterationState;

    fn get_demo_cells() -> Vec<CellState> {
        vec![
            CellState::new(0, point!(x:5.54, y:-0.19), 12),
            CellState::new(1, point!(x:5.77, y:-0.02), 40),
            CellState::new(2, point!(x:99.99, y:-0.42), 40),
        ]
    }

    // helper to create a add population action
    macro_rules! action_add_population {
        ($amount: literal) => {{
            fn add_population(mut cell_state: CellState) -> CellState {
                cell_state.population += $amount;
                cell_state
            }
            add_population
        }};
    }

    // helper to get some demo updates
    fn get_demo_updates() -> Vec<CellUpdate<CellState>> {
        vec![
            CellUpdate::<CellState> {
                target_cell: CellIndex(0),
                action: Box::new(action_add_population!(1)),
            },
            CellUpdate::<CellState> {
                target_cell: CellIndex(0),
                action: Box::new(action_add_population!(8)),
            },
            CellUpdate::<CellState> {
                target_cell: CellIndex(1),
                action: Box::new(action_add_population!(4)),
            },
            CellUpdate::<CellState> {
                target_cell: CellIndex(1),
                action: Box::new(action_add_population!(1)),
            },
            CellUpdate::<CellState> {
                target_cell: CellIndex(2),
                action: Box::new(action_add_population!(4)),
            },
            CellUpdate::<CellState> {
                target_cell: CellIndex(2),
                action: Box::new(action_add_population!(1)),
            },
        ]
    }

    fn get_demo_network() -> Vec<Vec<CellIndex>> {
        vec![vec![CellIndex(1)], vec![CellIndex(0)], vec![]]
    }

    impl PartialEq for CellUpdate<CellState> {
        // TODO: Improve CellUpdate Comparison for test
        fn eq(&self, other: &Self) -> bool {
            self.target_cell == other.target_cell
        }
    }

    mod test_run_cell_process {
        use super::*;
        fn test_setup() -> (Vec<CellState>, Vec<CellProcessT>, CellNetwork, GlobalState) {
            let cells_in = get_demo_cells();
            let cell_processes = default_cell_processes();
            let network = get_demo_network();
            let global_state = GlobalState {
                iterations: 0,
                // ..Default::default()
            };
            (cells_in, cell_processes, network, global_state)
        }

        #[test]
        fn should_run_processes_and_get_a_vector_of_updates() {
            let (cells_in, cell_processes, network, global_state) = test_setup();
            let updates = run_processes(
                &cells_in,
                &network,
                &cell_processes.iter().collect(),
                &global_state,
            );
            let expected_updates = get_demo_updates();
            assert_eq!(updates.len(), expected_updates.len());
            assert_eq!(updates, expected_updates);
        }
    }

    mod test_apply_cell_updates {
        use super::*;
        #[test]
        fn should_apply_population_addition_update_to_cell_and_increase_population() {
            let cells_in = get_demo_cells();
            let updates = vec![CellUpdate::<CellState> {
                target_cell: CellIndex(0),
                action: Box::new(action_add_population!(99)),
            }];
            let updated_cells = apply_cell_updates(cells_in, updates);
            assert_eq!(updated_cells[0].population, 111);
        }
        #[test]
        fn should_get_example_updates_and_apply_to_cells_changing_population() {
            let cells_in = get_demo_cells();
            let updates = get_demo_updates();
            let updated_cells = apply_cell_updates(cells_in, updates);
            assert_eq!(updated_cells[0].population, 21);
        }
    }

    mod test_run_iteration {
        use super::*;

        fn test_setup() -> (
            IterationState<CellState, GlobalState>,
            Vec<CellProcessT>,
            Vec<GlobalProcessT>,
        ) {
            let initial_state = IterationState {
                global_state: GlobalState { iterations: 0 },
                cells: get_demo_cells(),
                network: vec![vec![]], // Note network calculated internally
            };
            let cell_processes = default_cell_processes();
            let global_processes = default_global_processes();
            (initial_state, cell_processes, global_processes)
        }
        #[test]
        fn should_run_a_model_iteration_and_update_the_state() {
            let (initial_state, cell_processes, global_processes) = test_setup();

            let final_state =
                run_iteration(&cell_processes, &global_processes, initial_state, false);
            assert_eq!(final_state.cells.len(), 3);
            assert_eq!(final_state.cells[0].population, 17); // initially 12
            assert_eq!(final_state.cells[1].population, 46); // initially 40
            assert_eq!(final_state.cells[2].population, 44); // initially 40
            assert_eq!(final_state.global_state.iterations, 1); // initially 0#
        }
    }

    mod test_run_iteration_series_proceses {
        use super::*;
        use crate::process_runner::examples::example_processes::*;
        #[test]
        /// When we run cell processes we run all the processes on the state of the cell
        /// at the start of the iteration. This can cause issues if a process is dependent
        /// or overrides a previous update.
        fn should_run_processes_in_series() {
            let mut initial_state = IterationState {
                global_state: GlobalState { iterations: 0 },
                cells: get_demo_cells(),
                network: vec![vec![]], // Note network calculated internally
            };

            initial_state.cells[0].population = 5;

            // Each of these example processes increases the population by 1
            // The final process checks if the population is greater than 5
            // else sets it back to 0.
            // Even though all the earlier processes increase the population greater
            // than 5 because it was lower at the start it is reset to 0.
            let cell_processes = vec![
                CellProcess::new(0, Box::new(set_population_to_100)),
                CellProcess::new(8, Box::new(conditional_pop_reset)),
            ];
            let global_processes = default_global_processes();
            let final_state =
                run_iteration(&cell_processes, &global_processes, initial_state, true);
            // When we run these
            assert_eq!(final_state.cells.len(), 3);
            assert_eq!(final_state.cells[0].population, 100); // initially 12
        }
    }
}
