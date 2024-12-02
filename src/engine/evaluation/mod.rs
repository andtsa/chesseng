//! The Evaluation module of this chess engine

pub mod bitboards;
pub mod material;

use std::ops::Not;

use anyhow::Result;
use chess::Board;
use chess::EMPTY;

use crate::evaluation::material::interpolate;
use crate::evaluation::material::material;
use crate::evaluation::position::piece_position_benefit_for_side;
use crate::optlog;
use crate::opts::setopts;
use crate::opts::Opts;
use crate::search::moveordering::MoveOrdering;
use crate::setup::values::Value;

/// a bonus given to the side-to-move for having a tempo advantage
pub const TEMPO: Value = Value(25);

/// an interpolation between beginning, middle, and endgame
pub type Interp = (f64, f64, f64);

/// same as [`evaluate`] but first sets the global [`Opts`]
pub fn eval(pos: &Board, moves: &MoveOrdering, opts: Opts) -> Result<Value> {
    {
        setopts(opts)?;
    }
    Ok(evaluate(pos, moves))
}

/// the main evaluation function. returns a value representing the score of the
/// position from the point of view of the player whos turn it is to move
pub fn evaluate(pos: &Board, moves: &MoveOrdering) -> Value {
    // Initialize evaluation score
    let mut value = Value::ZERO;
    let stm = pos.side_to_move();

    // Check for mate or stalemate
    if moves.is_empty() {
        return if pos.checkers().eq(&EMPTY) {
            optlog!(eval;debug;"eval stalemate");
            // in stalemate, give a slightly negative score to the side that's winning to
            // encourage it to keep playing instead
            value -= material(pos, stm, (0.0, 0.0, 1.0));
            value += material(pos, stm.not(), (0.0, 0.0, 1.0));
            // value is small as to not significantly impact the search tree
            value / 10
        } else {
            // Side to move is checkmated
            optlog!(eval;debug;"eval checkmate");
            -Value::MATE // Large negative value
        };
    }

    let interp = interpolate(pos);

    // Calculate material and positional benefits from the side to move's
    // perspective
    value += material(pos, stm, interp);
    value -= material(pos, stm.not(), interp);

    value += piece_position_benefit_for_side(pos, stm, interp);
    value -= piece_position_benefit_for_side(pos, stm.not(), interp);

    // Add tempo bonus
    value += TEMPO; // Always positive for the side to move

    // more moves is a bit better
    // value += Value(moves.len() as i16);

    // Return the evaluation score
    value
}

mod position;
#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
