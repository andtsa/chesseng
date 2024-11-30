//! Move ordering functions
//!
//! https://www.chessprogramming.org/Move_Ordering

use std::fmt::Display;

use anyhow::anyhow;
use anyhow::Result;
use chess::BitBoard;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::Piece;
use chess::EMPTY;

use crate::opts::opts;
use crate::search::MV;
use crate::setup::values::Value;

/// A struct that holds a vector of moves, ordered by importance
#[derive(Debug)]
pub struct MoveOrdering {
    /// the principal variation move (if any)
    pub pv: Option<MV>,
    /// the rest of the moves
    pub rest: Vec<MV>,
}

/// convert a [`ChessMove`] to a [`MV`]
fn as_mv(m: &ChessMove) -> MV {
    MV(*m, Value::ZERO, false)
}

/// return all moves possible from this position, ordered by importance,
/// prioritising the principal variation
pub fn pv_ordered_moves(b: &Board, pv: &ChessMove) -> MoveOrdering {
    if !opts().unwrap().use_pv {
        return ordered_moves(b);
    }

    let mut ordering = MoveOrdering::empty();
    let mut rest = vec![];
    let mut mg = MoveGen::new_legal(b);

    mg.set_iterator_mask(BitBoard::from_square(pv.get_dest()));
    let moves_that_land_on_pv_square = mg.by_ref().collect::<Vec<ChessMove>>();
    for m in moves_that_land_on_pv_square {
        if m == *pv {
            ordering.pv = Some(MV(*pv, Value::ZERO, true));
        } else {
            rest.push(m);
        }
    }

    mg.set_iterator_mask(*b.pieces(Piece::Queen));
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Rook));
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Bishop));
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Knight));
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Pawn));
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(!EMPTY);
    rest.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    if let Some(pv) = &ordering.pv {
        rest.retain(|x| x != &pv.0);
    }

    ordering.rest = rest.iter().map(as_mv).collect();

    ordering
}

/// return all moves possible from this position, ordered by importance
pub fn ordered_moves(b: &Board) -> MoveOrdering {
    let mut moves = vec![];
    let mut mg = MoveGen::new_legal(b);

    mg.set_iterator_mask(*b.pieces(Piece::Queen));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Rook));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Bishop));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Knight));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Pawn));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    // mg.set_iterator_mask(*b.pieces(Piece::Queen));
    // moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    //
    mg.set_iterator_mask(!EMPTY);
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    MoveOrdering {
        pv: None,
        rest: moves.iter().map(as_mv).collect(),
    }
}

impl Display for MoveOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MoveOrdering: ")?;
        if let Some(m) = &self.pv {
            write!(f, "{} (pv), ", m.0)?
        }
        for m in &self.rest {
            write!(f, "{}, ", m.0)?
        }
        Ok(())
    }
}

impl MoveOrdering {
    /// create a new empty move ordering
    pub fn empty() -> Self {
        MoveOrdering {
            pv: None,
            rest: vec![],
        }
    }

    /// returns the number of moves in the vector.
    pub fn len(&self) -> usize {
        self.rest.len() + self.pv.iter().len()
    }

    /// are there any moves left?
    pub fn is_empty(&self) -> bool {
        self.rest.is_empty() && self.pv.is_none()
    }

    /// returns the most important move, if there is one.
    pub fn pop(&mut self) -> Option<MV> {
        if let Some(m) = self.pv.take() {
            Some(m)
        } else {
            self.rest.pop()
        }
    }

    /// get the move at the given index
    pub fn get(&self, idx: usize) -> Result<&MV> {
        if idx == 0 {
            self.pv.as_ref()
        } else {
            self.rest.get(idx - 1)
        }
        .ok_or(anyhow!("No move found"))
    }
}

impl<'a> IntoIterator for &'a MoveOrdering {
    type Item = &'a MV; // Iterator yields references to MV
    type IntoIter = std::iter::Chain<std::option::Iter<'a, MV>, std::slice::Iter<'a, MV>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pv.iter().chain(self.rest.iter()) // Both are slice iterators
    }
}
#[cfg(test)]
#[path = "tests/moveordering.rs"]
mod tests;
