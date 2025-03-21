//! history of good moves that caused cutoffs


use arr_macro::arr;
use chess::ChessMove;

use crate::optlog;

/// store moves that have historically been good, so we can use them again in
/// move ordering!
// pub static MOVE_HISTORY: MoveHistory = MoveHistory::empty();

/// todo:
#[derive(Debug)]
pub struct MoveHistory {
    /// todo:
    pub history: [[usize; 64]; 64],
    /// todo:
    pub killers: [[Option<ChessMove>; 2]; 512],
}

impl MoveHistory {
    /// new empty history
    pub const fn empty() -> Self {
        Self {
            history: arr![arr![0usize; 64]; 64],
            killers: arr![[None, None]; 512],
        }
    }

    /// has this non-capture move caused a cutoff in the past?
    pub fn is_killer(&self, mv: &ChessMove, ply: usize) -> bool {
        if let Some(entry) = self.killers.get(ply % 512) {
            entry.contains(&Some(*mv))
        } else {
            false
        }
    }

    /// get the historical value of this move
    pub fn history_score(&self, mv: &ChessMove) -> usize {
        self.history[mv.get_source().to_index()][mv.get_dest().to_index()]
    }

    /// this was a killer move, save it!
    pub fn save_killer(&mut self, mv: &ChessMove, ply: usize) {
        if let Some(entry) = self.killers.get_mut(ply % 512) {
            if entry[0] != Some(*mv) {
                entry[1] = entry[0];
                entry[0] = Some(*mv);
            }
        } else {
            optlog!(mv;debug;"killer move error..?");
        }
    }

    /// mark this move as a historical cutoff
    pub fn log_history(&mut self, mv: &ChessMove, ply: usize) {
        self.history[mv.get_source().to_index()][mv.get_dest().to_index()] += ply * ply
    }
}
