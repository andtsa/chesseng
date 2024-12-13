//! this is a not-only-UCI engine, this module contains the backend for adapting
//! the engine to the protocol
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::bail;
use anyhow::Result;
use lockfree::channel::RecvErr;

use crate::optlog;
use crate::search::exit_condition;
use crate::search::should_ponder;
use crate::search::Message;
use crate::search::SearchInfo;
use crate::setup::depth::Depth;
use crate::Engine;

/// How often to check for new uci messages from the search threads, in *ms*
pub const UCI_LISTENING_FREQUENCY: usize = 10;

/// Save the pondered best move
pub static PONDER_BEST_MOVE: (AtomicU64, AtomicU64) = (AtomicU64::new(0), AtomicU64::new(0));

impl Engine {
    /// Start the engine!!
    pub fn uci_go(&mut self) -> Result<()> {
        if exit_condition() {
            bail!("uci_go called with exit_condition true!");
        }
        let mut listener = self.begin_search()?;

        optlog!(comm;debug;"creating listener thread for {:?}", listener);

        thread::spawn(move || {
            println!("info string starting uci listen thread");
            let mut miss = 0;
            let start = Instant::now();
            let mut best = None;
            let mut ponder = None;
            let mut max_depth = Depth(0);
            loop {
                match listener.recv() {
                    Ok(msg) => match msg {
                        Message::BestMove(mv) => {
                            // println!("bestmove {}", mv);
                            optlog!(comm;debug;"received best move {} with val {}", mv.0, mv.1);
                            optlog!(comm;trace;"listener slept {} times", miss);
                            best = Some(mv);
                            miss = 0;
                        }
                        Message::Ponder(mv) => {
                            optlog!(comm;debug;"ponder {}", mv.0);
                            ponder = Some(mv);
                            miss = 0;
                        }
                        Message::BestGuess(mv) => {
                            optlog!(comm;debug;"best guess {}", mv.0);
                            if best.as_ref().is_none_or(|b| b.1 < mv.1) {
                                best = Some(mv);
                            }
                            miss = 0;
                        }
                        Message::Info(SearchInfo {
                            depth,
                            sel_depth,
                            multi_pv,
                            nodes,
                            score,
                            time,
                            hashfull,
                            tb_hits,
                            pv,
                        }) => {
                            max_depth = max_depth.max(depth);

                            println!(
                                "info depth {} seldepth {} multipv {} nodes {} nps {} hashfull {} tbhits {} time {} score {} pv {}",
                                depth.0,                              // Depth of the search
                                sel_depth.0,                          // Selective depth
                                multi_pv,                             // Number of principal variations
                                nodes,                                // Total nodes searched
                                (nodes as f64 / time.as_secs_f64()) as usize, // Nodes per second
                                hashfull,                             // Hash table usage (in per mille)
                                tb_hits,                              // Tablebase hits
                                time.as_millis(),                     // Time in milliseconds
                                score,                                // Score (in centipawns)
                                pv.iter().map(|m| format!("{}", m.0)).collect::<Vec<_>>().join(" "), // Principal variation
                            );
                        }
                    },
                    Err(RecvErr::NoMessage) => {
                        miss += 1;
                        thread::sleep(Duration::from_millis(UCI_LISTENING_FREQUENCY as u64));
                    }
                    Err(RecvErr::NoSender) => {
                        optlog!(comm;debug;"no sender, exiting listener thread");
                        break;
                    }
                }
                if exit_condition() {
                    optlog!(comm;debug;"exit condition met, exiting listener thread");
                    break;
                }
            }

            if !should_ponder() {
                // don't print move in ponder mode.
                if let Some(mv) = &best {
                    print!("bestmove {}", mv.0);
                }
                if let Some(mv) = &ponder {
                    print!(" ponder {}", mv.0);
                }
                println!();
            } else {
                if let Some(mv) = &best {
                    PONDER_BEST_MOVE
                        .0
                        .store(mv.as_u64(max_depth), Ordering::Relaxed);
                }
                if let Some(mv) = &ponder {
                    PONDER_BEST_MOVE
                        .1
                        .store(mv.as_u64(max_depth), Ordering::Relaxed);
                }
            }

            println!("exiting uci listen thread");

            optlog!(comm;info;"best move {} pondered {} in {}ms", best.unwrap_or_default(), ponder.unwrap_or_default(), start.elapsed().as_millis());
        });

        Ok(())
    }
}
