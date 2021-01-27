/// Cell Process Module
///
/// 'a lifetime represents a single iteration
///
use num::NumCast;
use std::fmt;

use crate::process_runner::state::CellIndex;
use crate::process_runner::state::CellStateBase;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ADD,
    SUB,
    SET,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Value<'a> {
    NumberF(f64),
    NumberI(i32),
    Vector(&'a Value<'a>),
}

impl<'a> From<Value<'a>> for u32 {
    fn from(src: Value) -> u32 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
            Value::Vector(_v) => panic!("Cannot cast from vector to number "),
        }
    }
}
impl<'a> From<Value<'a>> for i32 {
    fn from(src: Value) -> i32 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
            Value::Vector(_v) => panic!("Cannot cast from vector to number "),
        }
    }
}
impl<'a> From<Value<'a>> for f64 {
    fn from(src: Value) -> f64 {
        match src {
            Value::NumberF(v) => NumCast::from::<f64>(v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(v).unwrap(),
            Value::Vector(_v) => panic!("Cannot cast from vector to number "),
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

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::NumberF(v) => write!(f, "{}", v),
            Value::NumberI(v) => write!(f, "{}", v),
            Value::Vector(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellUpdate<'a> {
    pub action: Action,
    pub target_cell: CellIndex,
    pub target_field: String,
    pub value: Value<'a>,
}

#[allow(unused)]
impl<'a> CellUpdate<'a> {
    pub fn new(
        target_cell: CellIndex,
        value: Value<'a>,
        action: Action,
        target_field: &str,
    ) -> CellUpdate<'a> {
        CellUpdate {
            action,
            target_cell,
            target_field: String::from(target_field),
            value,
        }
    }
}

type ProcessFuncT<'a, T> = Box<dyn Fn(&T, &Vec<&T>) -> Vec<CellUpdate<'a>>>;

pub struct Process<'a, T: CellStateBase> {
    pub id: u32,
    pub func: ProcessFuncT<'a, T>,
}

#[allow(dead_code)]
impl<'a, T: CellStateBase> Process<'a, T> {
    pub fn new(id: u32, func: ProcessFuncT<T>) -> Process<T> {
        Process { id, func }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement tests
}
