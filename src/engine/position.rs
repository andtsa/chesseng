//! the definition of a position.
//!
//! just a wrapper around [`chess::Board`] to customise things

use chess::Board;
use chess::ChessMove;

/// a position in a game
#[derive(Debug, Clone, Default)]
pub struct Position {
    /// the board
    pub chessboard: Board,
    /// the number of plies in this game
    pub moves_played: usize,
}

impl Position {
    /// make a move on the board. allocates a new [`Board`] and increments the
    /// number of plies
    pub fn make_move(&self, mv: ChessMove) -> Self {
        let new_pos = self.chessboard.make_move_new(mv);
        Self {
            chessboard: new_pos,
            moves_played: self.moves_played + 1,
        }
    }
}

impl From<Board> for Position {
    fn from(board: Board) -> Self {
        Self {
            chessboard: board,
            moves_played: 0,
        }
    }
}