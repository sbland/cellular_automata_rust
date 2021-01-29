pub mod examples;
pub mod network;
pub mod process;
pub mod run;
pub mod state;

/* =============== TESTS =============== */

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    use process::CellUpdate;
    use process::Process;

    use examples::example_processes::example_process;
    use examples::example_processes::population_migration;
    use examples::example_state::CellState;
    use run::apply_cell_updates;
    use run::run_iteration;
    use run::run_processes;
    use state::CellIndex;
    use state::GlobalState;
    use state::IterationState;

    // use network::get_network_map;

    fn get_demo_cells() -> Vec<CellState> {
        vec![
            CellState::new(0, point!(x:5.54, y:-0.19), 12),
            CellState::new(1, point!(x:5.77, y:-0.02), 40),
            CellState::new(2, point!(x:99.99, y:-0.42), 40),
        ]
    }

    macro_rules! action_add_population {
        ($amount: literal) => {{
            fn add_population(mut cell_state: CellState) -> CellState {
                cell_state.population += $amount;
                cell_state
            }
            add_population
        }};
    }

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

    fn get_demo_processes() -> Vec<Process<CellState>> {
        vec![
            Process {
                id: 0,
                func: Box::new(example_process),
            },
            Process {
                id: 1,
                func: Box::new(population_migration),
            },
        ]
    }

    fn get_demo_netork() -> Vec<Vec<CellIndex>> {
        vec![vec![CellIndex(1)], vec![CellIndex(0)], vec![]]
    }

    impl PartialEq for CellUpdate<CellState> {
        // TODO: Improve CellUpdate Comparison for test
        fn eq(&self, other: &Self) -> bool {
            self.target_cell == other.target_cell
        }
    }

    #[test]
    fn test_run_processes() {
        let cells_in = get_demo_cells();
        let processes = get_demo_processes();
        let network = get_demo_netork();
        let updates = run_processes(&cells_in, &network, &processes);
        let expected_updates = get_demo_updates();
        assert_eq!(updates.len(), expected_updates.len());
        assert_eq!(updates, expected_updates);
    }

    #[test]
    fn test_run_cell_updates_add() {
        let cells_in = get_demo_cells();
        let updates = vec![CellUpdate::<CellState> {
            target_cell: CellIndex(0),
            action: Box::new(action_add_population!(99)),
        }];
        let updated_cells = apply_cell_updates(cells_in, updates);
        assert_eq!(updated_cells[0].population, 111);
    }

    #[test]
    fn test_apply_cell_updates() {
        let cells_in = get_demo_cells();
        let updates = get_demo_updates();
        let updated_cells = apply_cell_updates(cells_in, updates);
        assert_eq!(updated_cells[0].population, 21);
    }

    #[test]
    fn test_run_iteration() {
        let initial_state = IterationState {
            global_state: GlobalState { iterations: 0 },
            cells: get_demo_cells(),
            network: vec![vec![]], // Note network calculated internally
        };
        let processes = get_demo_processes();
        let final_state = run_iteration(&processes, initial_state);
        assert_eq!(final_state.cells.len(), 3);
        assert_eq!(final_state.cells[0].population, 17); // initially 12
        assert_eq!(final_state.cells[1].population, 46); // initially 40
        assert_eq!(final_state.cells[2].population, 44); // initially 40
    }
}
