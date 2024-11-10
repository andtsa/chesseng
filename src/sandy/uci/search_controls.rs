use anyhow::Result;
use sandy_engine::Engine;
use vampirc_uci::UciSearchControl;

pub trait SearchControl {
    fn search_control(&mut self, tc: UciSearchControl) -> Result<()>;
}

impl SearchControl for Engine {
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
