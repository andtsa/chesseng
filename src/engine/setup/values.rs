//! Code in this file is copied/modified from https://github.com/syzygy1/Rustfish/
//!
//! Licensed under GNU General Public License v3.0
//!
//! Published exclusively to https://github.com/andtsa/chesseng

use chess::Color;

use crate::setup::depth::MAX_MATE_PLY;
use crate::setup::depth::MAX_PLY;

/// A struct representing a *centipawn* value in the evaluation function, which
/// can be used to assign scores to positions.
///
/// Each [`Value`] is a wrapper around an [`i16`] integer, with specific
/// constants for various evaluation states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub i16);

impl Value {
    /// minimum legal value
    pub const MIN: Value = Value(-32_600);
    /// maximum legal value
    pub const MAX: Value = Value(32_600);

    /// positions without advantage for either side.
    pub const ZERO: Value = Value(0);

    pub const ONE: Value = Value(1);

    /// represents a draw score, similar to [`ZERO`].
    pub const DRAW: Value = Value(0);

    /// represents a known winning score for the side to move.
    pub const KNOWN_WIN: Value = Value(10_000);

    /// checkmate, highest legal value in the evaluation function
    pub const MATE: Value = Value(32_000);

    /// a value higher than [`MATE`], used to represent an infinite score.
    pub const INFINITE: Value = Value(32_001);

    /// undefined, used to represent an invalid score.
    pub const NONE: Value = Value(32_002);

    /// The score for when the engine believes it can checkmate within the depth
    /// limit of [`MAX_PLY`].
    ///
    /// This is slightly less than [`MATE`], adjusted by [`MAX_MATE_PLY`] and
    /// [`MAX_PLY`] to prioritize closer checkmates.
    pub const MATE_IN_MAX_PLY: Value = Value(Value::MATE.0 - MAX_MATE_PLY as i16 - MAX_PLY as i16);

    /// The score for when the engine expects to be checkmated within
    /// [`MAX_PLY`], meaning it evaluates the position as lost.
    pub const MATED_IN_MAX_PLY: Value =
        Value(-Value::MATE.0 + MAX_MATE_PLY as i16 + MAX_PLY as i16);

    /// Returns the absolute value of a [`Value`].
    pub fn abs(self) -> Value {
        Value(self.0.abs())
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        match value {
            Color::White => Value(1),
            Color::Black => Value(-1),
        }
    }
}
