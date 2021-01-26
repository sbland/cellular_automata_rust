use crate::process_runner::process::Action;
use crate::process_runner::process::CellUpdate;
use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellStateBase;
use geo::point;
use geo::Point;
use std::convert::From;

///
///  #[derive(Debug, Clone, PartialEq, Copy)]
///  pub struct CellState {
///      pub id: CellIndex,
///      pub position: Point<f64>,
///      pub population: u32,
///      ...
///  }
///
macro_rules! create_struct {
    ($cls: ident, [$(($ismut:ident $field_name:ident, $type:ty, $default:expr))*]) => {
        #[derive(Debug, Clone, PartialEq, Copy)]
        pub struct $cls {
            $(pub $field_name: $type),*
        }
    };
}

/*
    impl Default for CellState {
        fn default() -> CellState {
            CellState {
                id: CellIndex(0),
                position: point!(x:0.0, y:0.0),
                ...
            }
        }
    }
*/
macro_rules! impl_default {
    ($cls: ident, [$(($ismut:ident $field_name:ident, $type:ty, $default:expr))*]) => {
        impl Default for $cls {
            fn default() -> $cls {
                $cls {
                    $($field_name: $default),*
                }
            }
        }
    };
}

/// Apply the action to the field
///
/// This performs some type conversions and modifies the field
/// The target field must be mutable
macro_rules! action {
    // If Action::ADD then we simply add
    (ismut Action::ADD, $field_name:ident, $type:ty, $Self: ident, $cell_action: ident) => {
        $Self.$field_name = ($Self.$field_name as i32 + i32::from($cell_action.value)) as $type
    };
    // We have to include SUB so that we can add negative values to unsigned types
    (ismut Action::SUB, $field_name:ident, $type:ty, $Self: ident, $cell_action: ident) => {
        $Self.$field_name =
            ($Self.$field_name as $type - <$type>::from($cell_action.value)) as $type
    };
    (ismut Action::SET, $field_name:ident, $type:ty, $Self: ident, $cell_action: ident) => {
        $Self.$field_name = <$type>::from($cell_action.value)
    };
    // (cons $_:stmt) => {
    //     panic!()
    // };
    (cons $_:path, $__:ident, $___:ty, $____: ident, $_____: ident) => {
        panic!()
    };
}

/// Match a field to an action
///
/// Example Output:
/// ```
/// match cell_action.target_field.as_str() {
///     "population" => {
///         self.population = (self.population as i32 + i32::from(cell_action.value)) as u32
///     }
///     ...
///     &_ => (),
/// }
/// ```
macro_rules! map_action_to_fields {
    ((Action::$action_type: ident), [$(($mut: ident $field_name:ident, $type:ty, $default:expr)),*], $Self: ident, $cell_action: ident) => {
        match $cell_action.target_field.as_str() {
            $(stringify!($field_name) => action!($mut Action::$action_type, $field_name, $type, $Self, $cell_action)),*,
            &_ => (),
        }
    };
}

/// Implement the apply action method
macro_rules! impl_cell_state_apply {
    ($cls: ident, [$($field_info:tt) *]) => {
        impl $cls {
            /// Apply CellUpdates to the cell
            pub fn apply(&mut self, cell_action: &CellUpdate) {
                let action = &cell_action.action;
                match action {
                    // Map each field to each action
                    Action::ADD => map_action_to_fields!((Action::ADD), [$($field_info), *], self, cell_action),
                    Action::SUB => map_action_to_fields!((Action::SUB), [$($field_info), *], self, cell_action),
                    Action::SET => map_action_to_fields!((Action::SET), [$($field_info), *], self, cell_action),
                }
            }
        }
    };
}

/// Implement the new associate function
macro_rules! impl_new {
    ($cls: ident, [$(($mut: ident $field_name:ident, $type:ty, $default:expr)),*]) => {
        pub fn new(
            // TODO: We currently have to redefine the fields here
            // $($field_name:$type),
            id: u32,
            pos: impl Into<Option<Point<f64>>>,
            population: impl Into<Option<u32>>,
            population_attraction: impl Into<Option<f64>>,
            residential_capacity: impl Into<Option<u32>>,
            population_birth_rate: impl Into<Option<f64>>,
            population_death_rate: impl Into<Option<f64>>,
        ) -> CellState {
            CellState {
                // $($field_name:$default),
                id: CellIndex(id),
                position: pos.into().unwrap_or(point!(x: 0.0, y: 0.0)),
                population: population.into().unwrap_or(0),
                population_attraction: population_attraction.into().unwrap_or(1.0),
                residential_capacity: residential_capacity.into().unwrap_or(0),
                population_birth_rate: population_birth_rate.into().unwrap_or(1.0),
                population_death_rate: population_death_rate.into().unwrap_or(0.2),
            }
        }
    };
}

/// Implement class functions
macro_rules! impl_cls_funcs {
    ($cls: ident, [$($field_info:tt) *]) => {
        #[allow(dead_code)]
        impl $cls {
            impl_new!($cls, [$($field_info), *]);

            pub fn new_default(id: u32) -> $cls {
                $cls {
                    id: CellIndex(id),
                    ..Default::default()
                }
            }

            // TODO: Implement random generation
            // pub fn new_random(id: u32) -> $cls {
            //     $cls {
            //         $($field_name: $default),*
            //         id: CellIndex(id),
            //         position: point!(x: 0.0, y: 0.0),
            //         population: 100,
            //         population_attraction: 1.0,
            //         residential_capacity: 100,
            //         population_birth_rate: 1.0,
            //         population_death_rate: 0.2,
            //     }
            // }
        }
    };
}

macro_rules! impl_struct {
    ($cls: ident, [$($field_info:tt),*]) => {
        create_struct!($cls, [$($field_info)*]);
        impl_default!($cls, [$($field_info)*]);
        impl_cell_state_apply!($cls, [$($field_info)*]);
        impl_cls_funcs!($cls, [$($field_info)*]);
    };
}

impl_struct!(
    CellState,
     [
        (cons id, CellIndex, CellIndex(0)),
        (cons position, Point<f64>, point!(x:0.0, y:0.0)),
        (ismut population, u32, 0),
        (ismut population_attraction, f64, 1.0),
        (ismut residential_capacity, u32, 0),
        (ismut population_birth_rate, f64, 1.0),
        (ismut population_death_rate, f64, 0.0)
    ]
);

impl CellStateBase for CellState {
    fn id(&self) -> CellIndex {
        self.id
    }
    fn position(&self) -> Point<f64> {
        self.position
    }
    fn apply(&mut self, cell_action: &CellUpdate) {
        self.apply(cell_action)
    }
}
