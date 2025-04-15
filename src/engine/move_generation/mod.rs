//! move generation utilities
#![allow(unused)] // TODO: remove

use std::fmt::Debug;
use std::ops::BitAnd;
use std::ops::Not;

use chess::BitBoard;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::EMPTY;

use crate::evaluation::bitboards::CENTER_16;
use crate::evaluation::bitboards::CENTER_4;

/// An wrapper around [`MoveGen`] that orders the moves based on some heuristics
pub struct OrderedMoves {
    /// the internal move generator
    mgen: MoveGen,
    /// the positions to generate first
    pub masks: [BitBoard; 6],
    /// current mask index
    next_mask: usize,
}

/// constructor for [`OrderedMoves`]
pub fn prio_iterator(mgen: MoveGen, pos: &Board) -> OrderedMoves {
    let opp = pos.color_combined(pos.side_to_move().not());
    let masks = [
        *pos.pieces(chess::Piece::Queen),
        *pos.pieces(chess::Piece::Rook),
        *pos.pieces(chess::Piece::Bishop),
        *pos.pieces(chess::Piece::Knight),
        *pos.pieces(chess::Piece::Pawn),
        // CENTER_4,
        // CENTER_16,
        !EMPTY,
    ];

    OrderedMoves {
        mgen,
        masks,
        next_mask: 0,
    }
}

impl Iterator for OrderedMoves {
    type Item = ChessMove;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mv) = self.mgen.next() {
                return Some(mv); // quick return for branch prediction
            }
            if self.next_mask >= self.masks.len() {
                return None; // iterator is finished
            }
            self.mgen.set_iterator_mask(self.masks[self.next_mask]);
            self.next_mask += 1;
        }
    }
}

impl Debug for OrderedMoves {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "OrderedMoves(mgen?, {:?}, {})",
            self.masks, self.next_mask
        )
    }
}
