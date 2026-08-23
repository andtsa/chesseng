use std::time::Instant;

use anyhow::Result;
use sandy_engine::Engine;
use sandy_engine::timing::max_instant;
use vampirc_uci::UciTimeControl;

/// Implement this trait for the [`Engine`] to handle time control.
pub trait TimeControl {
    /// Convert a [`UciTimeControl`] into actual timing values for the
    /// [`Engine`].
    fn time_control(&mut self, tc: UciTimeControl) -> Result<()>;
}

impl TimeControl for Engine {
    fn time_control(&mut self, tc: UciTimeControl) -> Result<()> {
        match tc {
            UciTimeControl::Ponder => unimplemented!("ponder not yet implemented"),
            UciTimeControl::Infinite => self.set_search_until(max_instant())?,
            UciTimeControl::TimeLeft {
                white_time,      // Option<Duration>,
                black_time,      // Option<Duration>,
                white_increment, // Option<Duration>,
                black_increment, // Option<Duration>,
                moves_to_go,     // Option<u8>,
            } => {
                self.game_time_constraints(
                    white_time.map(cdt),
                    black_time.map(cdt),
                    white_increment.map(cdt),
                    black_increment.map(cdt),
                    moves_to_go,
                )?;
            }
            UciTimeControl::MoveTime(d) => self.set_search_until(Instant::now() + d.to_std()?)?,
        }
        Ok(())
    }
}

/// chrono_duration_to_std_time
pub(crate) fn cdt(d: chrono::Duration) -> std::time::Duration {
    // a player may be reported with a negative clock,
    // which has no std equivalent
    d.to_std().unwrap_or(std::time::Duration::ZERO)
}

#[cfg(test)]
mod tests {
    use super::cdt;

    #[test]
    fn matches_chrono() {
        for ms in [0i64, 1, 50, 500, 999, 1_500, 10_000, 60_000, 300_000] {
            let d = chrono::Duration::milliseconds(ms);
            assert_eq!(cdt(d), d.to_std().unwrap(), "{ms}ms converted wrong");
            assert_eq!(cdt(d).as_millis(), ms as u128, "{ms}ms not reflexive");
        }
    }

    #[test]
    fn negative_clocks_are_no_time_left() {
        let flagged = chrono::Duration::milliseconds(-500);
        assert_eq!(cdt(flagged), std::time::Duration::ZERO);
    }
}
