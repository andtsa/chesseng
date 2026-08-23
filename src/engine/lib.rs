//! # Sandy Engine
//! all logic for the engine lies in this lib
#![deny(rustdoc::broken_intra_doc_links)]

pub mod book;
pub mod debug;
pub mod engine_opts;
pub mod evaluation;
pub mod move_generation;
pub mod opts;
pub mod position;
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

use anyhow::Result;
use anyhow::anyhow;
use chess::ChessMove;
use chess::MoveGen;
use engine_opts::EngineOpts;
use lockfree::channel::RecvErr;
use log::info;
use log::trace;
use opts::opts;

use crate::position::Position;
use crate::position::is_irreversible;
use crate::search::Message;
use crate::search::SEARCH_TO;
use crate::search::SEARCH_UNTIL;
use crate::search::SEARCHING;
use crate::search::exit_condition;
use crate::setup::depth::Depth;
use crate::transposition_table::TT;
use crate::transposition_table::TranspositionTable;

/// this is why you're here, right?
#[derive(Debug)]
pub struct Engine {
    /// the board the engine will think on
    pub board: Position,
    /// the transposition table
    pub table: TT,
    /// hashes of the positions played since the last irreversible move,
    /// oldest first. used to detect repetition.
    pub history: Vec<u64>,
    /// this instance's options
    pub eng_opts: EngineOpts,
}

impl Engine {
    /// create a new engine!
    pub fn new() -> Result<Self> {
        info!("creating engine at version {}", env!("CARGO_PKG_VERSION"));

        Ok(Self {
            board: Default::default(),
            table: TT::new(),
            history: Vec::new(),
            eng_opts: opts()?.engine_opts,
        })
    }

    /// register a new move that has been played in the game.
    pub fn make_move(&mut self, mv: ChessMove) {
        let previous = self.board.chessboard;
        self.board = self.board.make_move(mv);

        // positions from before an irreversible move can never occur again, so
        // they are dropped. this keeps the history short enough that scanning
        // it at every search node is free.
        if is_irreversible(&previous, &self.board.chessboard, mv) {
            self.history.clear();
        }

        self.log_position(self.board.clone());
    }

    /// add a new position to the engine history.
    pub fn log_position(&mut self, pos: Position) {
        self.history.push(pos.chessboard.get_hash());
    }

    /// forget the game history and start it again from the current board.
    /// used whenever a new position is set over UCI.
    pub fn reset_history(&mut self) {
        self.history.clear();
        self.log_position(self.board.clone());
    }

    /// set the global [`SEARCHING`]
    pub fn set_search(&self, x: bool) {
        SEARCHING.store(x, Ordering::Relaxed);
    }

    /// set the global [`SEARCH_TO`]
    pub fn set_search_to(&self, x: Depth) {
        SEARCH_TO.store(x.0, Ordering::Relaxed);
    }

    /// set the global [`SEARCH_UNTIL`]
    pub fn set_search_until(&self, until: Instant) -> Result<()> {
        let until = until - Duration::from_millis(1);
        let _ = SEARCH_UNTIL
            .write()
            .map_err(|e| anyhow!("SEARCH_UNTIL [set,write] lock error: {e}"))?
            .insert(until);
        if SEARCH_UNTIL
            .read()
            .map_err(|e| anyhow!("SEARCH_UNTIL [set,read] lock error: {e}"))?
            .is_some_and(|u| u < Instant::now())
        {
            // the deadline is already behind us, so the search must not run.
            self.set_search(false);
        }
        Ok(())
    }

    /// resize the transposition table
    pub fn resize_table(&mut self, size: usize) -> Result<usize> {
        Ok(self
            .table
            .get()
            .write()
            .map_err(|e| anyhow!("table lock error: {e}"))?
            .resize(size))
    }

    /// forget every transposition table entry, so that a new game doesn't
    /// inherit the previous one's results.
    pub fn clear_table(&mut self) -> Result<()> {
        self.table
            .get()
            .write()
            .map_err(|e| anyhow!("table lock error: {e}"))?
            .clear();
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

    /// get the best move from this position using the current thread
    pub fn best_move(&mut self, to_depth: Depth, move_time: Duration) -> Result<ChessMove> {
        self.set_search_to(to_depth);
        self.set_search_until(Instant::now() + move_time)?;

        // as in [`Self::uci_go`], keep a legal move aside so that a search which
        // produces nothing still yields a move to play rather than an error.
        let fallback = MoveGen::new_legal(&self.board.chessboard).next();

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
                        si.depth.0, si.score, si.nodes
                    ),
                },
                Err(RecvErr::NoMessage) => {
                    thread::sleep(Duration::from_millis(50));
                }
                Err(RecvErr::NoSender) => {
                    return match best.map(|mv| mv.0).or(fallback) {
                        Some(mv) => Ok(mv),
                        None => Err(anyhow!("no legal moves in this position")),
                    };
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
