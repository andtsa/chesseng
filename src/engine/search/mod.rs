pub mod moveordering;
pub mod negamax;
mod nodes;
mod process;

use std::thread;
use std::time::Instant;

use anyhow::bail;
use anyhow::Result;
use chess::ChessMove;
use lockfree::channel::spsc::{Receiver, Sender};
use log::debug;
use log::trace;

use crate::search::moveordering::ordered_moves;
use crate::search::negamax::{DbOpt, negamax};
use crate::setup::depth::Depth;
use crate::setup::depth::ONE_PLY;
use crate::setup::values::Value;
use crate::Engine;

pub static mut SEARCH_UNTIL: Option<Instant> = None;
pub static mut SEARCH_TO: Depth = Depth(0);
pub static mut SEARCHING: bool = false;
pub static mut EXIT: bool = false;

#[derive(Debug)]
pub enum Message {
    BestMove(ChessMove, Value),
    BestGuess(ChessMove, Value),
    Info(SearchInfo),
}

#[derive(Debug)]
pub struct SearchInfo {
    pub depth: Depth,
    pub score: Value,
    pub nodes: usize,
}

pub fn exit_condition() -> bool {
    unsafe {
        if EXIT || SEARCH_UNTIL.is_some_and(|u| u < Instant::now()) {
            SEARCHING = false;
            true
        } else {
            false
        }
    }
}

impl Engine {
    pub fn begin_search(&mut self) -> Result<Receiver<Message>> {
        debug!("begin_search called with depth {:?}", unsafe { SEARCH_TO });
        self.set_search(true);
        if exit_condition() {
            bail!("begin_search called with exit_condition true!");
        }

        let (mut publisher, receiver) = lockfree::channel::spsc::create();
        let board = self.board;
        let debug = self.debug;
        let trace = self.trace;

        thread::spawn(move || {
            let mut best_move: Option<ChessMove> = None;
            let mut best_value: Value = Value::MIN;

            let mut target_depth = Depth(0);
            let mut total_nodes = 0;
            while !exit_condition() && target_depth < unsafe { SEARCH_TO } {
                // go one level deeper
                target_depth += ONE_PLY;
                debug!("iterative deepening searching to depth {:?}", target_depth);

                // reset best move
                best_value = Value::MIN;
                let mut new_best_move: Option<ChessMove> = None;
                let mut alpha = Value::MIN;
                let beta = Value::MAX;

                let moves = ordered_moves(&board);
                trace!("ordered moves: {}", moves);

                for mv in moves {
                    let search_result = -negamax(
                        board.make_move_new(mv),
                        target_depth - 1,
                        -beta,
                        -alpha,
                        DbOpt { debug, trace, ab: true },
                    );

                    trace!(
                        "move {} has value {} ({} nodes)",
                        mv,
                        search_result.next_position_value,
                        search_result.nodes_searched
                    );
                    total_nodes += search_result.nodes_searched + 1;

                    if search_result.next_position_value > best_value {
                        best_value = search_result.next_position_value;
                        new_best_move = Some(mv);
                        // UCI guess, not final move but have one ready in case stop is received
                        if let Some(mv) = best_move {
                            publisher.send(Message::BestGuess(mv, best_value)).unwrap();
                        }
                    }
                    alpha = alpha.max(search_result.next_position_value);
                    
                    if exit_condition() {
                        return;
                    }
                } // we have checked all moves for this depth

                best_move = new_best_move;
                
                // new depth info
                info(&mut publisher, target_depth, best_value, total_nodes);

                if let Some(mv) = best_move {
                    publisher.send(Message::BestMove(mv, best_value)).unwrap();
                }
            }
            
            debug!("sending best move {:?}", best_move);

            if let Some(mv) = best_move {
                publisher.send(Message::BestMove(mv, best_value)).unwrap();
            }
        });

        Ok(receiver)
    }
}

fn info(publisher: &mut Sender<Message>, target_depth: Depth, best_value: Value, total_nodes: usize) {
    publisher
        .send(Message::Info(SearchInfo {
            depth: target_depth,
            score: best_value,
            nodes: total_nodes,
        }))
        .unwrap();
}

impl DbOpt {
    pub fn ab(v: bool) -> Self {
        Self { ab: v, debug: false, trace: false }
    }
    
    pub fn abd(v: bool) -> Self {
        Self { ab: v, debug: v, trace: false }
    }
    
    pub fn abt(v: bool) -> Self {
        Self { ab: v, debug: v, trace: v }
    }
    
    pub fn dt(v: bool) -> Self {
        Self { ab: false, debug: v, trace: v }
    }
    
    pub fn d(v: bool) -> Self {
        Self { ab: false, debug: v, trace: false }
    }
}
