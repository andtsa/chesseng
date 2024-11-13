pub const ONE_PLY: Depth = Depth(1);
pub const ONE_MOVE: Depth = Depth(2);

/// upper limit for how deeply the engine will search in plies.
pub const MAX_PLY: u16 = 128;

/// maximum depth at which the engine can claim a forced checkmate sequence.
pub const MAX_MATE_PLY: u16 = 128;

/// A struct representing a depth in *plies*
///
/// Each [`Depth`] is a wrapper around an [`u16`] integer, with specific
/// constants for various states.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Depth(pub u16);

impl Depth {
    pub const MAX: Depth = Depth(MAX_PLY / 2);
    pub const ZERO: Depth = Depth(0);
}

impl From<u8> for Depth {
    fn from(d: u8) -> Self {
        Depth(d as u16)
    }
}
