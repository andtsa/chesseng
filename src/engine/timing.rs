//! chess is a timed game, here we deal with that
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TranspositionTable;
use crate::Engine;

/// one hundred years :)
const MAX_TIME: Duration = Duration::from_secs(60 * 60 * 24 * 365 * 100);

/// The duration before the search should end to allow for submitting the move.
pub const SUBMIT_DURATION: Duration = Duration::from_millis(5);

/// Returns the maximum instant that can be represented.
pub fn max_instant() -> Instant {
    Instant::now() + MAX_TIME
}

impl<K: TKey, E: TEntry, TT: TranspositionTable<K, E>> Engine<K, E, TT> {
    /// Set the time until which the engine should search.
    pub fn game_time_constraints(
        &mut self,
        white_time: Option<Duration>,
        black_time: Option<Duration>,
        white_increment: Option<Duration>,
        black_increment: Option<Duration>,
        moves_to_go: Option<u8>,
    ) -> Result<()> {
        let (time, increment) = match self.board.side_to_move() {
            chess::Color::White => (white_time, white_increment),
            chess::Color::Black => (black_time, black_increment),
        };
        match (time, increment, moves_to_go) {
            (Some(t), Some(i), x) => self.set_search_until(
                Instant::now() + i - SUBMIT_DURATION + (t / x.unwrap_or(50) as u32),
            ),
            (Some(t), None, x) => {
                self.set_search_until(Instant::now() + t - SUBMIT_DURATION / x.unwrap_or(50) as u32)
            }
            (None, Some(i), _) => self.set_search_until(Instant::now() + i - SUBMIT_DURATION),
            (None, None, _) => self.set_search_until(Instant::now() - SUBMIT_DURATION),
        }
    }
}
