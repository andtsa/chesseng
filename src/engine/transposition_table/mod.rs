pub mod bounds;

use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::RwLock;

use anyhow::anyhow;
use anyhow::Result;
use chess::ChessMove;
use chess::Piece;
use chess::Square;

use crate::opts::opts;
use crate::search::MV;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::transposition_table::bounds::EvalBound;

/// # A single transposition table entry.
/// [`TranspositionTable`]
/// ## Memory Layout
/// * 2 bytes for the key
///
/// **[`AtomicU64`]:**
/// * evaluation value: 2 bytes
/// * depth: 2 bytes
/// * source square: 1 byte
/// * destination square: 1 byte
/// * promotion: 3 bits
/// * bound: 2 bits
/// * is_pv: 1 bit
/// * is_valid_entry: 1 bit
/// * 1 parity bit for checking
#[derive(Debug)]
pub struct TableEntry {
    /// only store 2 bytes of the key to verify if a collision occurred
    pub key: u16,
    pub value: AtomicU64,
}

pub struct TranspositionTable {
    size: usize,
    used: usize,
    table: Vec<TableEntry>,
}

pub static TT_INITIALISED: AtomicBool = AtomicBool::new(false);
pub static mut TT: MaybeUninit<RwLock<TranspositionTable>> = MaybeUninit::uninit();

pub fn table_mut_ref() -> &'static mut RwLock<TranspositionTable> {
    if !TT_INITIALISED.load(Ordering::Relaxed) {
        panic!("transposition table not initialised, asked for mut ref");
    }
    unsafe { TT.assume_init_mut() }
}

pub fn table_ref() -> &'static RwLock<TranspositionTable> {
    if !TT_INITIALISED.load(Ordering::Relaxed) {
        panic!("transposition table not initialised, asked for mut ref");
    }
    unsafe { TT.assume_init_ref() }
}

impl TranspositionTable {
    pub fn new() -> Result<Self> {
        let size = (opts()?.hash_size * 1024 / size_of::<TableEntry>())
            .checked_next_power_of_two()
            .ok_or(anyhow!("invalid hash map size (overflowed)"))?;
        let mut table = Vec::with_capacity(size);
        for _ in 0..size {
            table.push(TableEntry {
                key: 0,
                value: AtomicU64::new(0),
            });
        }
        Ok(Self {
            table,
            size,
            used: 0,
        })
    }

    pub fn index(&self, key: u64) -> usize {
        (key as usize) % self.size
    }

    pub fn lookup(&self, key: u64) -> Option<&TableEntry> {
        let index = self.index(key);
        let slot = &self.table[index];

        if slot.key == (key & 0xFFFF) as u16 {
            Some(slot)
        } else {
            None
        }
    }

    pub(crate) fn insert(&mut self, key: u64, new_entry: TableEntry) {
        let index = self.index(key);
        let slot = &mut self.table[index];

        let existing_depth = slot.depth();

        let new_depth = new_entry.depth();

        if new_depth >= existing_depth {
            if !slot.is_valid_entry() {
                self.used += 1;
            }
            slot.key = (key & 0xFFFF) as u16;
            slot.value
                .store(new_entry.value.load(Ordering::Relaxed), Ordering::Relaxed);
        }
    }

    pub fn hashfull(&self) -> usize {
        self.used * 1000 / self.size
    }
}

impl TableEntry {
    const PROMOTION_BITS: [Option<Piece>; 8] = [
        None,
        Some(Piece::Knight),
        Some(Piece::Bishop),
        Some(Piece::Rook),
        Some(Piece::Queen),
        None,
        None,
        None,
    ];

    pub fn pack(
        key: u64,
        eval: Value,
        depth: Depth,
        mv: ChessMove,
        bound: EvalBound,
        is_pv: bool,
    ) -> Self {
        let mut value = 0u64;
        value |= (eval.0 as u64) << 48;
        value |= (depth.0 as u64) << 32;
        value |= (mv.get_source().to_int() as u64) << 24;
        value |= (mv.get_dest().to_int() as u64) << 16;
        value |= (Self::PROMOTION_BITS
            .iter()
            .position(|x| x.eq(&mv.get_promotion()))
            .unwrap_or(0) as u64)
            << 13;
        value |= (bound as u64) << 11;
        value |= (is_pv as u64) << 10;
        // since we just created this entry, it is valid (unlike the empty ones
        // initially in the table)
        value |= 0b10;
        // parity bit, set to 1 if the number of bits set in the value is even
        // ensures the full value is always odd parity
        value |= 1 ^ (value.count_ones() & 1) as u64;
        Self {
            key: key as u16,
            value: AtomicU64::new(value),
        }
    }

    pub fn depth(&self) -> Depth {
        Depth((self.value.load(Ordering::Relaxed) >> 32) as u16)
    }

    pub fn eval(&self) -> Value {
        Value((self.value.load(Ordering::Relaxed) >> 48) as i16)
    }

    pub fn mv(&self) -> ChessMove {
        let value = self.value.load(Ordering::Relaxed);
        let src = (value >> 24) as u8;
        let dest = (value >> 16) as u8;
        let promotion = Self::PROMOTION_BITS[(0b111 & (value >> 13)) as usize];
        unsafe { ChessMove::new(Square::new(src), Square::new(dest), promotion) }
    }

    pub fn mv_struct(&self) -> MV {
        MV(self.mv(), self.eval())
    }

    pub fn bound(&self) -> EvalBound {
        match 0b11 & (self.value.load(Ordering::Relaxed) >> 11) as u8 {
            0 => EvalBound::Exact,
            1 => EvalBound::LowerBound,
            2 => EvalBound::UpperBound,
            _ => unreachable!("bound value out of range"),
        }
    }

    pub fn is_pv(&self) -> bool {
        (self.value.load(Ordering::Relaxed) >> 10) & 1 == 1
    }

    pub fn is_valid_entry(&self) -> bool {
        let val = self.value.load(Ordering::Relaxed);
        (val & 0b10) == 0b10 && val.count_ones() & 1 == 1
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
