use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::Engine;

pub const MAX_TIME: Duration = Duration::from_secs(60 * 60 * 24 * 365 * 100);

pub const SUBMIT_DURATION: Duration = Duration::from_millis(5);

pub fn max_instant() -> Instant {
    Instant::now() + MAX_TIME
}

impl Engine {
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
