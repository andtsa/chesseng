use std::fmt::Write;
use std::thread;

use anyhow::Result;
use lockfree::channel::RecvErr;
use log::debug;
use log::trace;

use crate::search::exit_condition;
use crate::search::Message;
use crate::search::SearchInfo;
use crate::Engine;

impl Engine {
    /// Start the engine!!
    pub fn uci_go(&mut self) -> Result<()> {
        let mut listener = self.begin_search()?;

        debug!("creating listener thread for {:?}", listener);

        thread::spawn(move || {
            let mut miss = 0;
            let mut best = None;
            let mut ponder = None;
            loop {
                match listener.recv() {
                    Ok(msg) => match msg {
                        Message::BestMove(mv) => {
                            // println!("bestmove {}", mv);
                            trace!("listener slept {} times", miss);
                            best = Some(mv);
                            miss = 0;
                        }
                        Message::Ponder(mv) => {
                            debug!("ponder {}", mv.0);
                            ponder = Some(mv);
                            miss = 0;
                        }
                        Message::BestGuess(mv) => {
                            debug!("best guess {}", mv.0);
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
                        thread::sleep(std::time::Duration::from_millis(50));
                    }
                    Err(RecvErr::NoSender) => {
                        debug!("no sender, exiting listener thread");
                        break;
                    }
                }
                if exit_condition() {
                    if let Some(mv) = best {
                        print!("bestmove {} ", mv.0);
                    }
                    if let Some(mv) = ponder {
                        print!("ponder {} ", mv.0);
                    }
                    println!();
                    break;
                }
            }
        });

        Ok(())
    }
}
