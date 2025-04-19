//! move generation utilities

pub mod heuristics;
pub mod ordering;
use std::fmt::Debug;

use chess::BitBoard;
use chess::Board;
use chess::ChessMove;
use chess::EMPTY;
use chess::MoveGen;
use heuristics::mvv_lva_score;

use crate::evaluation::bitboards::CENTER_4;
use crate::evaluation::bitboards::CENTER_16;

/// An wrapper around [`MoveGen`] that orders the moves based on some heuristics
pub struct OrderedMoves {
    /// prioritised moves (eg PV)
    prio_moves: Vec<ChessMove>,
    /// the internal move generator
    mgen: MoveGen,
    /// the positions to generate first
    pub masks: [BitBoard; 8],
    /// current mask index
    cur_mask: usize,
}

/// constructor for [`OrderedMoves`]
///
/// prio_moves will have elements removed from the end, so order them from least
/// important to most important
pub fn prio_iterator(mut mgen: MoveGen, pos: &Board, prio: &[ChessMove]) -> OrderedMoves {
    for mv in prio {
        mgen.remove_move(*mv);
    }
    let masks = [
        *pos.pieces(chess::Piece::Queen),
        *pos.pieces(chess::Piece::Rook),
        *pos.pieces(chess::Piece::Bishop),
        *pos.pieces(chess::Piece::Knight),
        *pos.pieces(chess::Piece::Pawn),
        CENTER_4,
        CENTER_16,
        !EMPTY,
    ];

    // collect capture moves and sort by mvv-lva
    mgen.set_iterator_mask(*pos.color_combined(!pos.side_to_move()));

    let mut prio_moves: Vec<ChessMove> = mgen.by_ref().collect();
    prio_moves.sort_by_cached_key(|mv| mvv_lva_score(pos, mv));

    prio_moves.extend_from_slice(prio);

    mgen.set_iterator_mask(masks[0]);

    OrderedMoves {
        prio_moves,
        mgen,
        masks,
        cur_mask: 0,
    }
}

impl Iterator for OrderedMoves {
    type Item = ChessMove;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prio) = self.prio_moves.pop() {
            return Some(prio);
        }
        loop {
            if let Some(mv) = self.mgen.next() {
                return Some(mv); // quick return for branch prediction
            }
            if self.cur_mask >= self.masks.len() - 1 {
                return None; // iterator is finished
            }
            self.cur_mask += 1;
            self.mgen.set_iterator_mask(self.masks[self.cur_mask]);
        }
    }
}

impl OrderedMoves {
    /// number of moves contained by this iterator
    pub fn len(&mut self) -> usize {
        self.mgen.set_iterator_mask(!EMPTY);
        let mgen_len = self.mgen.len();
        self.mgen.set_iterator_mask(self.masks[self.cur_mask]);
        self.prio_moves.len() + mgen_len
    }

    /// is the iterator finished?
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    /// get the first move from the iterator
    pub fn first(&mut self) -> Option<ChessMove> {
        if let Some(m) = self.next() {
            self.prio_moves.push(m);
            Some(m)
        } else {
            None
        }
    }

    /// consume self to print
    pub fn display(self) -> String {
        let mut ret = String::new();
        ret.push_str("OrderedMoves: ");
        ret.push_str(
            &self
                .enumerate()
                .map(|(i, m)| format!("{i}:{}", m))
                .collect::<Vec<String>>()
                .join(", "),
        );
        ret
    }
}

impl Debug for OrderedMoves {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "OrderedMoves(mgen?, {:?}, {})",
            self.masks, self.cur_mask
        )
    }
}
