//! # Sandy Engine
//! all logic for the engine lies in this lib
#![deny(rustdoc::broken_intra_doc_links)]

pub mod book;
pub mod evaluation;
pub mod search;
pub mod setup;
mod threads;
pub mod timing;
pub mod transposition_table;
pub mod uci;
pub mod util;

use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use lockfree::channel::RecvErr;
use lockfree::map::SharedIncin;
use log::info;
use log::trace;

use crate::search::exit_condition;
use crate::search::Message;
use crate::search::SEARCHING;
use crate::search::SEARCH_TO;
use crate::search::SEARCH_UNTIL;
use crate::setup::depth::Depth;
use crate::transposition_table::TranspositionTable;

#[derive(Debug)]
pub struct Engine {
    pub debug: bool,
    pub trace: bool,
    pub board: Board,
    // pub opening_book:
    pub tt: TranspositionTable,
    // pub slaves: SandPool,
}

impl Engine {
    pub fn new() -> Result<Self> {
        info!("creating engine at version {}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            debug: false,
            trace: false,
            board: Board::default(),
            tt: TranspositionTable::with_incin(SharedIncin::new()),
        })
    }

    pub fn set_search(&self, x: bool) {
        unsafe { SEARCHING = x }
    }

    pub fn set_search_to(&self, x: Depth) {
        unsafe { SEARCH_TO = x }
    }

    pub fn set_search_until(&self, until: Instant) {
        let until = until - Duration::from_millis(1);
        unsafe {
            SEARCH_UNTIL = Some(until);
            if SEARCH_UNTIL.is_some_and(|u| u < Instant::now()) {
                SEARCHING = false;
            }
        }
    }

    /// # begin setting up the engine
    /// 1. load opening book
    /// 2. load parameters from file
    /// 3. load endgame tablebases
    /// 4. ...
    pub fn setup(&mut self) -> Result<()> {
        // ...
        Ok(())
    }

    /// # Clean up after engine done
    /// 1. de-allocate any no-drop resources
    /// 2. save metrics
    pub fn clean_up(&mut self) -> Result<()> {
        // ...
        Ok(())
    }

    pub fn best_move(&mut self, to_depth: Depth, move_time: Duration) -> Result<ChessMove> {
        self.set_search_to(to_depth);
        self.set_search_until(Instant::now() + move_time);

        let mut move_listener = self.begin_search()?;
        loop {
            match move_listener.recv() {
                Ok(msg) => match msg {
                    Message::BestMove(mv, _val) => return Ok(mv),
                    Message::BestGuess(_mv, _val) => {}
                    Message::Info(si) => trace!(
                        "depth: {}, score: {}, nodes: {}",
                        si.depth.0,
                        si.score,
                        si.nodes
                    ),
                },
                Err(RecvErr::NoMessage) => {
                    thread::sleep(Duration::from_millis(50));
                    if exit_condition() {
                        self.set_search(false);
                    }
                }
                Err(RecvErr::NoSender) => {
                    return Err(anyhow::anyhow!("no sender"));
                }
            }
        }
    }
}
