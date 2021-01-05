use num::NumCast;
use std::fmt;

use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellState;

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
            action: action,
            target_cell: target_cell,
            target_field: String::from(target_field),
            value: value,
        }
    }
}

type ProcessFuncT = Box<dyn Fn(&CellState, &Vec<&CellState>) -> Vec<CellUpdate>>;

pub struct Process {
    pub id: u32,
    pub func: ProcessFuncT,
}

#[allow(dead_code)]
impl Process {
    pub fn new(id: u32, func: ProcessFuncT) -> Process {
        Process { id: id, func: func }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement tests
}
