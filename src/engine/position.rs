//! the definition of a position.
//!
//! just a wrapper around [`chess::Board`] to customise things

use chess::Board;
use chess::ChessMove;
use chess::Color;
use chess::EMPTY;
use chess::Piece;

/// the number of halfmoves without a pawn move or a capture after which the
/// game is drawn.
pub const FIFTY_MOVE_LIMIT: usize = 100;

/// a position in a game
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Position {
    /// the board
    pub chessboard: Board,
    /// how many halfmoves have been played since the last pawn move or
    /// capture. [`chess::Board`] does not track this, and drops it when
    /// parsing a FEN, so it is maintained here.
    pub halfmove_clock: usize,
}

impl Position {
    /// make a move on the board. allocates a new [`Board`] and advances the
    /// halfmove clock
    pub fn make_move(&self, mv: ChessMove) -> Self {
        let new_pos = self.chessboard.make_move_new(mv);
        let halfmove_clock = if resets_fifty_move_clock(&self.chessboard, mv) {
            0
        } else {
            self.halfmove_clock + 1
        };
        Self {
            chessboard: new_pos,
            halfmove_clock,
        }
    }

    /// has the game been drawn by the fifty-move rule?
    pub fn is_fifty_move_draw(&self) -> bool {
        self.halfmove_clock >= FIFTY_MOVE_LIMIT
    }

    /// is there too little material left for either side to ever deliver
    /// checkmate? covers king versus king, and king and a single minor piece
    /// versus king.
    pub fn is_insufficient_material(&self) -> bool {
        let board = &self.chessboard;

        // any pawn, rook or queen can still mate (or promote into something
        // that can), so those end it immediately.
        if *board.pieces(Piece::Pawn) | *board.pieces(Piece::Rook) | *board.pieces(Piece::Queen)
            != EMPTY
        {
            return false;
        }

        // a single minor piece cannot force mate, whichever side owns it.
        // two or more can, so this deliberately stops at king versus king and
        // king and one minor versus king.
        (*board.pieces(Piece::Knight) | *board.pieces(Piece::Bishop)).popcnt() <= 1
    }
}

/// does `mv` restart the fifty-move counter?
///
/// only pawn moves and captures do. note that this is a strict subset of
/// [`is_irreversible`]: giving up castling rights makes earlier positions
/// unreachable, but does not reset the fifty-move clock.
pub fn resets_fifty_move_clock(previous: &Board, mv: ChessMove) -> bool {
    previous.piece_on(mv.get_source()) == Some(Piece::Pawn)
        || previous.piece_on(mv.get_dest()).is_some()
}

/// does `mv` make every position played so far unreachable?
///
/// true for pawn moves, captures, and moves that give up castling rights.
/// after any of those, no earlier position can be repeated.
pub fn is_irreversible(previous: &Board, current: &Board, mv: ChessMove) -> bool {
    resets_fifty_move_clock(previous, mv)
        || previous.castle_rights(Color::White) != current.castle_rights(Color::White)
        || previous.castle_rights(Color::Black) != current.castle_rights(Color::Black)
}

impl From<Board> for Position {
    fn from(board: Board) -> Self {
        Self {
            chessboard: board,
            halfmove_clock: 0,
        }
    }
}
