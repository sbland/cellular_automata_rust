use geo::Point;

pub trait CellStateBase {
    fn id(&self) -> CellIndex;
    fn position(&self) -> Point<f64>;
}

#[derive(Clone, Default)]
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

impl From<CellIndex> for u32 {
    fn from(src: CellIndex) -> u32 {
        let CellIndex(v) = src;
        v as u32
    }
}

#[derive(Clone)]
pub struct IterationState<T: CellStateBase> {
    pub global_state: GlobalState,
    pub cells: Vec<T>,
    pub network: Vec<Vec<CellIndex>>,
}

#[cfg(test)]
mod tests {}
