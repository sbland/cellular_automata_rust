use crate::process_runner::process::CellUpdate;
use geo::Point;

pub trait CellStateBase {
    fn id(&self) -> CellIndex;
    fn position(&self) -> Point<f64>;
    fn apply(&mut self, cell_action: &CellUpdate);
}

#[derive(Clone)]
pub struct GlobalState {
    pub iterations: u32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CellIndex(pub u32);

impl Into<String> for CellIndex {
    fn into(self) -> String {
        let CellIndex(v) = self;
        format!("{}", v)
    }
}

impl Into<usize> for CellIndex {
    fn into(self) -> usize {
        let CellIndex(v) = self;
        v as usize
    }
}

#[derive(Clone)]
pub struct IterationState<T: CellStateBase> {
    pub global_state: GlobalState,
    pub cells: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::example_state::CellState;
    use crate::process_runner::process::Action;
    use crate::process_runner::process::Value;
    use geo::point;

    #[test]
    fn test_new_cellstate() {
        let cell = CellState::new(0, point!(x: 0.0, y: 0.0), 10, None, None, None, None);
        assert_eq!(
            cell,
            CellState {
                id: CellIndex(0),
                position: point!(x: 0.0, y: 0.0),
                population: 10,
                population_attraction: 1.0,
                residential_capacity: 0,
                population_birth_rate: 1.0,
                population_death_rate: 0.2,
            }
        );
    }

    #[test]
    fn test_cellstate_apply() {
        let mut cell = CellState::new_default(0);
        cell.population = 100;
        let update = CellUpdate::new(CellIndex(0), Value::NumberI(3), Action::ADD, "population");
        cell.apply(&update);
        assert_eq!(cell.population, 103);
    }

    #[test]
    fn test_cellstate_apply_pop_attr() {
        let mut cell = CellState::new_default(0);
        cell.population_attraction = 100.0;
        let update = CellUpdate::new(
            CellIndex(0),
            Value::NumberI(3),
            Action::ADD,
            "population_attraction",
        );
        cell.apply(&update);
        assert_eq!(cell.population_attraction, 103.0);
    }

    #[test]
    fn test_cellstate_apply_negative() {
        let mut cell = CellState::new_default(0);
        cell.population = 100;
        let update = CellUpdate::new(CellIndex(0), Value::NumberI(-1), Action::ADD, "population");
        cell.apply(&update);
        assert_eq!(cell.population, 99);
    }
}
