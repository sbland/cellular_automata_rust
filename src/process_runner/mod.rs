/*
TODO: Implement additional processses
*/

pub mod example_processes;
pub mod network;
pub mod process;
pub mod run;
pub mod state;

/* =============== TESTS =============== */

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    use process::Action;
    use process::CellUpdate;
    use process::Process;

    use example_processes::example_process;
    use example_processes::population_migration;
    use run::run_cell_updates;
    use run::run_iteration;
    use run::run_processes;
    use state::CellState;
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

    fn get_demo_updates() -> Vec<CellUpdate> {
        vec![
            CellUpdate {
                action: Action::ADD,
                target_cell: 0,
                value: 1,
            },
            CellUpdate {
                action: Action::ADD,
                target_cell: 0,
                value: 4,
            },
            CellUpdate {
                action: Action::ADD,
                target_cell: 1,
                value: 4,
            },
            CellUpdate {
                action: Action::ADD,
                target_cell: 1,
                value: 1,
            },
            CellUpdate {
                action: Action::ADD,
                target_cell: 2,
                value: 4,
            },
            CellUpdate {
                action: Action::ADD,
                target_cell: 2,
                value: 0,
            },
        ]
    }

    fn get_demo_processes() -> Vec<Process> {
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

    fn get_demo_netork() -> Vec<Vec<u32>> {
        vec![vec![1], vec![0], vec![]]
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
    fn test_run_cell_updates_set() {
        let cells_in = get_demo_cells();
        let updates = vec![CellUpdate {
            action: Action::SET,
            target_cell: 0,
            value: 99,
        }];
        let updated_cells = run_cell_updates(cells_in.clone(), updates.clone());
        assert_eq!(updated_cells[0].population, 99);
    }

    #[test]
    fn test_run_cell_updates_add() {
        let cells_in = get_demo_cells();
        let updates = vec![CellUpdate {
            action: Action::ADD,
            target_cell: 0,
            value: 99,
        }];
        let updated_cells = run_cell_updates(cells_in.clone(), updates.clone());
        assert_eq!(updated_cells[0].population, 111);
    }

    #[test]
    fn test_run_cell_updates() {
        let cells_in = get_demo_cells();
        let updates = get_demo_updates();
        let updated_cells = run_cell_updates(cells_in.clone(), updates.clone());
        assert_eq!(
            updated_cells[0].population,
            cells_in[0].population + updates[0].value + updates[1].value
        );
    }

    #[test]
    fn test_run_iteration() {
        let initial_state = IterationState {
            global_state: GlobalState { iterations: 0 },
            cells: get_demo_cells(),
        };

        let processes = get_demo_processes();
        let final_state = run_iteration(&processes, initial_state);
        assert_eq!(final_state.cells.len(), 3);
        assert_eq!(final_state.cells[0].population, 17);
        assert_eq!(final_state.cells[1].population, 45);
        assert_eq!(final_state.cells[2].population, 44);
    }
}
