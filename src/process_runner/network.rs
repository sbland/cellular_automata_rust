use super::state::CellState;
// use geo::algorithm::euclidean_distance

pub fn check_is_neighbour(cell_a: &CellState, cell_b: &CellState) -> bool {
    if cell_a.id == cell_b.id {
        return false;
    }
    true
}

pub fn get_network_map(cells: &Vec<CellState>) -> Vec<Vec<u32>> {
    let mut network: Vec<Vec<u32>> = Vec::new();
    for cell in cells.iter() {
        println!("Finding neighbours for cell {}", cell.id);
        let mut cell_network: Vec<u32> = Vec::new();
        for cell_n in cells.iter() {
            println!("check network");
            if check_is_neighbour(&cell, &cell_n) {
                cell_network.push(cell_n.id);
            }
        }
        network.push(cell_network);
    }
    network
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    #[test]
    fn returns_a_network() {
        let cells = vec![
            CellState::new(0, point!(x:0, y:0), 12),
            CellState::new(1, point!(x:0, y:1), 40),
        ];
        let network = get_network_map(&cells);
        assert_eq!(network, vec![vec![1], vec![0]]);
    }
}
