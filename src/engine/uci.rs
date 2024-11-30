//! this is a not-only-UCI engine, this module contains the backend for adapting
//! the engine to the protocol
use std::fmt::Write;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use lockfree::channel::RecvErr;

use crate::optlog;
use crate::search::exit_condition;
use crate::search::Message;
use crate::search::SearchInfo;
use crate::Engine;

/// How often to check for new uci messages from the search threads, in *ms*
pub const UCI_LISTENING_FREQUENCY: usize = 10;

impl Engine {
    /// Start the engine!!
    pub fn uci_go(&mut self) -> Result<()> {
        let mut listener = self.begin_search()?;

        optlog!(comm;debug;"creating listener thread for {:?}", listener);

        thread::spawn(move || {
            let mut miss = 0;
            let start = Instant::now();
            let mut best = None;
            let mut ponder = None;
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
                            nodes,
                            score,
                            time,
                            pv,
                        }) => {
                            println!(
                                "info depth {} score {} nodes {} nps {} time {} pv {}",
                                depth.0,
                                score,
                                nodes,
                                (nodes as f64 / time.as_secs_f64()) as usize,
                                time.as_millis(),
                                pv.iter().fold(String::new(), |mut acc, m| {
                                    write!(acc, "{} ", m.0).expect("strings shouldn't fail");
                                    acc
                                })
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
            if let Some(mv) = &best {
                print!("bestmove {} ", mv.0);
            }
            if let Some(mv) = &ponder {
                print!("ponder {} ", mv.0);
            }
            println!();
            optlog!(comm;info;"best move {} pondered {} in {}ms", best.unwrap_or_default(), ponder.unwrap_or_default(), start.elapsed().as_millis());
        });

        Ok(())
    }
}
