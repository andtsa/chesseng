extern crate chess;
extern crate regex;

use chess::{Board, ChessMove};

pub mod game;
pub mod trial;
pub mod bot;
pub mod engine;
pub mod util;
pub mod zobrist;
pub mod experiments;
pub trait Engine {
    fn evaluate(&self, board : &Board) -> f64;
    fn next_move(&mut self, board : &Board) -> ChessMove;
}

