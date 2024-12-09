//! Display implementation for the [`Value`] struct.
use std::fmt::Display;
use std::fmt::Formatter;

use crate::position::Position;
use crate::setup::values::Value;
use crate::transposition_table::entry::TableEntry;
use crate::transposition_table::TranspositionTable;
use crate::transposition_table::TT;

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

impl Display for TableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}/{:?}/{}/{:?}/Pv {}/V {}",
            self.key,
            self.eval(),
            self.depth(),
            self.mv(),
            self.bound(),
            self.is_pv(),
            self.is_valid_entry()
        )
    }
}

impl Display for TT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.get().0.read() {
            Ok(lock) => {
                writeln!(
                    f,
                    "transposition table: {}/{}.",
                    lock.entry_count(),
                    lock.capacity()
                )?;
                for row in 0..lock.capacity() {
                    writeln!(
                        f,
                        "[({:#03}) -> {}]",
                        row,
                        lock.get(row as u64)
                            .map_or("empty".to_string(), |e| format!("|{e}|"))
                    )?;
                }
                writeln!(f, "---------------")
            }
            Err(e) => {
                writeln!(f, "transposition table [error]: {e}")
            }
        }
    }
}
