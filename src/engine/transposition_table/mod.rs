//! transposition tables!
use crate::setup::depth::Depth;

/// The type for a board hash
pub type BoardHash = u64;

/// The bound of an evaluation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalBound {
    /// The evaluation is precise
    Exact,
    /// The evaluation is a lower bound
    LowerBound,
    /// The evaluation is an upper bound
    UpperBound,
}

/// A single entry in the transposition table
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableEntry {
    /// the evaluation this entry holds
    pub evaluation: i32,
    /// The depth this entry was computed from
    pub computed_from_depth: Depth,
    /// The depth this entry was computed for
    pub computed_for_depth: Depth,
    /// The bound of the evaluation
    pub bound: EvalBound,
}

/// The transposition table
pub type TranspositionTable = lockfree::map::Map<BoardHash, TableEntry>;
