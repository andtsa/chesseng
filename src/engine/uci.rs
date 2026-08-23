//! this is a not-only-UCI engine, this module contains the backend for adapting
//! the engine to the protocol
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use chess::ChessMove;
use chess::MoveGen;
use lockfree::channel::RecvErr;

use crate::Engine;
use crate::optlog;
use crate::search::Message;
use crate::search::SearchInfo;
use crate::search::exit_condition;

/// How often to check for new uci messages from the search threads, in *ms*
pub const UCI_LISTENING_FREQUENCY: usize = 10;

impl Engine {
    /// Start the engine!!
    pub fn uci_go(&mut self) -> Result<()> {
        // whatever happens, a `go` has to answer with exactly one bestmove, or
        // the GUI might wait forever. a legal move from the current position is kept
        // aside so there is always something to send.
        let fallback = MoveGen::new_legal(&self.board.chessboard).next();

        let mut listener = match self.begin_search() {
            Ok(l) => l,
            Err(e) => {
                // the search never started, usually because the time budget was
                // already spent by the time we got here.
                optlog!(comm;warn;"search did not start: {e}");
                send_bestmove(fallback, None);
                return Ok(());
            }
        };

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
                            sel_depth,
                            multi_pv,
                            nodes,
                            score,
                            time,
                            hashfull,
                            tb_hits,
                            pv,
                        }) => {
                            println!(
                                "info depth {} seldepth {} multipv {} nodes {} nps {} hashfull {} tbhits {} time {} score {} pv {}",
                                depth.0,     // Depth of the search
                                sel_depth.0, // Selective depth
                                multi_pv,    // Number of principal variations
                                nodes,       // Total nodes searched
                                (nodes as f64 / time.as_secs_f64()) as usize, // Nodes per second
                                hashfull,    // Hash table usage (in per mille)
                                tb_hits,     // Tablebase hits
                                time.as_millis(), // Time in milliseconds
                                score,       // Score (in centipawns)
                                pv.iter()
                                    .map(|m| format!("{}", m.0))
                                    .collect::<Vec<_>>()
                                    .join(" "), // Principal variation
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
            // a ponder move only makes sense next to a real search result
            let pondered = best.as_ref().and(ponder.as_ref()).map(|mv| mv.0);
            send_bestmove(best.as_ref().map(|mv| mv.0).or(fallback), pondered);
            optlog!(comm;info;"best move {} pondered {} in {}ms", best.unwrap_or_default(), ponder.unwrap_or_default(), start.elapsed().as_millis());
        });

        Ok(())
    }
}

/// `mv` is [`None`] only when the position has no legal moves at all,
/// in which case the protocol's null move says so explicitly.
fn send_bestmove(mv: Option<ChessMove>, ponder: Option<ChessMove>) {
    match mv {
        Some(mv) => {
            print!("bestmove {mv}");
            if let Some(ponder) = ponder {
                print!(" ponder {ponder}");
            }
            println!();
        }
        None => println!("bestmove 0000"),
    }
}
