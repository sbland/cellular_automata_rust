use geo::Point;

pub struct GlobalData {
    pub iterations: u32,
}

pub struct CellState {
    pub id: u32,
    pub position: Point<u32>,
    pub population: u32,
}

impl CellState {
    pub fn new(id: u32, pos: Point<u32>, population: u32) -> CellState {
        CellState {
            id: id,
            position: pos,
            population: population,
        }
    }
}

pub struct IterationState {
    pub global_data: GlobalData,
    pub cells: Vec<CellState>,
}
