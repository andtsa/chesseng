#![allow(dead_code, unused)]

use std::ops::{BitAnd, Not};
use std::process::exit;
use chess::{Board, ChessMove, Square, BitBoard};
use chess::Color::{Black, White};
use chess::Piece::Pawn;
use crate::Engine;
use crate::game::stockfish_evaluation::Stockfish;
// use crate::bot::Bot;
// use crate::util::{fen_to_str, Stringify};
// use crate::engine::parameters;

pub fn run() {
    let board = Board::default();
    let mut bb = board.color_combined(White).bitand(board.pieces(Pawn)).to_size(0);
    let mut i : Vec<u64> = Vec::new();
    println!("{:b}", bb);
    println!("{:b}", (bb.not()+1));
    println!("{:b}", bb&(bb.not()+1)); // least significant bit

    let mut pointer = 0u64;
    while bb > 0 {
        if bb & 1<<pointer != 0 {
            bb = bb ^ 1<<pointer;
            i.push(pointer);
        }
        pointer += 1;
    }

    println!("{}", board.king_square(White).to_index());
}

/*
You already know how to do it by successive division by 2.

x >> 1 is the same as x / 2 for any unsigned integer in C.

If you need to make this faster, you can do a
"divide and conquer"—shift, say, 4 bits at a time until you
reach 0, then go back and look at the last 4 bits. That
means at most 16 shifts and 19 compares instead of 63 of each.
Whether it's actually faster on a modern CPU, I couldn't say
without testing. And you can take this a step farther, to
first do groups of 16, then 4, then 1. Probably not useful
here, but if you had some 1024-bit integers, it might be
worth considering.
 */

// // let bot = Bot::new();
// let mut board : Board = Board::default();
//
// let mut s = Stockfish::new();
//
// // let mvs = vec!["f3", "e5", "g4", "Qh4"]; // black scholar's mate on white
// // let mvs = vec!["e4", "f6","a3", "g5","Qh5"]; // white wins by scholar's mate
// // let mvs = vec!["e4", "d5", "xd5", "e6", "Bb5", "c6", "Qe2", "Bb4", "Qg4","Bxd2","Kxd2","f6","Qxe6"];
// // let mvs = vec!["e4", "d6","d4", "Nf6","Nc3",];
//
// let mvs = vec!["e4", "d6", "d4", "Nf6", "Nc3", "g6", "Be3", "Bg7", "Qd2", "c6", "f3", "b5", "Nge2", "Nbd7", "Bh6", "Bxh6", "Qxh6", "Bb7", "a3" ,"e5", "O-O-O", "Qe7", "Kb1", "a6",
//                "Nc1", "O-O-O", "Nb3", "exd4" ,"Rxd4", "c5", "Rd1", "Nb6", "g3", "Kb8","Na5", "Ba8" , "Bh3", "d5", "Qf4", "Ka7", "Rhe1", "d4", "Nd5", "Nbxd5","exd5", "Qd6", "Rxd4", "cxd4", "Re7", "Kb6", "Qxd4", "Kxa5"];
// //
// for m in mvs {
// // println!("\n--\n{}\n{}",fen_to_str(board.to_string()) , bot.eval(board));
// println!("{}", s.evaluate(board));
// board = board.make_move_new(ChessMove::from_san(&board, m).unwrap());
// }