use super::state::CellState;

pub fn check_is_neighbour(cell_a: CellState) {}

pub fn get_network_map(cells: &Vec<CellState>) -> Vec<Vec<u32>> {
    let mut network: Vec<Vec<u32>> = Vec::new();
    for cell in cells.iter() {
        println!("Finding neighbours for cell {}", cell.id);
        let mut cell_network: Vec<u32> = Vec::new();
        for cell_n in cells.iter() {
            if cell.id != cell_n.id {
                println!("check network");
                cell_network.push(cell_n.id);
            }
        }
        network.push(cell_network);
    }
    network
}
