use anyhow::Result;
use sandy_engine::transposition_table::TEntry;
use sandy_engine::transposition_table::TKey;
use sandy_engine::transposition_table::TranspositionTable;
use sandy_engine::Engine;
use vampirc_uci::UciSearchControl;

/// Implement this trait for the [`Engine`] to handle search control.
pub trait SearchControl {
    /// Convert a [`UciSearchControl`] into actual search values for the
    /// [`Engine`].
    fn search_control(&mut self, tc: UciSearchControl) -> Result<()>;
}

impl<K: TKey, E: TEntry, TT: TranspositionTable<K, E>> SearchControl for Engine<K, E, TT> {
    fn search_control(&mut self, tc: UciSearchControl) -> Result<()> {
        if !tc.search_moves.is_empty() {
            unimplemented!("search moves not yet implemented");
        }
        if let Some(depth) = tc.depth {
            self.set_search_to(depth.into());
        }
        if let Some(_mate) = tc.mate {
            unimplemented!("mate not yet implemented");
        }
        if let Some(_nodes) = tc.nodes {
            unimplemented!("nodes not yet implemented");
        }
        Ok(())
    }
}
