//! a no-op transposition table
use std::marker::PhantomData;

use crate::search::SearchResult;
use crate::setup::depth::Depth;
use crate::transposition_table::EvalBound;
use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TableAccess;
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
    /// the table key and value types
    _phantom: PhantomData<(K, E)>,
}

impl TKey for EmptyHash {
    type FromType = ();
    fn hash(_from: &Self::FromType) -> Self {
        EmptyHash
    }
    fn equals(&self, _other: &Self) -> bool {
        false
    }
}

impl TEntry for EmptyEntry {
    type Key = EmptyHash;

    fn key(&self) -> Self::Key {
        EmptyHash
    }

    fn new_empty() -> Self {
        EmptyEntry
    }

    fn new_from_result(
        _hash: u64,
        _depth: Depth,
        _result: &SearchResult,
        _bound: EvalBound,
    ) -> Self {
        EmptyEntry
    }

    fn depth(&self) -> Depth {
        Depth(0)
    }

    fn bound(&self) -> EvalBound {
        EvalBound::Exact
    }

    fn search_result(&self) -> SearchResult {
        SearchResult::default()
    }

    fn is_valid(&self) -> bool {
        false
    }
}

impl<E: TEntry, K: TKey<FromType = ()>> TranspositionTable<K, E> for EmptyTable<K, E> {
    fn new(_kb: usize) -> Self {
        EmptyTable {
            _phantom: Default::default(),
        }
    }
    fn resize(&mut self, _kb: usize) {}
    fn get(&self, _hash: K) -> Option<E> {
        None
    }
    fn insert(&mut self, _hash: K, _entry: E) {}
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

impl TableAccess<EmptyHash, EmptyEntry, EmptyTable<EmptyHash, EmptyEntry>>
    for EmptyTable<EmptyHash, EmptyEntry>
{
    fn hit(&self) {}
    fn share(&self) -> EmptyTable<EmptyHash, EmptyEntry> {
        // the table is always empty, just make a new one
        EmptyTable::new(0)
    }
}
