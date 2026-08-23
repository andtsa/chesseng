//! a lock-based Vec transposition table

use std::sync::Arc;
use std::sync::RwLock;

use chess::Board;

use crate::optlog;
use crate::transposition_table::TEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TableAccess;
use crate::transposition_table::TranspositionTable;
use crate::transposition_table::entry::TableEntry;

/// a lock-based Vec transposition table
#[derive(Debug)]
pub struct VL {
    /// the table
    table: Vec<TableEntry>,
    /// the number of entries in the table
    size: usize,
    /// number of valid entries in the table
    occupied: usize,
}

impl TranspositionTable<u64, TableEntry> for VL {
    fn new(bytes: usize) -> Self {
        // the number of entries must be a power of 2
        let size = (bytes / size_of::<TableEntry>())
            .next_power_of_two()
            .checked_shr(1)
            .unwrap_or_default();
        optlog!(tt;info;"created VL table with {size} entries.");
        let table = vec![TableEntry::new_empty(); size];
        Self {
            table,
            size,
            occupied: 0,
        }
    }

    fn resize(&mut self, bytes: usize) -> usize {
        let size = (bytes / size_of::<TableEntry>())
            .next_power_of_two()
            .checked_shr(1)
            .unwrap_or_default();

        if size == self.size {
            return size;
        }

        // entries are indexed modulo the table size, so none of them are
        // addressable any more. start from an empty table rather than leaving
        // stale entries and a stale `occupied` count behind.
        self.table = vec![TableEntry::new_empty(); size];
        self.occupied = 0;

        optlog!(tt;info;"resized VL table from {} to {} entries.", self.size, size);

        self.size = size;
        size
    }

    fn get(&self, hash: u64) -> Option<TableEntry> {
        if self.size == 0 {
            return None;
        }
        let idx = (hash as usize) % self.size;
        let entry = self.table.get(idx);
        if entry.is_some_and(|e| hash.equals(&e.key())) {
            entry.cloned()
        } else {
            None
        }
    }

    fn insert(&mut self, hash: u64, entry: TableEntry) {
        if self.size == 0 {
            return;
        }
        let idx = (hash as usize) % self.size;
        if !self.table[idx].is_valid_entry() {
            self.occupied += 1;
        }
        self.table[idx] = entry;
        debug_assert!(self.table[idx].is_valid_entry());
    }

    fn clear(&mut self) {
        // the entries are emptied in place: truncating the vec would leave
        // `size` pointing past its end, and the next insert would panic.
        self.table
            .iter_mut()
            .for_each(|e| *e = TableEntry::new_empty());
        self.occupied = 0;
    }

    fn entry_count(&self) -> usize {
        self.occupied
    }

    fn capacity(&self) -> usize {
        self.size
    }

    fn hashfull(&self) -> usize {
        (self.occupied * 1000)
            .checked_div(self.size)
            .unwrap_or_default()
    }
}

impl TKey for u64 {
    type FromType = Board;

    fn hash(from: &Self::FromType) -> Self {
        from.get_hash()
    }
    fn equals(&self, other: &Self) -> bool {
        *self == *other
    }
}

/// a shared reference to a VL transposition table
pub type VlShare = Arc<RwLock<VL>>;

impl TableAccess<u64, TableEntry, VL> for VlShare {
    fn hit(&self) {
        // to-do
        // currently hit counts are accumulated in the search functionality as
        // TB_HITS
    }

    fn share(&self) -> VlShare {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use chess::Board;

    use super::*;
    use crate::search::SearchResult;
    use crate::setup::depth::ONE_PLY;
    use crate::setup::values::Value;
    use crate::transposition_table::EvalBound;

    /// build a table with a handful of entries in it
    fn populated() -> VL {
        let mut table = VL::new(1 << 16);
        let result = SearchResult {
            pv: vec![crate::search::MV(chess::ChessMove::default(), Value::ONE)],
            next_position_value: Value::ONE,
            nodes_searched: 1,
            tb_hits: 0,
            depth: ONE_PLY,
            from_draw: false,
            aborted: false,
        };
        for hash in 1..8u64 {
            table.insert(
                hash,
                TableEntry::new_from_result(hash, ONE_PLY, &result, EvalBound::Exact),
            );
        }
        table
    }

    /// clearing must leave a table that can still be written to. truncating
    /// the vec instead of emptying the entries left `size` pointing past the
    /// end, so the next insert panicked.
    #[test]
    fn clear_leaves_the_table_usable() {
        let mut table = populated();
        let capacity = table.capacity();

        table.clear();

        assert_eq!(table.entry_count(), 0);
        assert_eq!(table.capacity(), capacity);
        assert!(table.get(3).is_none(), "cleared entry was still readable");

        // this panicked before the fix
        let result = SearchResult {
            pv: vec![crate::search::MV(chess::ChessMove::default(), Value::ONE)],
            next_position_value: Value::ONE,
            nodes_searched: 1,
            tb_hits: 0,
            depth: ONE_PLY,
            from_draw: false,
            aborted: false,
        };
        table.insert(
            3,
            TableEntry::new_from_result(3, ONE_PLY, &result, EvalBound::Exact),
        );
        assert!(table.get(3).is_some());
    }

    /// after shrinking, the occupancy count must not describe the old table
    #[test]
    fn resize_resets_occupancy() {
        let mut table = populated();
        assert!(table.entry_count() > 0);

        table.resize(1 << 12);

        assert!(
            table.entry_count() <= table.capacity(),
            "{} entries in a table of {}",
            table.entry_count(),
            table.capacity()
        );
        assert!(table.hashfull() <= 1000);
    }

    /// a hash that isn't in the table must not return another entry's result
    #[test]
    fn get_misses_are_none() {
        let table = populated();
        assert!(table.get(u64::MAX).is_none());
        let _ = Board::default();
    }
}
