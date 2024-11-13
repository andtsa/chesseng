#![allow(dead_code)]
use crate::setup::values::Value;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum EvalBound {
    Exact,
    LowerBound,
    UpperBound,
}

impl EvalBound {
    pub(crate) fn from_bounds(p0: Value, p1: Value, p2: Value) -> Self {
        if p0 == p1 {
            Self::Exact
        } else if p0 == p2 {
            Self::LowerBound
        } else {
            Self::UpperBound
        }
    }
}
