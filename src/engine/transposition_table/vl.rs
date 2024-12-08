//! a lock-based Vec transposition table

use std::sync::RwLock;

use chess::Board;

use crate::transposition_table::entry::TableEntry;
use crate::transposition_table::TKey;
use crate::transposition_table::TranspositionTable;

/// a lock-based Vec transposition table
pub struct VL {
    /// the table
    table: RwLock<Vec<TableEntry>>,
    /// the number of entries in the table
    size: usize,
    /// number of valid entries in the table
    occupied: usize,
}

impl TranspositionTable<u64, TableEntry> for VL {
    fn new(kb: usize) -> Self {
        // the number of entries must be a power of 2
        let size = (kb * 1024 / size_of::<TableEntry>()).next_power_of_two();
        let mut table = vec![];
        table.reserve_exact(size);
        Self {
            table: RwLock::new(table),
            size,
            occupied: 0,
        }
    }

    fn resize(&mut self, kb: usize) {
        let size = (kb * 1024 / size_of::<TableEntry>()).next_power_of_two();
        let mut table = vec![];
        table.reserve_exact(size);
        self.table = RwLock::new(table);
        self.size = size;
    }

    fn get(&self, hash: u64) -> Option<TableEntry> {
        let idx = (hash as usize) % self.size;
        let table = self.table.read().unwrap();
        let entry = table.get(idx);
        entry.cloned()
    }

    fn insert(&mut self, hash: u64, entry: TableEntry) {
        let idx = (hash as usize) % self.size;
        let mut table = self.table.write().unwrap();
        if !table[idx].is_valid_entry() {
            self.occupied += 1;
        }
        table[idx] = entry;
        debug_assert!(table[idx].is_valid_entry());
    }

    fn clear(&mut self) {
        let mut table = self.table.write().unwrap();
        table.clear();
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
    type PartialHash = u16;

    fn hash(from: &Self::FromType) -> Self {
        from.get_hash()
    }

    fn matches(&self, other: Self::PartialHash) -> bool {
        *self as u16 == other
    }

    fn equals(&self, other: &Self) -> bool {
        *self == *other
    }
}
