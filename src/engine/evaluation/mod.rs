pub mod bitboards;
pub mod material;

use chess::Board;
use chess::EMPTY;

use crate::evaluation::material::material;
use crate::search::moveordering::MoveOrdering;
use crate::setup::values::Value;

pub fn evaluate(pos: &Board, moves: &MoveOrdering) -> Value {
    let mut value = Value::ZERO;
    let mut mult = Value::from(pos.side_to_move());

    // check for mate
    if moves.is_empty() {
        if pos.checkers().eq(&EMPTY) {
            // stalemate
            mult = -mult;
        } else {
            // checkmate
            return mult * Value::MATE;
        };
    }

    value += material(pos, pos.side_to_move());
    value -= material(pos, !pos.side_to_move());

    value * mult
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
