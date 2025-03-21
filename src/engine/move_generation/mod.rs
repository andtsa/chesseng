//! Move ordering functions
//!
//! https://www.chessprogramming.org/Move_Ordering

use std::fmt::Display;

use chess::BitBoard;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use heuristics::score_move;
use history::MoveHistory;

pub mod heuristics;
pub mod history;

use crate::move_generation::heuristics::move_gen_ordering;
use crate::position::Position;

/// A struct that holds a vector of moves, ordered by importance
#[derive(Debug)]
pub struct MoveOrdering(pub Vec<ChessMove>);

/// return all moves possible from this position, ordered by importance,
/// prioritising the principal variation
pub fn pv_ordered_moves(pos: &Position, pv: &ChessMove, hist: &MoveHistory) -> MoveOrdering {
    let mut moves = vec![];
    let mut mg = MoveGen::new_legal(&pos.chessboard);

    mg.set_iterator_mask(BitBoard::from_square(pv.get_dest()));
    for m in mg.by_ref().collect::<Vec<ChessMove>>() {
        if m != *pv {
            moves.push(m);
        }
    }

    move_gen_ordering(&pos.chessboard, mg, &mut moves);

    let mut valued = moves
        .iter()
        .map(|mv| (*mv, score_move(pos, mv, hist)))
        .collect::<Vec<_>>();
    valued.sort_unstable_by_key(|e| e.1);

    MoveOrdering(
        [*pv]
            .into_iter()
            .chain(valued.into_iter().map(|x| x.0))
            .collect::<Vec<_>>(),
    )
}

/// return all moves possible from this position, ordered by importance
pub fn ordered_moves(pos: &Position, hist: &MoveHistory) -> MoveOrdering {
    let mut moves = vec![];
    let mg = MoveGen::new_legal(&pos.chessboard);

    move_gen_ordering(&pos.chessboard, mg, &mut moves);

    let mut valued = moves
        .iter()
        .map(|mv| (*mv, score_move(pos, mv, hist)))
        .collect::<Vec<_>>();
    valued.sort_unstable_by_key(|e| e.1);

    MoveOrdering(valued.into_iter().map(|x| x.0).collect())
}

/// return all moves possible from this position, unordered
#[inline]
pub fn unordered_moves(b: &Board) -> MoveOrdering {
    MoveOrdering(MoveGen::new_legal(b).collect())
}

impl Display for MoveOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MoveOrdering: ")?;
        for m in &self.0 {
            write!(f, "{}, ", m)?
        }
        Ok(())
    }
}

impl MoveOrdering {
    /// returns the number of moves in the vector.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// are there any moves left?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// returns the last move in the vector, or None if it is empty.
    pub fn pop(&mut self) -> Option<ChessMove> {
        self.0.pop()
    }

    /// No moves in this [`MoveOrdering`]!
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl IntoIterator for MoveOrdering {
    type Item = ChessMove;
    type IntoIter = std::vec::IntoIter<ChessMove>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[path = "./tests/moveordering.rs"]
mod tests;
