use num::NumCast;
use std::ops;

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

// TODO: can we use 'into' here instread
impl Value {
    pub fn to<Z: num::ToPrimitive + NumCast>(&self) -> Z {
        match self {
            Value::NumberF(v) => NumCast::from::<f64>(*v).unwrap(),
            Value::NumberI(v) => NumCast::from::<i32>(*v).unwrap(),
        }
    }
}

impl ops::Add<f64> for Value {
    type Output = f64;
    fn add(self, rhs: f64) -> f64 {
        match self {
            Value::NumberF(v) => v + rhs,
            Value::NumberI(v) => v as f64 + rhs,
        }
    }
}

impl ops::AddAssign<f64> for Value {
    fn add_assign(&mut self, rhs: f64) {
        match self {
            Value::NumberF(v) => *v += rhs,
            Value::NumberI(v) => *v += rhs as i32,
        };
    }
}

impl ops::AddAssign<Value> for u32 {
    fn add_assign(&mut self, rhs: Value) {
        match rhs {
            Value::NumberF(v) => *self += v as u32,
            Value::NumberI(v) => *self += v as u32,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellUpdate {
    pub action: Action,
    pub target_cell: CellIndex,
    pub value: Value,
}

#[allow(unused)]
impl CellUpdate {
    pub fn new(target_cell: CellIndex, value: Value, action: Action) -> CellUpdate {
        CellUpdate {
            action: action,
            target_cell: target_cell,
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
