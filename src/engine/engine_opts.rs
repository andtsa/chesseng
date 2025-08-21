//! options specific to the current engine

/// Options for the engine's execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EngineOpts {
    /// should the search use alpha beta pruning?
    pub use_ab: bool,
    /// should the search use principal variation search?
    pub use_pv: bool,
    /// should the search use transposition tables?
    pub use_tt: bool,
    /// should the search use move ordering?
    pub use_mo: bool,
    /// should the engine ponder?
    pub ponder: bool,
    /// how big should the transposition table be? value in **bytes**
    pub hash_size: usize,
    /// how many threads should the search use?
    pub threads: usize,
}
