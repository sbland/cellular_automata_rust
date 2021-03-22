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
    use cells::run::CellUpdate;
    use examples::example_processes::default_cell_processes;
    use examples::example_processes::default_global_processes;
    use examples::example_processes::CellProcessT;
    use examples::example_processes::GlobalProcessT;
    use examples::example_state::CellState;
    use examples::example_state::GlobalState;
    use global::run::GlobalUpdate;
    use run::run_iteration;
    use state::IterationState;

    fn get_demo_cells() -> Vec<CellState> {
        vec![
            CellState::new(0, point!(x:5.54, y:-0.19), 12),
            CellState::new(1, point!(x:5.77, y:-0.02), 40),
            CellState::new(2, point!(x:99.99, y:-0.42), 40),
        ]
    }

    impl PartialEq for CellUpdate<CellState> {
        // TODO: Improve CellUpdate Comparison for test
        fn eq(&self, other: &Self) -> bool {
            self.target_cell == other.target_cell
        }
    }

    impl PartialEq for GlobalUpdate<GlobalState> {
        // TODO: Improve GlobalUpdate Comparison for test
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
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
                global_state: GlobalState {
                    iterations: 0,
                    population: 0,
                },
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
                global_state: GlobalState {
                    iterations: 0,
                    population: 0,
                },
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
