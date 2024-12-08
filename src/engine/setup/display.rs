//! Display implementation for the [`Value`] struct.
use std::fmt::Display;
use std::fmt::Formatter;

use crate::position::Position;
use crate::setup::values::Value;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self == Value::MATE {
            write!(f, "mate")
        } else if *self == Value::MIN {
            write!(f, "-inf")
        } else if *self == Value::MAX {
            write!(f, "inf")
        } else {
            write!(f, "cp {}", self.0)
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chessboard)
    }
}
