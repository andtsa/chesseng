pub mod bitboards;
pub mod material;

use std::ops::Not;

use chess::Board;
use chess::EMPTY;

use crate::evaluation::material::interpolate;
use crate::evaluation::material::material;
use crate::evaluation::position::piece_position_benefit_for_side;
use crate::search::moveordering::MoveOrdering;
use crate::setup::values::Value;
use crate::DebugLevel::Info;
use crate::Opts;

pub const TEMPO: Value = Value(25);

pub type Interp = (f64, f64, f64);

pub fn eval(pos: &Board, moves: &MoveOrdering) -> Value {
    evaluate(pos, moves, Opts::new().eval(Info))
}

pub fn evaluate(pos: &Board, moves: &MoveOrdering, db: Opts) -> Value {
    // Initialize evaluation score
    let mut value = Value::ZERO;
    let stm = pos.side_to_move();

    // Check for mate or stalemate
    if moves.is_empty() {
        return if pos.checkers().eq(&EMPTY) {
            db.edp("eval stalemate");
            // in stalemate, give a slightly negative score to the side that's winning to
            // encourage it to keep playing instead
            value -= material(pos, stm, (0.0, 0.0, 1.0));
            value += material(pos, stm.not(), (0.0, 0.0, 1.0));
            // value is small as to not significantly impact the search tree
            value / 10
        } else {
            // Side to move is checkmated
            db.edp("eval checkmate");
            -Value::MATE // Large negative value
        };
    }

    let interp = interpolate(pos);
    db.edp(&format!("eval interp: {:?}", interp));

    // Calculate material and positional benefits from the side to move's
    // perspective
    value += material(pos, stm, interp);
    value -= material(pos, stm.not(), interp);

    db.edp(&format!("eval after material: {:?}", value));

    value += piece_position_benefit_for_side(pos, stm, interp, db);
    value -= piece_position_benefit_for_side(pos, stm.not(), interp, db);

    db.edp(&format!("eval after both piece_positions: {:?}", value));

    // Add tempo bonus (if desired)
    value += TEMPO; // Always positive for the side to move

    db.edp(&format!("eval after tempo: {:?}", value));

    // Return the evaluation score
    value
}

mod position;
#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
