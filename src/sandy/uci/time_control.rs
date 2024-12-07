use std::time::Instant;

use anyhow::Result;
use sandy_engine::timing::max_instant;
use sandy_engine::transposition_table::TEntry;
use sandy_engine::transposition_table::TKey;
use sandy_engine::transposition_table::TranspositionTable;
use sandy_engine::Engine;
use vampirc_uci::UciTimeControl;

/// Implement this trait for the [`Engine`] to handle time control.
pub trait TimeControl {
    /// Convert a [`UciTimeControl`] into actual timing values for the
    /// [`Engine`].
    fn time_control(&mut self, tc: UciTimeControl) -> Result<()>;
}

impl<K: TKey, E: TEntry, TT: TranspositionTable<K, E>> TimeControl for Engine<K, E, TT> {
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
            UciTimeControl::MoveTime(d) => {
                self.set_search_until(Instant::now() + d.to_std().unwrap())?
            }
        }
        Ok(())
    }
}

/// chrono_duration_to_std_time
fn cdt(d: chrono::Duration) -> std::time::Duration {
    std::time::Duration::new(
        d.num_seconds() as u64,
        d.num_microseconds().unwrap_or(0) as u32,
    )
}
