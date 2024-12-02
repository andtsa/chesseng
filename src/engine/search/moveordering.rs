//! Move ordering functions
//!
//! https://www.chessprogramming.org/Move_Ordering

use std::fmt::Display;

use chess::BitBoard;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::Piece;
use chess::EMPTY;

/// A struct that holds a vector of moves, ordered by importance
#[derive(Debug)]
pub struct MoveOrdering(pub Vec<ChessMove>);

/// return all moves possible from this position, ordered by importance,
/// prioritising the principal variation
pub fn pv_ordered_moves(b: &Board, pv: &ChessMove) -> MoveOrdering {
    let mut moves = vec![];
    let mut mg = MoveGen::new_legal(b);

    mg.set_iterator_mask(BitBoard::from_square(pv.get_dest()));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

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

    mg.set_iterator_mask(!EMPTY);
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    MoveOrdering(moves)
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

    MoveOrdering(moves)
}

/// return all moves possible from this position, unordered
pub fn unordered_moves(b: &Board) -> MoveOrdering {
    let mut moves = vec![];
    let mut mg = MoveGen::new_legal(b);

    mg.set_iterator_mask(!EMPTY);
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    MoveOrdering(moves)
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
}

impl IntoIterator for MoveOrdering {
    type Item = ChessMove;
    type IntoIter = std::vec::IntoIter<ChessMove>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
