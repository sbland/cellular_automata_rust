use geo::Point;

#[derive(Clone)]
pub struct GlobalState {
    pub iterations: u32,
}

#[derive(Clone, Copy)]
pub struct CellState {
    pub id: u32,
    pub position: Point<f64>,
    pub population: u32,
}

impl CellState {
    pub fn new(id: u32, pos: Point<f64>, population: u32) -> CellState {
        CellState {
            id: id,
            position: pos,
            population: population,
        }
    }
}

#[derive(Clone)]
pub struct IterationState {
    pub global_state: GlobalState,
    pub cells: Vec<CellState>,
}
