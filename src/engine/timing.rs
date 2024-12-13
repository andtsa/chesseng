//! chess is a timed game, here we deal with that
use std::cmp::Ordering;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::Engine;

/// one day :)
const MAX_TIME: Duration = Duration::from_secs(60 * 60 * 24);

/// The duration before the search should end to allow for submitting the move.
pub const SUBMIT_DURATION: Duration = Duration::from_millis(5);

/// If the time-remaining difference between the players is greater than this,
/// maybe we should change our time management a bit.
const CONCERN_THRESHOLD: Duration = Duration::from_secs(15);

/// Returns the maximum instant that can be represented.
pub fn max_instant() -> Instant {
    Instant::now() + MAX_TIME
}

impl Engine {
    /// Set the time until which the engine should search.
    pub fn game_time_constraints(
        &mut self,
        white_time: Option<Duration>,
        black_time: Option<Duration>,
        white_increment: Option<Duration>,
        black_increment: Option<Duration>,
        moves_to_go: Option<u8>,
    ) -> Result<()> {
        let (our_time, our_inc, their_time, their_inc) = match self.board.chessboard.side_to_move()
        {
            chess::Color::White => (white_time, white_increment, black_time, black_increment),
            chess::Color::Black => (black_time, black_increment, white_time, white_increment),
        };

        let mut stop_search_at = Instant::now() - SUBMIT_DURATION;

        // estimate how long the game still has to go.
        // TODO: improve this estimate.
        let est_moves_left = moves_to_go.unwrap_or(50).max(1) as u32;

        if let Some(inc) = our_inc {
            // if there is an increment, we should use all of it
            stop_search_at += inc;
        }

        if let Some(time) = our_time {
            // if there is a time constraint, we should use it cautiously
            stop_search_at += time / est_moves_left;
        }

        if let (Some(inc_1), Some(inc_2)) = (our_inc, their_inc) {
            // if there's a difference in increments,
            // we should be more cautious because the opponent
            // will have more time later in the game
            match inc_1.cmp(&inc_2) {
                Ordering::Greater => {
                    // we have an advantage, so we can use a little bit more time
                    stop_search_at += (inc_1 - inc_2) / est_moves_left;
                }
                Ordering::Less => {
                    // opponent has the advantage, use a little less time in this case
                    stop_search_at -= (inc_2 - inc_1) / est_moves_left;
                }
                Ordering::Equal => {
                    // do nothing, the timing is fair :)
                }
            }
        }

        if let (Some(t1), Some(t2)) = (our_time, their_time) {
            // we can take a hint from our opponent, and play more naturally
            // closer to their pace. if the opponent is playing
            // quickly, we should speed up (as is more natural in human games)
            match t1.cmp(&t2) {
                Ordering::Less => {
                    if t2 - t1 > CONCERN_THRESHOLD / 2 {
                        // maybe we should think faster
                        stop_search_at -= (t2 - t1) / est_moves_left;
                    }
                }
                Ordering::Equal => {
                    // the game is flowing evenly :)
                    stop_search_at -= SUBMIT_DURATION;
                    // tiny change to get tiny advantage.
                }
                Ordering::Greater => {
                    // we seem to have more time left than the opponent.
                    // is this difference significant?
                    if t1 - t2 > CONCERN_THRESHOLD {
                        // maybe we should think longer about our moves
                        stop_search_at += (t1 - t2) / ((est_moves_left / 4).max(1));
                    }
                }
            }
        }

        if let Some(time) = our_time {
            debug_assert!(stop_search_at - Instant::now() < time);
            stop_search_at = stop_search_at.min(Instant::now() + time);
        }

        self.set_search_until(stop_search_at)
    }
}
