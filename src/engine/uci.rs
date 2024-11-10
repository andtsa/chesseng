use std::thread;

use lockfree::channel::RecvErr;
use log::debug;
use log::trace;

use crate::search::exit_condition;
use crate::search::Message;
use crate::search::SearchInfo;
use crate::setup::values::Value;
use crate::Engine;

impl Engine {
    /// Start the engine!!
    pub fn uci_go(&mut self) -> anyhow::Result<()> {
        let mut listener = self.begin_search()?;

        debug!("creating listener thread for {:?}", listener);

        thread::spawn(move || {
            let mut miss = 0;
            let mut best = (None, Value::MIN);
            loop {
                match listener.recv() {
                    Ok(msg) => match msg {
                        Message::BestMove(mv, val) => {
                            // println!("bestmove {}", mv);
                            trace!("listener slept {} times", miss);
                            best = (Some(mv), val);
                            miss = 0;
                        }
                        Message::BestGuess(mv, val) => {
                            debug!("best guess {}", mv);
                            if val > best.1 {
                                best = (Some(mv), val);
                            }
                            miss = 0;
                        }
                        Message::Info(SearchInfo {
                            depth,
                            nodes,
                            score,
                        }) => {
                            println!("info depth {} score {}, nodes {}", depth.0, score, nodes);
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
                    if let Some(mv) = best.0 {
                        println!("bestmove {}", mv);
                    }
                    break;
                }
            }
        });

        Ok(())
    }
}
