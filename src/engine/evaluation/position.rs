//! evaluating the positions of the pieces on the board
use std::ops::BitAnd;

use chess::Board;
use chess::ChessMove;
use chess::Color;
use chess::Square;

use crate::evaluation::bitboards::EG_PESTO_TABLE;
use crate::evaluation::bitboards::MG_PESTO_TABLE;
use crate::evaluation::bitboards::POS_PIECE_TYPES;
use crate::evaluation::Interp;
use crate::setup::values::Value;

/// mobility weights for each piece type and phase of the game (.0 is beginning,
/// .1 is middle, .2 is end)
pub const MOBILITY_WEIGHTS: [(Value, Value, Value); 6] = [
    (Value(1), Value(2), Value(3)),  // Pawn
    (Value(3), Value(5), Value(2)),  // Knight
    (Value(3), Value(4), Value(6)),  // Bishop
    (Value(2), Value(5), Value(6)),  // Rook
    (Value(1), Value(4), Value(3)),  // Queen
    (Value(-2), Value(0), Value(5)), // King
];

/// returns the benefit this side has from its pieces' positions
pub fn piece_position_benefit_for_side(pos: &Board, color: Color, interp: Interp) -> Value {
    let mut value = Value::ZERO;
    let side = pos.color_combined(color);

    for (idx, pt) in POS_PIECE_TYPES.into_iter().enumerate() {
        let bb = pos.pieces(pt).bitand(side);
        let mg = MG_PESTO_TABLE[idx];
        let eg = EG_PESTO_TABLE[idx];

        for sq in bb {
            let (row, col) = sq_pi(sq, color);
            value += Value::from(
                ((mg[row][col] as f64) * (interp.0 + interp.1)) + (eg[row][col] as f64 * interp.2),
            );
        }

        // crate::optlog!(eval;trace;"ppbfs after {idx} : {:?}", value);
    }

    value
}

/// Converts a square to a pesto index.
///
/// `side_square_to_pesto_index(sq: `[`Square`]
/// `, color: `
/// [`Color`]
/// `) -> (usize: row, usize: col)`
pub fn sq_pi(sq: Square, color: Color) -> (usize, usize) {
    let (rank, file) = (sq.get_rank().to_index(), sq.get_file().to_index());
    let rank = if color == Color::Black {
        rank
    } else {
        7 - rank
    };
    // let file = if color == Color::Black {
    //     file
    // } else {
    //     file
    // };
    (rank, file)
}

/// mobility evaluation for the side to move
pub fn current_mobility_evaluation(board: &Board, interp: Interp, moves: &[ChessMove]) -> Value {
    let mut mobility_score = Value::ZERO;

    for (idx, piece) in POS_PIECE_TYPES.iter().enumerate() {
        let weight = MOBILITY_WEIGHTS[idx].0 * interp.0
            + MOBILITY_WEIGHTS[idx].1 * interp.1
            + MOBILITY_WEIGHTS[idx].2 * interp.2;

        mobility_score += Value(
            moves
                .iter()
                .filter(|m| board.piece_on(m.get_source()) == Some(*piece))
                .count() as i16,
        ) * weight;
    }

    mobility_score
}

#[cfg(test)]
#[path = "tests/positions.rs"]
mod tests;
