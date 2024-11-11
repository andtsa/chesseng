use crate::setup::depth::Depth;

pub type BoardHash = u64;

pub enum EvalBound {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct TableEntry {
    // pub best_move: Option<ChessMove>,
    pub evaluation: i32,
    pub computed_from_depth: Depth,
    pub computed_for_depth: Depth,
    pub bound: EvalBound,
}

pub type TranspositionTable = lockfree::map::Map<BoardHash, TableEntry>;
