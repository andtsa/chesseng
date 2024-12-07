//! transposition tables!

pub mod empty_table;

/// The default size of a transposition table, in kilobytes
pub const DEFAULT_TABLE_SIZE: usize = 64;

/// A key for a transposition table
/// - F: the type of the position's identifier (e.g. the board state, or
///   [`chess::Board`] object)
/// - P: the partial identifier, e.g. [`u16`] for 1/4 match on a 64-bit hash
pub trait TKey: Sized {
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
pub trait TEntry {
    /// the type of the hash used to identify this entry
    type PartialHash;
    /// the partial hash for this entry
    fn partial_hash(&self) -> Self::PartialHash;
    /// create a new empty entry
    fn new_empty() -> Self;
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
