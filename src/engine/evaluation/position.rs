//! evaluating the positions of the pieces on the board
use std::ops::BitAnd;

use chess::BitBoard;
use chess::Board;
use chess::Color;
use chess::Piece;
use chess::Square;

use crate::evaluation::Interp;
use crate::evaluation::bitboards::EG_PESTO_TABLE;
use crate::evaluation::bitboards::MG_PESTO_TABLE;
use crate::evaluation::bitboards::POS_PIECE_TYPES;
use crate::setup::values::Value;

/// bitboard of dark squares
const DARK_SQUARES: BitBoard = BitBoard(0xAA55AA55AA55AA55);
/// bitboard of light squares
const LIGHT_SQUARES: BitBoard = BitBoard(0x55AA55AA55AA55AA);

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
    (rank, file)
}

/// check how much are my bishops blocked by other pieces.
///
/// issue: engine sacks bishops to minimise penalty
#[allow(dead_code)]
pub fn bishop_penalty(pos: &Board, side: Color) -> Value {
    let bishop_squares = pos.pieces(Piece::Bishop) & pos.color_combined(side);
    let mut penalty = Value::ZERO;

    let dark_bishops = bishop_squares.bitand(DARK_SQUARES);
    let light_bishops = bishop_squares.bitand(LIGHT_SQUARES);

    penalty += Value::from((pos.combined() & DARK_SQUARES).popcnt() * dark_bishops.popcnt());
    penalty += Value::from((pos.combined() & LIGHT_SQUARES).popcnt() * light_bishops.popcnt());

    penalty
}

#[cfg(test)]
#[path = "tests/positions.rs"]
mod tests;
