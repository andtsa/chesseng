//! the actual logic of move ordering
#![allow(dead_code)]
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::EMPTY;

use super::history::MoveHistory;
use crate::evaluation::material::INITIAL_VALUES;
use crate::position::Position;
use crate::setup::values::Value;

/// the actual logic of move ordering
#[inline]
pub fn move_gen_ordering(_b: &Board, mut mg: MoveGen, buf: &mut Vec<ChessMove>) {
    // mg.set_iterator_mask(*b.pieces(Piece::Queen));
    // buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    //
    // mg.set_iterator_mask(*b.pieces(Piece::Rook));
    // buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    //
    // mg.set_iterator_mask(*b.pieces(Piece::Bishop));
    // buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    //
    // mg.set_iterator_mask(*b.pieces(Piece::Knight));
    // buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    //
    // mg.set_iterator_mask(*b.pieces(Piece::Pawn));
    // buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(!EMPTY);
    buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
}

/// assign a score to each move so we can sort them
pub fn score_move(pos: &Position, mv: &ChessMove, hist: &MoveHistory) -> Value {
    if pos.chessboard.piece_on(mv.get_dest()).is_some() {
        // Use MVV-LVA for captures
        return mvv_lva_score(&pos.chessboard, mv);
    }

    let mut score = Value::ZERO;
    // add killer move bonus
    if hist.is_killer(mv, pos.moves_played) {
        score += Value(8000);
    }
    // add history heuristic bonus
    score += hist.history_score(mv);
    score
}

/// Assigns a score to a capture move based on MVV-LVA
fn mvv_lva_score(b: &Board, mv: &ChessMove) -> Value {
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
