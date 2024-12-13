use std::time::Instant;

use anyhow::Result;
use sandy_engine::timing::max_instant;
use sandy_engine::Engine;
use vampirc_uci::UciTimeControl;

/// Implement this trait for the [`Engine`] to handle time control.
pub trait TimeControl {
    /// Convert a [`UciTimeControl`] into actual timing values for the
    /// [`Engine`].
    fn time_control(&mut self, tc: UciTimeControl) -> Result<()>;
}

impl TimeControl for Engine {
    fn time_control(&mut self, tc: UciTimeControl) -> Result<()> {
        // don't ponder unless set true in the next match statement. not optimal but
        // still O(1) :)
        match tc {
            UciTimeControl::Ponder => self.set_ponder(true),
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
fn cdt(d: chrono::Duration) -> std::time::Duration {
    std::time::Duration::new(
        d.num_seconds().max(0).unsigned_abs(),
        d.num_nanoseconds()
            .unwrap_or(0)
            .max(0)
            .unsigned_abs()
            .max(1_000_000_000 - 1) as u32,
    )
}
