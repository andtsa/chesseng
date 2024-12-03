//! the actual logic of move ordering
#![allow(dead_code)]
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::Piece;
use chess::EMPTY;

use crate::evaluation::material::INITIAL_VALUES;
use crate::setup::values::Value;

/// the actual logic of move ordering
#[inline]
pub fn move_gen_ordering(b: &Board, mut mg: MoveGen, buf: &mut Vec<ChessMove>) {
    mg.set_iterator_mask(*b.pieces(Piece::Queen));
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Rook));
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Bishop));
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Knight));
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(*b.pieces(Piece::Pawn));
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(!EMPTY);
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
}

/// Assigns a score to a capture move based on MVV-LVA
pub fn mvv_lva_score(b: &Board, mv: &ChessMove) -> Value {
    match (b.piece_on(mv.get_source()), b.piece_on(mv.get_dest())) {
        (Some(capturing_piece), Some(captured_piece)) => {
            let capturing_value = INITIAL_VALUES[capturing_piece as usize];
            let captured_value = INITIAL_VALUES[captured_piece as usize];
            captured_value - capturing_value
        }
        _ if cfg!(debug_assertions) => unreachable!("mvv_lva_score called with a non-capture move"),
        _ => Value::ZERO,
    }
}
