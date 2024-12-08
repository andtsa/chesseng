//! transposition tables!

use crate::search::SearchResult;
use crate::setup::depth::Depth;
use crate::transposition_table::empty_table::EmptyEntry;
use crate::transposition_table::empty_table::EmptyHash;
use crate::transposition_table::empty_table::EmptyTable;

pub mod empty_table;
mod entry;
mod vl;

/// The default size of a transposition table, in kilobytes
pub const DEFAULT_TABLE_SIZE: usize = 64;

/// A key for a transposition table
/// - F: the type of the position's identifier (e.g. the board state, or
///   [`chess::Board`] object)
/// - P: the partial identifier, e.g. [`u16`] for 1/4 match on a 64-bit hash
pub trait TKey: Sync + Sized + Copy + Clone {
    /// the type of the position's identifier
    type FromType;
    /// the type of the partial hash used to identify this key
    type PartialHash;

    /// create a new key from the position's identifier
    /// (e.g. the board state, or [`chess::Board`] object)
    fn hash(from: &Self::FromType) -> Self;
    /// a partial match between keys, used to determine inequality
    fn matches(&self, other: Self::PartialHash) -> bool;
    /// a full match between keys, used to determine equality
    /// (slower than [`matches`])
    fn equals(&self, other: &Self) -> bool;
}

/// A transposition table entry
pub trait TEntry: Sync {
    /// the type of the hash used to identify this entry
    type PartialHash;
    /// the partial hash for this entry
    fn partial_hash(&self) -> Self::PartialHash;
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
    /// create a new transposition table, with a size of `kb` kilobytes
    fn new(kb: usize) -> Self;
    /// resize the transposition table to `kb` kilobytes
    fn resize(&mut self, kb: usize);
    /// get the entry for a hash, if it exists
    fn get(&self, hash: Key) -> Option<&Entry>;
    /// insert an entry for a hash.
    ///
    /// intended as a wrapper around [`entry`]
    fn insert(&mut self, hash: Key, entry: Entry);
    /// get the entry for a hash, creating it if it doesn't exist
    fn entry(&mut self, hash: Key) -> &mut Entry;
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
    fn access(&self) -> T;
}

/// the type of the currently used transposition table
pub type TableImpl = EmptyTable<EmptyHash, EmptyEntry>;

/// the actual transposition table struct that's passed to the search threads
#[derive(Debug)]
pub struct TT {
    /// the table access struct, defined for whatever the currently used
    /// implementation is
    table: TableImpl,
}

impl TT {
    /// create a new table access point
    pub fn new() -> Self {
        let table: EmptyTable<EmptyHash, EmptyEntry> = EmptyTable::new(DEFAULT_TABLE_SIZE);
        Self { table }
    }

    /// get a reference to the table
    pub fn get(&self) -> EmptyTable<EmptyHash, EmptyEntry> {
        self.table.access()
    }

    /// get a mutable reference to the table
    pub fn get_mut(&mut self) -> EmptyTable<EmptyHash, EmptyEntry> {
        self.table.access()
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
