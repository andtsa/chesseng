#![feature(const_trait_impl)]
#![allow(dead_code, unused)]
/*! **TODO**
- [ ] benchmark different search function setups to optimise for move generation.
- [ ] research move ordering techniques and benchmark implementations to find best end-result speed
- [ ] find more ways to prune search tree branches for unpromising paths
- [ ] implement iterative deepening on top of DFS for minimax
- [ ] set iterative deepening end to timestamp instead of depth, to let engine think deeper when time is available
- [ ] create each search branch on a different thread in compute_best_move to make things faster

- [ ] add more theory to existing evaluation function
- [ ] create environment for genetic algorithm optimisation of parameters
- [ ] look into gradient descent optimisation for tuning evaluation parameters

- [ ] research optimal CNN setup
- [ ] implement neural network
- [ ] train using test.db lichess database
- [ ] see if LSTM or recurrent NN is viable for this purpose
bitboards for NN:
white pawns    |  black pawns
white knights  |  black knights
white bishops  |  black bishops
white rooks    |  black rooks
white queens   |  black queens
white kings    |  black kings
 */
extern crate core;

mod evaluation;
mod util;
mod move_generation;
mod bot;
mod opening_book;
mod read_book;

use std::collections::HashMap;
use std::io::stdin;
use std::ops::{BitAnd, BitOrAssign};
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;
use chess::{Board, Color, BoardStatus, ChessMove, Square, BitBoard, get_king_moves};
use chess::BoardStatus::{Checkmate, Stalemate};
use chess::Color::{Black, White};
use chess::File::D;
use chess::Piece::Pawn;
use chess::Rank::{Second, Third};
use ndarray::{arr2, Array2};
use crate::bot::{Bot, dx};
use crate::evaluation::{squares_covered_by_side};
use crate::move_generation::{all_moves, compute_best_move, ordered_moves};
use crate::opening_book::OpeningBook;
use crate::read_book::read_binary_opening_book;
use crate::util::{fen_to_str, fen_to_string_highlighted, make_cm, Stringify};



// dev options, discard when finished.
pub const TEST_RUN : bool = true;
pub const COMPUTER_ONLY : bool = false;

pub const COMPUTER_PLAYER : Color = White;
pub const SEARCH_DEPTH : u32 = 4;



