use geo::Point;

#[derive(Clone)]
pub struct GlobalState {
    pub iterations: u32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CellIndex(pub u32);

#[derive(Clone, Copy)]
pub struct CellState {
    pub id: CellIndex,
    pub position: Point<f64>,
    pub population: u32,
}

#[allow(dead_code)]
impl CellState {
    pub fn new(id: u32, pos: Point<f64>, population: u32) -> CellState {
        CellState {
            id: CellIndex(id),
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
