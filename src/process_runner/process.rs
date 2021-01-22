use num::NumCast;
use std::fmt;

use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellStateBase;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ADD, // can also add a neg val
    SET,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Value {
    NumberF(f64),
    NumberI(i32),
}

impl From<Value> for u32 {
    fn from(src: Value) -> u32 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
        }
    }
}
impl From<Value> for i32 {
    fn from(src: Value) -> i32 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
        }
    }
}
impl From<Value> for f64 {
    fn from(src: Value) -> f64 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
        }
    }
}

// TODO: Work out how to implement below to allow boiler plate
// OR use a macro
// impl<Z: num::NumCast> From<Value> for Z {
//     fn from(src: Value) -> Z {
//         match src {
//             Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
//             Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
//         }
//     }
// }

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::NumberF(v) => write!(f, "{}", v),
            Value::NumberI(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellUpdate {
    pub action: Action,
    pub target_cell: CellIndex,
    pub target_field: String,
    pub value: Value,
}

#[allow(unused)]
impl CellUpdate {
    pub fn new(
        target_cell: CellIndex,
        value: Value,
        action: Action,
        target_field: &str,
    ) -> CellUpdate {
        CellUpdate {
            action,
            target_cell,
            target_field: String::from(target_field),
            value,
        }
    }
}

type ProcessFuncT<T> = Box<dyn Fn(&T, &[&T]) -> Vec<CellUpdate>>;

pub struct Process<T: CellStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<T>,
}

#[allow(dead_code)]
impl<T: CellStateBase> Process<T> {
    pub fn new(id: u32, func: ProcessFuncT<T>) -> Process<T> {
        Process { id, func }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement tests
}