fn player_move(board : &Board) -> ChessMove {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("TODO: panic message");
    let res = buffer.trim_end();
    if res == "" {exit(0)}
    let mut mv = ChessMove::from_san(board, res);//make_cm(res);
    let mut repeat_q : bool = mv.is_err() || !board.legal(mv.clone().unwrap());
    while repeat_q {
        println!("invalid move, retry: ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("TODO: panic message");
        let res = buffer.trim_end();
        mv = ChessMove::from_san(board, res);
        repeat_q = mv.is_err() || !board.legal(mv.clone().unwrap());
        // println!("{res},{repeat_q}");
    }
    return mv.clone().unwrap();
}


fn bot_move(bot : &Bot, board : &Board) -> ChessMove {
    let now = Instant::now();
    println!("computing best move for {}", board.side_to_move().stringify());
    let sd = bot.get_search_depth(*board);

    let mut mv = compute_best_move(board, sd, bot); //full_search(board, FULL_SEARCH_DEPTH).moves.pop_front().unwrap();
    let mut repeat_q : bool = !board.legal(mv);
    while repeat_q {
        println!("Computer made illegal move {}, retrying", mv.to_string());
        mv = compute_best_move(board, sd, bot);
        repeat_q = !board.legal(mv);
    }

    println!("bot computed move in {} milliseconds", now.elapsed().as_millis());
    return mv;
}

fn end_game(board : Board, message : &str, start : Instant, bot : &Bot) {
    println!("{message}\nGame lasted for {} seconds", start.elapsed().as_secs_f64());
    println!("\nFinal Board:\n{}", fen_to_str(board.to_string()));
    bot.save();
    exit(0);
}

fn main() {
    if TEST_RUN {test_run();}
    let judge = Bot::new();

    let mut b1 = Bot::new();
    let mut b2 = Bot::new();
    // b1.search_depth = 2;
    // b2.search_depth = 3;
    b2.mutate(2.0, 0.5);
    // b1.mutate(2.0, 1.0);
    // let mut mut_vec : Vec<f64> = Vec::new();
    // for _ in 0..289 {
    //     mut_vec.push(dx(0.005));
    // }
    // let mut_matrix = Array2::from_shape_vec((17,17), mut_vec);
    // println!("{}", b1.printout());
    // b1.matrix_mutate(mut_matrix.unwrap());
    // println!("{}", b1.printout());


    let start = Instant::now();
    let mut board : Board = Board::default();

    println!("Starting new game with White: {} Black: {}.\n{}", if COMPUTER_ONLY||COMPUTER_PLAYER==White{"Computer"}else{"Player"}, if COMPUTER_ONLY||COMPUTER_PLAYER==Black{"Computer"}else{"Player"}, fen_to_str(board.to_string()));
    let mut turn_count : u32 = 0;
    let mut moves_since_capture : u32 = 0;
    loop {
        if board.status()==Checkmate {
            println!("{}",fen_to_str(board.to_string()));
            println!("Checkmate!!");
            if board.side_to_move() == Black {
                println!("White won");
                //println!("Parameters:\n{}", b1.printout());
            } else {
                println!("Black won");
                //println!("Parameters:\n{}", b2.printout());
            }
            if board.side_to_move() == COMPUTER_PLAYER {
                // b2 won,
                b2.save();
            } else {
                // b1 won,
                b1.save();
            }
            exit(0);
        } else if board.status()==BoardStatus::Stalemate {
            println!("{}",fen_to_str(board.to_string()));
            end_game(board, "Stalemate!", start, &b2);
        }

        let mut mv : ChessMove;

        println!("{} to move", board.side_to_move().stringify());

        if board.side_to_move() == COMPUTER_PLAYER {
            mv = bot_move(&b1, &board);
        } else {
            if COMPUTER_ONLY {
                mv = bot_move(&b2, &board);
            } else {
                mv = player_move(&board);
            }
        }

        let capture = board.piece_on(mv.get_dest()) != None;
        let pawn_move = board.piece_on(mv.get_source()) == Option::from(Pawn);
        if capture || pawn_move {
            moves_since_capture = 0;
            if board.combined().popcnt() <= 3 {
                end_game(board, "Draw by insufficient material", start, &b2);
            }
        } else {
            moves_since_capture += 1;
        }
        if moves_since_capture >= 50 {
            end_game(board, "Draw by 50-move rule.", start, &b2);
        }

        board = board.make_move_new(mv);
        let e = judge.eval(board);
        println!("\n{}\nTurn #{} Move #{}. [{}{}]", fen_to_string_highlighted(board.to_string(), mv, capture, pawn_move), turn_count>>1, turn_count, if e>0.0{"+"}else{""},e);
        turn_count+=1;
    }

}


fn test_run() {
    let bot = Bot::new();
    let mut board : Board = Board::default();
    let book = OpeningBook::new();
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


    let mvs = vec!["e4", "d6", "d4", "Nf6", "Nc3", "g6", "Be3", "Bg7", "Qd2", "c6", "f3", "b5", "Nge2", "Nbd7", "Bh6", "Bxh6", "Qxh6", "Bb7", "a3" ,"e5", "O-O-O", "Qe7", "Kb1", "a6",
        "Nc1", "O-O-O", "Nb3", "exd4" ,"Rxd4", "c5", "Rd1", "Nb6", "g3", "Kb8","Na5", "Ba8" , "Bh3", "d5", "Qf4", "Ka7", "Rhe1", "d4", "Nd5", "Nbxd5","exd5", "Qd6", "Rxd4", "cxd4", "Re7", "Kb6", "Qxd4", "Kxa5"];

    for m in mvs {
        println!("\n--\n{}\n{}",fen_to_str(board.to_string()) , bot.eval(board));

        board = board.make_move_new(ChessMove::from_san(&board, m).unwrap());
    }

    println!("{}", board.to_string());


    exit(0);
}