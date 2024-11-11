// use chess::{Board, ChessMove};
// use smallvec::SmallVec;
// use crate::setup::depth::Depth;
// use crate::setup::values::Value;
//
// pub const EXPECTED_PV_DISTANCE: usize = 4;
//
// #[derive(Debug, Clone)]
// pub struct RootNode {
//     pub state: Board,
// }
//
// #[derive(Debug, Clone)]
// pub struct Node {
//     // /// index within a static PvNode vector
//     // pub parent: usize,
//     pub moves_since_parent: SmallVec<[ChessMove; EXPECTED_PV_DISTANCE]>,
//     pub pv: bool,
//     pub alpha: Value,
//     pub beta: Value,
//     pub depth: Depth,
// }
//
// // unsafe impl Sync for PvNode {}
// unsafe impl Sync for Node {}
