use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use geo::point;
use geo::Point;

#[derive(Clone)]
pub struct GlobalState {
    pub iterations: u32,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CellIndex(pub u32);

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CellState {
    pub id: CellIndex,
    pub position: Point<f64>,
    pub population: u32,
    pub population_attraction: f64,
    pub residential_capacity: u32,
    pub population_birth_rate: f64,
    pub population_death_rate: f64,
}

impl Default for CellState {
    fn default() -> CellState {
        CellState {
            id: CellIndex(0),
            position: point!(x:0.0, y:0.0),
            population: 0,
            population_attraction: 1.0,
            residential_capacity: 0,
            population_birth_rate: 1.0,
            population_death_rate: 0.2,
        }
    }
}

// TODO: make this a derive macro
impl CellState {
    /// Apply CellUpdates to the cell
    pub fn apply(&mut self, cell_action: &CellUpdate) {
        macro_rules! action {
            ("ADD", $field_name:ident, $type:ty) => {
                self.$field_name = (self.$field_name as i32 + i32::from(cell_action.value)) as $type
            };
            ("SET", $field_name:ident, $type:ty) => {
                self.$field_name = <$type>::from(cell_action.value)
            };
        }
        match cell_action.action {
            // TODO: implement for all cell fields
            Action::ADD => match cell_action.target_field.as_str() {
                "population" => action!("ADD", population, u32),
                "population_attraction" => action!("ADD", population_attraction, f64),
                &_ => (),
            },
            Action::SET => match cell_action.target_field.as_str() {
                "population" => action!("SET", population, u32),
                "population_attraction" => action!("SET", population_attraction, f64),
                &_ => (),
            },
        }
    }
}

#[allow(dead_code)]
impl CellState {
    pub fn new(
        id: u32,
        pos: Point<f64>,
        population: impl Into<Option<u32>>,
        population_attraction: impl Into<Option<f64>>,
        residential_capacity: impl Into<Option<u32>>,
        population_birth_rate: impl Into<Option<f64>>,
        population_death_rate: impl Into<Option<f64>>,
    ) -> CellState {
        CellState {
            id: CellIndex(id),
            position: pos,
            population: population.into().unwrap_or(0),
            population_attraction: population_attraction.into().unwrap_or(1.0),
            residential_capacity: residential_capacity.into().unwrap_or(0),
            population_birth_rate: population_birth_rate.into().unwrap_or(1.0),
            population_death_rate: population_death_rate.into().unwrap_or(0.2),
        }
    }

    pub fn new_default(id: u32) -> CellState {
        CellState {
            // TODO: Implement random generation
            id: CellIndex(id),
            ..Default::default()
        }
    }

    pub fn new_random(id: u32) -> CellState {
        CellState {
            // TODO: Implement random generation
            id: CellIndex(id),
            position: point!(x: 0.0, y: 0.0),
            population: 100,
            population_attraction: 1.0,
            residential_capacity: 100,
            population_birth_rate: 1.0,
            population_death_rate: 0.2,
        }
    }
}

#[derive(Clone)]
pub struct IterationState {
    pub global_state: GlobalState,
    pub cells: Vec<CellState>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_runner::process::Value;

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
    fn test_cellstate_apply_negative() {
        let mut cell = CellState::new_default(0);
        cell.population = 100;
        let update = CellUpdate::new(CellIndex(0), Value::NumberI(-1), Action::ADD, "population");
        cell.apply(&update);
        assert_eq!(cell.population, 99);
    }
}
