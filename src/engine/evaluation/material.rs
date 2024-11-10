use std::ops::BitAnd;

use chess::Board;
use chess::Color;
use chess::Piece;

use crate::setup::values::Value;

pub const MIDDLEGAME_SCORE: Value = Value(6666);
pub const ENDGAME_SCORE: Value = Value(3333);
pub const START_COUNT: Value = Value(8810);

pub const PIECE_TYPES: [Piece; 5] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];

pub const INITIAL_VALUES: [Value; 5] = [
    Value(100), // Pawn
    Value(280), // Knight
    Value(310), // Bishop
    Value(500), // Rook
    Value(900), // Queen
];

pub const MIDGAME_VALUES: [Value; 5] = [
    Value(95),  // Pawn
    Value(310), // Knight
    Value(300), // Bishop
    Value(500), // Rook
    Value(900), // Queen
];

pub const ENDGAME_VALUES: [Value; 5] = [
    Value(240),  // Pawn
    Value(200),  // Knight
    Value(300),  // Bishop
    Value(600),  // Rook
    Value(1050), // Queen
];

pub fn material(board: &Board, side: Color) -> Value {
    let mut value = Value::ZERO;

    let (early, mid, end) = interp(board);

    for (idx, piece) in PIECE_TYPES.iter().enumerate() {
        let count = board
            .pieces(*piece)
            .bitand(board.color_combined(side))
            .popcnt();
        value += (INITIAL_VALUES[idx] * Value::from(count)) * early;
        value += (MIDGAME_VALUES[idx] * Value::from(count)) * mid;
        value += (ENDGAME_VALUES[idx] * Value::from(count)) * end;
    }

    value
}

#[allow(dead_code)]
pub fn material_count_for_side(pos: &Board, color: Color) -> Value {
    let side = pos.color_combined(color);
    let mut value = Value::ZERO;

    for (idx, piece) in PIECE_TYPES.iter().enumerate() {
        let count = pos.pieces(*piece).bitand(side).popcnt();
        value += INITIAL_VALUES[idx] * Value::from(count);
    }

    value
}

pub fn total_material(pos: &Board) -> Value {
    let mut value = Value::ZERO;

    for (idx, piece) in PIECE_TYPES.iter().enumerate() {
        let count = pos.pieces(*piece).popcnt();
        value += INITIAL_VALUES[idx] * count;
    }

    value
}

/// Interpolate between [`GAME_PHASE`] values based on the number of pieces on
/// the board.
///
/// Returns a multiplier for each (EARLY_VALUES, MG_VALUES, EG_VALUES).
pub fn interp(pos: &Board) -> (f64, f64, f64) {
    let total_value = total_material(pos).0 as f64;
    // Calculate the midpoint between middlegame and endgame thresholds
    let midpoint = (MIDDLEGAME_SCORE + ENDGAME_SCORE).0 as f64 / 2.0;

    // Early Game Coefficient
    let early_game_coeff = if total_value >= MIDDLEGAME_SCORE.0 as f64 {
        1.0
    } else if total_value <= midpoint {
        0.0
    } else {
        (total_value - midpoint) / (MIDDLEGAME_SCORE.0 as f64 - midpoint)
    };

    // Endgame Coefficient
    let endgame_coeff = if total_value <= ENDGAME_SCORE.0 as f64 {
        1.0
    } else if total_value >= midpoint {
        0.0
    } else {
        (midpoint - total_value) / (midpoint - ENDGAME_SCORE.0 as f64)
    };

    // Middle Game Coefficient
    let middle_game_coeff = 1.0 - early_game_coeff - endgame_coeff;

    (early_game_coeff, middle_game_coeff, endgame_coeff)
}
