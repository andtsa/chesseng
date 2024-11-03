use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use chess::{Board, ChessMove};
use chess::BoardStatus::Ongoing;
use rayon::join;
use rayon::prelude::*;
use crate::bot::Bot;
use crate::Engine;
use crate::engine::move_generation::generate_moves;
use crate::util::{all_moves};
use crate::util::pick_max;
use crate::util::pick_min;

impl Bot {
    pub fn iterative_deepening_minimax(&self, board : &Board, maximizing : bool, timeout : Duration) -> ChessMove {
        let mut best_move : Option<ChessMove> = None;
        let mut best_value : f64 = if maximizing { f64::MIN } else { f64::MAX };
        let mut depth: i32 = 2;
        let start = Instant::now();
        let end = start + timeout;
        let move_list = &generate_moves(board);
        if move_list.len() <= 1 {
            return move_list[0];
        }
        let moves: Arc<Mutex<Vec<(f64, ChessMove)>>> = Arc::new(Mutex::new(Vec::new()));


        while Instant::now() < end {
            println!("Searching to a depth of {}", depth);
            move_list.iter().for_each(|&mv| {
                    let moves = Arc::clone(&moves);
                    let new_board = board.make_move_new(mv);
                    
                        let value = self.minimax(&new_board, end, depth - 1, depth - 1, maximizing, f64::MIN, f64::MAX);
                        if value != None {
                            moves.lock().unwrap().push((value.unwrap_or(best_value / 2.0), mv));
                        }
                });
            let binding = moves.lock().unwrap();
            for (value, mv) in binding.deref() {
                if (value > &best_value && maximizing) || (value < &best_value && !maximizing) {
                    best_value = *value;
                    best_move = Some(*mv);
                }
            }
            depth += 1;
        }

        best_move.unwrap_or(move_list[0])
    }

    pub fn minimax(&self, board : &Board, end : Instant, depth : i32, original_depth : i32, maximizing : bool, mut alpha: f64, mut beta: f64) -> Option<f64> {
        if board.status() != Ongoing {
            return Some(self.evaluate(board));
        }
        let table = Arc::clone(&self.transposition_table);
        let r = table.read().unwrap().get(&board.get_hash()).cloned();
        if r != None && r.unwrap().0 > depth {
            return Some(r.unwrap().1);
        }
        if depth <= 0 {
            let r_eval = Some(self.evaluate(board));
            table.write().unwrap().insert(board.get_hash(),(original_depth, r_eval.unwrap()));
            return r_eval;
        }
        if Instant::now() > end {
            return None;
        }

        if maximizing {
            let mut max_eval = f64::MIN;
            for &mv in &generate_moves(board) {
                let new_board = board.make_move_new(mv);
                let eval = self.minimax(&new_board,end,depth-1, original_depth,false,alpha,beta);
                if eval == None {
                    return None
                }
                max_eval = pick_max(max_eval, eval.unwrap());
                alpha = pick_max(alpha, eval.unwrap());
                if beta <= alpha {
                    break;
                }
            }
            if max_eval > f64::MIN && Instant::now() < end {
                table.write().unwrap().insert(board.get_hash(), (original_depth, max_eval));
            }
            return Some(max_eval)
        } else {
            let mut min_eval = f64::MAX;
            for &mv in &generate_moves(board) {
                let new_board = board.make_move_new(mv);
                let eval = self.minimax(&new_board,end,depth-1,original_depth, true,alpha,beta);
                if eval == None {
                    return None
                }
                min_eval = pick_min(min_eval, eval.unwrap());
                beta = pick_min(beta, eval.unwrap());
                if beta <= alpha {
                    break;
                }
            }
            if min_eval < f64::MAX && Instant::now() < end {
                table.write().unwrap().insert(board.get_hash(), (original_depth, min_eval));
            }
            return Some(min_eval)
        }
    }
}

