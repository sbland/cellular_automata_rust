use geo::Point;
use std::fmt;

pub trait CellStateBase: fmt::Debug + Clone {
    fn id(&self) -> CellIndex;
    fn position(&self) -> Point<f64>;
    fn randomize(&self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CellIndex(pub u32);

impl fmt::Display for CellIndex {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl From<CellIndex> for u32 {
    fn from(src: CellIndex) -> u32 {
        let CellIndex(v) = src;
        v as u32
    }
}

#[cfg(test)]
mod tests {}
