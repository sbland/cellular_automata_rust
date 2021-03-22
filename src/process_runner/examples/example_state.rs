use crate::process_runner::cells::state::CellIndex;
use crate::process_runner::cells::state::CellStateBase;
use crate::process_runner::global::state::GlobalStateBase;
use geo::point;
use geo::Coordinate;
use geo::Point;

type PointF64 = Point<f64>;

#[derive(Debug, Clone, PartialEq)]
pub struct CellState {
    pub id: CellIndex,
    pub position: PointF64,
    pub population: u32,
    pub peep_ids: Vec<u32>,
}

impl Default for CellState {
    fn default() -> CellState {
        CellState {
            id: CellIndex(0),
            position: point!(x:0.0, y:0.0),
            population: 0,
            peep_ids: vec![1, 2, 3],
        }
    }
}

impl CellState {
    pub fn new(
        id: u32,
        pos: impl Into<Option<Point<f64>>>,
        population: impl Into<Option<u32>>,
    ) -> CellState {
        CellState {
            id: CellIndex(id),
            position: pos.into().unwrap_or(Point(Coordinate { x: 0.0, y: 0.0 })),
            population: population.into().unwrap_or(0),
            peep_ids: vec![1, 2, 3],
        }
    }
}

impl CellStateBase for CellState {
    fn id(&self) -> CellIndex {
        self.id
    }
    fn position(&self) -> Point<f64> {
        self.position
    }
}

// Global State
#[derive(Debug, Copy, Clone, Default)]
pub struct GlobalState {
    pub iterations: u32,
    pub population: u32,
}

#[allow(dead_code)]
impl GlobalState {
    pub fn new(population: u32) -> GlobalState {
        GlobalState {
            iterations: 1,
            population,
        }
    }
}

impl GlobalStateBase for GlobalState {}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    #[test]
    fn test_new_cellstate() {
        let cell = CellState::new(0, point!(x: 0.0, y: 0.0), 10);
        assert_eq!(
            cell,
            CellState {
                id: CellIndex(0),
                position: point!(x: 0.0, y: 0.0),
                population: 10,
                peep_ids: vec![1, 2, 3],
            }
        );
    }
}
