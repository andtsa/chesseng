use std::fmt::Display;
use std::fmt::Formatter;

use crate::setup::depth::MAX_PLY;
use crate::setup::values::Value;

impl Display for Value {
    /// value() converts a Value to a string suitable for use with the UCI
    /// protocol specification:
    ///
    /// * `cp <x>` : The score from the engine's point of view in centipawns
    /// * `mate <y>` : Mate in y moves, not plies. If the engine is getting
    ///   mated, use negative values for y.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = if self >= &Value::ZERO {
            *self
        } else {
            -(*self)
        };
        if w < Value::MATE - Value::from(MAX_PLY) {
            // no one is getting mated within MAX_PLY plies
            Ok(write!(f, "cp {}", w.0)?)
        } else {
            write!(f, "mate ")?;
            let mut dtm = if self > &Value::ZERO {
                (Value::MATE - *self).0 + 1
            } else {
                (-Value::MATE - *self).0
            };
            dtm /= 2;
            write!(f, "{}", &dtm.to_string())
        }
    }
}
