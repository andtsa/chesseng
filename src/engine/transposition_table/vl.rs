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
        } else {
            self.table.resize(size, TableEntry::new_empty());
        }

        optlog!(tt;info;"resized VL table from {} to {} entries.", self.size, size);

        self.size = size;
        size
    }

    fn get(&self, hash: u64) -> Option<TableEntry> {
        let idx = (hash as usize) % self.size;
        let entry = self.table.get(idx);
        if entry.is_some_and(|e| hash.equals(&e.key())) {
            entry.cloned()
        } else {
            None
        }
    }

    fn insert(&mut self, hash: u64, entry: TableEntry) {
        let idx = (hash as usize) % self.size;
        if !self.table[idx].is_valid_entry() {
            self.occupied += 1;
        }
        self.table[idx] = entry;
        debug_assert!(self.table[idx].is_valid_entry());
    }

    fn clear(&mut self) {
        self.table.clear();
        self.occupied = 0;
    }

    fn entry_count(&self) -> usize {
        self.occupied
    }

    fn capacity(&self) -> usize {
        self.size
    }

    fn hashfull(&self) -> usize {
        self.occupied * 1000 / self.size
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
