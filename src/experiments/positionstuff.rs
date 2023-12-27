#[warn(dead_code)]
use std::str::FromStr;
use chess::{Board, ChessMove, Square};
use crate::bot::Bot;
use crate::util::fen_to_str;

pub fn run() {

    println!("{}", Bot::new().positions.pawn[23]);
    // println!("{}", 3usize - 5usize);

    let a1 = Square::A2;
    let a2 = Square::A4;
    // println!("{} {}", a1.to_index(), a2.to_index());
    let a1 = Square::E4;
    let a2 = Square::E5;
    // println!("{} {}", a1.to_index(), a2.to_index()-63);
    // let a1 = Square::B3;
    // let a2 = Square::B6;
    // println!("{} {}", a1.to_index(), a2.to_index());
    let mv = ChessMove::from_str("e7e5").unwrap();
    // println!("{}", mv.get_source().to_index());
    // let mut b = Board::default();
    // b = b.make_move_new(mv);
    // println!("{}", fen_to_str(b.to_string()));

}
// 6k1/pp4pp/1bn5/3p4/6P1/2PQ3P/PK6/4q3 b - - 0 1