//! a no-op transposition table
use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TranspositionTable;

/// a no-op entry
#[derive(Debug)]
pub struct EmptyEntry;
/// a no-op hash
#[derive(Debug)]
pub struct EmptyHash;
/// a no-op transposition table
#[derive(Debug)]
pub struct EmptyTable<E> {
    /// the one entry (it's also empty)
    one_entry: E,
}

impl TKey for EmptyHash {
    type FromType = ();
    type PartialHash = ();

    fn hash(_from: &Self::FromType) -> Self {
        EmptyHash
    }

    fn matches(&self, _other: Self::PartialHash) -> bool {
        false
    }

    fn equals(&self, _other: &Self) -> bool {
        false
    }
}

impl TEntry for EmptyEntry {
    type PartialHash = ();

    fn partial_hash(&self) -> Self::PartialHash {}
    fn new_empty() -> Self {
        EmptyEntry
    }
}

impl<E: TEntry, K: TKey<FromType = (), PartialHash = ()>> TranspositionTable<K, E>
    for EmptyTable<E>
{
    fn new(_kb: usize) -> Self {
        EmptyTable {
            one_entry: E::new_empty(),
        }
    }

    fn resize(&mut self, _kb: usize) {}

    fn get(&self, _hash: K) -> Option<&E> {
        None
    }

    fn insert(&mut self, _hash: K, _entry: E) {}

    fn entry(&mut self, _hash: K) -> &mut E {
        &mut self.one_entry
    }

    fn clear(&mut self) {}

    fn entry_count(&self) -> usize {
        1
    }

    fn capacity(&self) -> usize {
        1
    }

    fn hashfull(&self) -> usize {
        0
    }
}
