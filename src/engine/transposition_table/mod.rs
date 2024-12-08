//! transposition tables!

use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::setup::depth::Depth;
use crate::transposition_table::empty_table::EmptyEntry;
use crate::transposition_table::empty_table::EmptyHash;
use crate::transposition_table::empty_table::EmptyTable;

pub mod empty_table;

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
pub trait TEntry: Sync + Clone {
    /// the type of the hash used to identify this entry
    type PartialHash;
    /// the partial hash for this entry
    fn partial_hash(&self) -> Self::PartialHash;
    /// create a new empty entry
    fn new_empty() -> Self;
    /// the depth of the entry
    /// - .0: the move from which the search started
    /// - .1: the depth of the search that created this entry
    fn depth(&self) -> (Depth, Depth);
    /// the relative evaluation of the entry
    fn bound(&self) -> EvalBound;
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
#[derive(Debug)]
pub struct TableAccess<K: TKey, E: TEntry, T: Send + Sync + TranspositionTable<K, E>> {
    /// how many successful reads were made in this transposition table
    pub hits: AtomicUsize,
    /// the actual table. must be [`Sync`]
    pub table: Arc<T>,
    /// phantom data for the key and entry types
    _phantom: PhantomData<(K, E)>,
}

// // SAFETY: to-do
// unsafe impl Send for TableAccess {}
// // SAFETY: idk
// unsafe impl Sync for TableAccess {}

/// the type of the currently used transposition table
pub type TableImpl = EmptyTable<EmptyHash, EmptyEntry>;

/// the actual transposition table struct that's passed to the search threads
#[derive(Debug)]
pub struct TT {
    /// the table access struct, defined for whatever the currently used
    /// implementation is
    table: TableAccess<EmptyHash, EmptyEntry, TableImpl>,
}

impl TT {
    /// create a new table access point
    pub fn new() -> Self {
        let table: EmptyTable<EmptyHash, EmptyEntry> = EmptyTable::new(DEFAULT_TABLE_SIZE);
        Self {
            table: TableAccess {
                hits: AtomicUsize::new(0),
                table: Arc::new(table),
                _phantom: Default::default(),
            },
        }
    }

    /// get a reference to the table's arc
    pub fn get_arc(&self) -> Arc<EmptyTable<EmptyHash, EmptyEntry>> {
        self.table.table.clone()
    }

    /// get a reference to the table
    pub fn get(&self) -> &EmptyTable<EmptyHash, EmptyEntry> {
        &self.table.table
    }

    /// get a mutable reference to the table
    pub fn get_mut(&mut self) -> &mut EmptyTable<EmptyHash, EmptyEntry> {
        Arc::make_mut(&mut self.table.table)
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
