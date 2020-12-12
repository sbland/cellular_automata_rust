pub struct GlobalData {
    pub iterations: u32,
}

pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }
}

pub struct CellState {
    pub id: u32,
    pub position: Point,
    pub population: u32,
}

impl CellState {
    pub fn new(id: u32, pos: Point, population: u32) -> CellState {
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
