use super::state::CellIndex;
use super::state::CellStateBase;
use geo::algorithm::geodesic_distance::GeodesicDistance;

pub fn check_is_neighbour<T: CellStateBase>(cell_a: &T, cell_b: &T) -> bool {
    if cell_a.id() == cell_b.id() {
        return false;
    }
    let distance = cell_a.position().geodesic_distance(&cell_b.position());
    if distance > 80000.0 {
        return false;
    }
    true
}

pub fn get_network_map<T: CellStateBase>(cells: &[T]) -> Vec<Vec<CellIndex>> {
    let mut network: Vec<Vec<CellIndex>> = Vec::new();
    for cell in cells.iter() {
        // println!("Finding neighbours for cell {}", cell.id);
        let mut cell_network: Vec<CellIndex> = Vec::new();
        for cell_n in cells.iter() {
            if check_is_neighbour(cell, cell_n) {
                cell_network.push(cell_n.id());
            }
        }
        network.push(cell_network);
    }
    network
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::example_state::CellState;
    use geo::point;

    #[test]
    fn returns_a_network() {
        let cells = vec![
            CellState::new(0, point!(x:5.54, y:-0.19), 12, None, None, None, None),
            CellState::new(1, point!(x:5.77, y:-0.02), 40, None, None, None, None),
            CellState::new(2, point!(x:5.94, y:0.42), 40, None, None, None, None),
        ];
        let network = get_network_map(&cells);
        assert_eq!(
            network,
            vec![
                vec![CellIndex(1)],
                vec![CellIndex(0), CellIndex(2)],
                vec![CellIndex(1)]
            ]
        );
    }

    #[test]
    fn checks_if_is_neighbour() {
        let cell_a = CellState::new(0, point!(x:5.54, y:-0.19), 12, None, None, None, None);
        let cell_b = CellState::new(1, point!(x:5.77, y:-0.02), 40, None, None, None, None);
        let cell_c = CellState::new(2, point!(x:5.94, y:0.42), 40, None, None, None, None);
        let are_neighbours = check_is_neighbour(&cell_a, &cell_b);
        assert_eq!(are_neighbours, true);
        let are_neighbours = check_is_neighbour(&cell_a, &cell_c);
        assert_eq!(are_neighbours, false);
        let are_neighbours = check_is_neighbour(&cell_b, &cell_c);
        assert_eq!(are_neighbours, true);
    }
}
