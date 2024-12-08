//! a no-op transposition table
use std::marker::PhantomData;

use crate::setup::depth::Depth;
use crate::transposition_table::EvalBound;
use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TranspositionTable;

/// a no-op entry
#[derive(Debug, Clone)]
pub struct EmptyEntry;
/// a no-op hash
#[derive(Debug, Clone, Copy)]
pub struct EmptyHash;
/// a no-op transposition table
#[derive(Debug, Clone)]
pub struct EmptyTable<K, E> {
    /// the one entry (it's also empty)
    one_entry: E,
    /// the table key type
    _phantom: PhantomData<K>,
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

    fn depth(&self) -> (Depth, Depth) {
        (Depth(0), Depth(0))
    }

    fn bound(&self) -> EvalBound {
        EvalBound::Exact
    }
}

impl<E: TEntry, K: TKey<FromType = (), PartialHash = ()>> TranspositionTable<K, E>
    for EmptyTable<K, E>
{
    fn new(_kb: usize) -> Self {
        EmptyTable {
            one_entry: E::new_empty(),
            _phantom: Default::default(),
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
