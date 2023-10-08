use std::ops::BitAnd;
use chess::{Board, Color};
use chess::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::bot::Bot;

impl Bot {
    pub fn get_material_value(&self, board : &Board, side : Color) -> f64 {
        let mut v = 0.0;
        v += self.pieces.pawn * (board.color_combined(side).bitand(board.pieces(Pawn)).popcnt() as f64);
        v += self.pieces.knight * (board.color_combined(side).bitand(board.pieces(Knight)).popcnt() as f64);
        v += self.pieces.bishop * (board.color_combined(side).bitand(board.pieces(Bishop)).popcnt() as f64);
        v += self.pieces.rook * (board.color_combined(side).bitand(board.pieces(Rook)).popcnt() as f64);
        v += self.pieces.queen * (board.color_combined(side).bitand(board.pieces(Queen)).popcnt() as f64);
        v += self.pieces.king * (board.color_combined(side).bitand(board.pieces(King)).popcnt() as f64);
        v
    }
}
