//! # Sandy Engine
//! all logic for the engine lies in this lib
#![deny(rustdoc::broken_intra_doc_links)]

pub mod book;
pub mod debug;
pub mod evaluation;
pub mod opts;
pub mod search;
pub mod setup;
pub mod timing;
pub mod transposition_table;
pub mod uci;
pub mod util;

use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use lockfree::channel::RecvErr;
use log::info;
use log::trace;

use crate::search::exit_condition;
use crate::search::Message;
use crate::search::SEARCHING;
use crate::search::SEARCH_TO;
use crate::search::SEARCH_UNTIL;
use crate::setup::depth::Depth;

#[derive(Debug)]
pub struct Engine {
    pub board: Board,
}

impl Engine {
    pub fn new() -> Result<Self> {
        info!("creating engine at version {}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            board: Board::default(),
        })
    }

    pub fn set_search(&self, x: bool) {
        SEARCHING.store(x, Ordering::Relaxed);
    }

    pub fn set_search_to(&self, x: Depth) {
        SEARCH_TO.store(x.0, Ordering::Relaxed);
    }

    pub fn set_search_until(&self, until: Instant) -> Result<()> {
        let until = until - Duration::from_millis(1);
        let _ = SEARCH_UNTIL
            .try_write()
            .map_err(|e| anyhow!("SEARCH_UNTIL lock error: {e}"))?
            .insert(until);
        if SEARCH_UNTIL
            .try_read()
            .map_err(|e| anyhow!("SEARCH_UNTIL lock error: {e}"))?
            .is_some_and(|u| u < Instant::now())
        {
            SEARCHING.store(false, Ordering::Relaxed);
        }
        Ok(())
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
        self.set_search_until(Instant::now() + move_time)?;

        let mut move_listener = self.begin_search()?;

        let mut best = None;
        loop {
            match move_listener.recv() {
                Ok(msg) => match msg {
                    Message::BestMove(mv) => {
                        trace!("new bestmove {}/{}", mv.0, mv.1);
                        best = Some(mv);
                    }
                    Message::Ponder(_) => {}
                    Message::BestGuess(_) => {}
                    Message::Info(si) => trace!(
                        "depth: {}, score: {}, nodes: {}",
                        si.depth.0,
                        si.score,
                        si.nodes
                    ),
                },
                Err(RecvErr::NoMessage) => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(RecvErr::NoSender) => {
                    return if let Some(mv) = best {
                        Ok(mv.0)
                    } else {
                        Err(anyhow!("sender dropped before best move found"))
                    }
                }
            }
            if exit_condition() {
                self.set_search(false);
                if let Some(mv) = best {
                    return Ok(mv.0);
                }
            }
        }
    }
}
