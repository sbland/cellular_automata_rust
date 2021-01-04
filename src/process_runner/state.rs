use geo::point;
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
    pub population_attraction: f64,
    pub residential_capacity: u32,
}

impl Default for CellState {
    fn default() -> CellState {
        CellState {
            id: CellIndex(0),
            position: point!(x:0.0, y:0.0),
            population: 0,
            population_attraction: 1.0,
            residential_capacity: 0,
        }
    }
}

#[allow(dead_code)]
impl CellState {
    pub fn new(id: u32, pos: Point<f64>, population: u32) -> CellState {
        CellState {
            id: CellIndex(id),
            position: pos,
            population: population,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct IterationState {
    pub global_state: GlobalState,
    pub cells: Vec<CellState>,
}
