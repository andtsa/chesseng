//! transposition tables!

use std::sync::Arc;
use std::sync::RwLock;

use crate::search::SearchResult;
use crate::setup::depth::Depth;
use crate::transposition_table::vl::VlShare;
use crate::transposition_table::vl::VL;

pub mod empty_table;
pub mod entry;
pub mod vl;

/// The default size of a transposition table, in bytes
pub const DEFAULT_TABLE_SIZE: usize = 256;

/// A key for a transposition table
/// - FromType: the type of the position's identifier (e.g. the board state, or
///   [`chess::Board`] object)
pub trait TKey: Sync + Sized + Copy + Clone {
    /// the type of the position's identifier
    type FromType;

    /// create a new key from the position's identifier
    /// (e.g. the board state, or [`chess::Board`] object)
    fn hash(from: &Self::FromType) -> Self;
    /// a full match between keys, used to determine equality
    /// (slower than [`matches`])
    fn equals(&self, other: &Self) -> bool;
}

/// A transposition table entry
pub trait TEntry: Sync {
    /// the type of the hash used to identify this entry
    type Key: TKey;
    /// the hash-key of this entry
    fn key(&self) -> Self::Key;
    /// create a new empty entry
    fn new_empty() -> Self;
    /// create a new entry to store a search result
    fn new_from_result(hash: u64, depth: Depth, result: &SearchResult, bound: EvalBound) -> Self;
    /// the depth of the search that created this entry
    fn depth(&self) -> Depth;
    /// the relative evaluation of the entry
    fn bound(&self) -> EvalBound;
    /// a [`SearchResult`] from this entry
    fn search_result(&self) -> SearchResult;
    /// do the entry's values make sense?
    fn is_valid(&self) -> bool;
    // ...
}

/// A transposition table of hashes with type `K` and entries of type `E`
pub trait TranspositionTable<Key: TKey, Entry: TEntry> {
    /// create a new transposition table, with a size of `bytes` bytes
    fn new(bytes: usize) -> Self;
    /// resize the transposition table to `bytes` bytes
    fn resize(&mut self, bytes: usize);
    /// get the entry for a hash, if it exists
    fn get(&self, hash: Key) -> Option<Entry>;
    /// insert an entry for a hash.
    ///
    /// intended as a wrapper around [`entry`]
    fn insert(&mut self, hash: Key, entry: Entry);
    /// empty the transposition table
    fn clear(&mut self);
    /// the number of entries in the table
    fn entry_count(&self) -> usize;
    /// capacity in number of entries
    fn capacity(&self) -> usize;
    /// UCI hashfull
    fn hashfull(&self) -> usize;
}

/// A model for concurrent access to the transposition table
pub trait TableAccess<K: TKey, E: TEntry, T: Send + Sync + TranspositionTable<K, E>> {
    /// increment number of successful reads were made in this transposition
    /// table
    fn hit(&self);
    /// access the table. the implementation must ensure that this is
    /// - safe
    /// - fast
    /// - calling &mut self functions on the returned table is safe & updates
    ///   the _same_ table
    fn share(&self) -> Self;
}

/// the type of the currently used transposition table
pub type TableImpl = VL;

/// the type for the currently used thread-sharing implementation
pub type ShareImpl = VlShare;

/// the actual transposition table struct that's passed to the search threads
#[derive(Debug)]
pub struct TT {
    /// the table access struct, defined for whatever the currently used
    /// implementation is
    table: ShareImpl,
}

impl TT {
    /// create a new table access point
    pub fn new() -> Self {
        // let table: EmptyTable<EmptyHash, EmptyEntry> =
        // EmptyTable::new(DEFAULT_TABLE_SIZE);
        Self {
            table: VlShare(Arc::new(RwLock::new(VL::new(DEFAULT_TABLE_SIZE)))),
        }
    }

    /// get a reference to the table
    pub fn get(&self) -> VlShare {
        self.table.share()
    }

    /// get a mutable reference to the table
    pub fn get_mut(&mut self) -> VlShare {
        self.table.share()
    }
}

/// The bound of an evaluation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalBound {
    /// The evaluation is precise
    Exact,
    /// The evaluation is a lower bound
    LowerBound,
    /// The evaluation is an upper bound
    UpperBound,
}
