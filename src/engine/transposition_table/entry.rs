//! transposition table entries
#![allow(dead_code)]
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use chess::ChessMove;
use chess::Piece;
use chess::Square;

use crate::search::SearchResult;
use crate::search::MV;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::transposition_table::EvalBound;
use crate::transposition_table::TEntry;

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
    pub key: u64,
    /// the packed value of this entry
    pub value: AtomicU64,
}

impl TableEntry {
    /// the possible promotion pieces
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

    /// pack a new entry with the given values
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
            key,
            value: AtomicU64::new(value),
        }
    }

    /// get the depth of this entry's value
    pub fn depth(&self) -> Depth {
        Depth((self.value.load(Ordering::Relaxed) >> 32) as u16)
    }

    /// get the evaluation value from this entry
    pub fn eval(&self) -> Value {
        Value((self.value.load(Ordering::Relaxed) >> 48) as i16)
    }

    /// get the [`ChessMove`] from this entry
    pub fn mv(&self) -> ChessMove {
        let value = self.value.load(Ordering::Relaxed);
        let src = (value >> 24) as u8;
        let dest = (value >> 16) as u8;
        let promotion = Self::PROMOTION_BITS[(0b111 & (value >> 13)) as usize];
        // SAFETY: the values are stored in the correct range since they're always
        // created from a [`ChessMove`] struct in the first place
        unsafe { ChessMove::new(Square::new(src), Square::new(dest), promotion) }
    }

    /// get the [`MV`] struct for the move stored in this entry
    pub fn mv_struct(&self) -> MV {
        MV(self.mv(), self.eval())
    }

    /// get the [`EvalBound`] for this entry
    pub fn bound(&self) -> EvalBound {
        match 0b11 & (self.value.load(Ordering::Relaxed) >> 11) as u8 {
            0 => EvalBound::Exact,
            1 => EvalBound::LowerBound,
            2 => EvalBound::UpperBound,
            _ => unreachable!("bound value out of range"),
        }
    }

    /// check if the entry is in the principal variation
    pub fn is_pv(&self) -> bool {
        (self.value.load(Ordering::Relaxed) >> 10) & 1 == 1
    }

    /// check if the entry is valid using the 'valid' bit
    pub fn is_valid_entry(&self) -> bool {
        let val = self.value.load(Ordering::Relaxed);
        (val & 0b10) == 0b10 && val.count_ones() & 1 == 1
    }
}

impl Clone for TableEntry {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            value: AtomicU64::new(self.value.load(Ordering::Relaxed)),
        }
    }
}

impl TEntry for TableEntry {
    type Key = u64;

    #[inline]
    fn key(&self) -> Self::Key {
        self.key
    }

    #[inline]
    fn new_empty() -> Self {
        Self {
            key: 0,
            value: AtomicU64::new(0),
        }
    }

    fn new_from_result(hash: u64, depth: Depth, result: &SearchResult, bound: EvalBound) -> Self {
        Self::pack(
            hash,
            result.next_position_value,
            depth,
            result.pv[0].0,
            bound,
            false,
        )
    }

    #[inline]
    fn depth(&self) -> Depth {
        self.depth()
    }

    #[inline]
    fn bound(&self) -> EvalBound {
        self.bound()
    }

    #[inline]
    fn search_result(&self) -> SearchResult {
        SearchResult {
            pv: vec![self.mv_struct()], // ?
            next_position_value: self.eval(),
            nodes_searched: 1,
            tb_hits: 1,
        }
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.is_valid_entry()
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
