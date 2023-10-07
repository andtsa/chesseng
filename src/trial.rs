#![allow(dead_code, unused)]
use std::process::exit;
use chess::{Board, ChessMove};
use crate::bot::Bot;
use crate::util::{fen_to_str, Stringify};

pub fn run() {
    let bot = Bot::new();
    let mut board : Board = Board::default();

    /*

    99999999
    88877888
    22433422
    01244210
    00144100
    00222200
    11100111
    00000000







    11111111
    11111111
    11111111
    01111110
    00111100
    00111100
    11100111
    00000000

    99999999
    88877888
    22433422
    01244210
    00144100
    00222200
    00000000
    00000000

    */
    board = board.null_move().unwrap();
    println!("{}, {}", board.side_to_move().stringify(), board.side_to_move().to_index());
    exit(0);
    // let book = OpeningBook::new();
    // println!("{}", book.next(&board).unwrap().to_string());

    // read_binary_opening_book("openings.bin");

    // board = Board::from_str("r1b2r1k/4qp1p/p1Nppb1Q/4nP2/1p2P3/2N5/PPP4P/2KR1BR1 b - - 5 18").unwrap();
    // board = Board::from_str("b2r3r/4Rp1p/p2q1np1/kp1P4/3Q4/P4PPB/1PP4P/1K6 w - - 0 1").unwrap();

    // println!("{}", fen_to_str(board.to_string()));
    // for v in ordered_moves(&board) {
    //     println!("{}",v.to_string());
    // }

    // let reps = 100000;
    //
    //
    // let t1 = Instant::now();
    // for _ in 0..reps {
    //     all_moves(&board);
    // }
    // let st1 = t1.elapsed().as_secs_f64();
    // println!("t1: {}s", st1/(reps as f64));
    //
    // let t2 = Instant::now();
    // for _ in 0..reps {
    //     ordered_moves(&board);
    // }
    // let st2 = t2.elapsed().as_secs_f64();
    // println!("t1: {}s", st2/(reps as f64));
    // let mvs = vec!["f3", "e5", "g4", "Qh4"]; // black scholar's mate on white
    // let mvs = vec!["e4", "f6","a3", "g5","Qh5"]; // white wins by scholar's mate
    // let mvs = vec!["e4", "d5", "xd5", "e6", "Bb5", "c6", "Qe2", "Bb4", "Qg4","Bxd2","Kxd2","f6","Qxe6"];
    // let mvs = vec!["e4", "d6","d4", "Nf6","Nc3",];


    // let mvs = vec!["e4", "d6", "d4", "Nf6", "Nc3", "g6", "Be3", "Bg7", "Qd2", "c6", "f3", "b5", "Nge2", "Nbd7", "Bh6", "Bxh6", "Qxh6", "Bb7", "a3" ,"e5", "O-O-O", "Qe7", "Kb1", "a6",
    //                "Nc1", "O-O-O", "Nb3", "exd4" ,"Rxd4", "c5", "Rd1", "Nb6", "g3", "Kb8","Na5", "Ba8" , "Bh3", "d5", "Qf4", "Ka7", "Rhe1", "d4", "Nd5", "Nbxd5","exd5", "Qd6", "Rxd4", "cxd4", "Re7", "Kb6", "Qxd4", "Kxa5"];
    // 
    // for m in mvs {
    //     println!("\n--\n{}\n{}",fen_to_str(board.to_string()) , bot.eval(board));
    // 
    //     board = board.make_move_new(ChessMove::from_san(&board, m).unwrap());
    // }
    // 
    // println!("{}", board.to_string());
    // 
    // 
    // exit(0);
}
