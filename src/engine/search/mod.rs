//! The search module contains the search logic for the engine.
mod main_search;
pub mod negamax;

use std::fmt::Display;
use std::ops::Neg;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use anyhow::anyhow;
use chess::ChessMove;
use lockfree::channel::spsc::Sender;
use log::debug;

use crate::position::Position;
use crate::setup::depth::Depth;
use crate::setup::values::Value;

/// how many os threads should the search use?
pub const SEARCH_THREADS: usize = 8;

/// when should the search stop?
pub static SEARCH_UNTIL: RwLock<Option<Instant>> = RwLock::new(None);
/// what's the maximum depth the search should go to?
pub static SEARCH_TO: AtomicU16 = AtomicU16::new(0);
/// is the search running?
pub static SEARCHING: AtomicBool = AtomicBool::new(false);
/// should the search exit?
pub static EXIT: AtomicBool = AtomicBool::new(false);

/// A move and its value
#[derive(Debug, Clone, Copy)]
pub struct MV(pub ChessMove, pub Value);

/// The root node of the search
#[derive(Debug)]
pub struct RootNode {
    /// the current board state
    pub board: Position,
    /// the principal variation
    pub pv: Vec<MV>,
    /// the current evaluation of the root node
    pub eval: Value,
    /// the previous evaluation of the root node
    pub previous_eval: Value,
}

/// The result of a single negamax search call
#[derive(Debug, Default)]
pub struct SearchResult {
    /// The principal variation
    pub pv: Vec<MV>,
    /// The value of the best move found
    pub next_position_value: Value,
    /// how many nodes were searched by this call and its recursive sub-calls
    pub nodes_searched: u32,
    /// how many transposition table hits were made
    pub tb_hits: u32,
    /// actual depth the search reached
    pub depth: Depth,
}

/// A message that can be sent from the search threads to the main/UCI thread
#[derive(Debug)]
pub enum Message {
    /// best move from a full search to a certain depth
    BestMove(MV),
    /// UCI ponder move
    Ponder(MV),
    /// the next best guess from a non-fully-searched depth
    BestGuess(MV),
    /// A UCI info message
    Info(SearchInfo),
}

/// a UCI info message during a search
#[derive(Debug)]
pub struct SearchInfo {
    /// The depth that was reached (in plies)
    pub depth: Depth,
    /// selective search depth in plies
    pub sel_depth: Depth,
    /// this for the multi pv mode.
    /// for the best move/pv add "multipv 1" in the string when you send the pv.
    /// in k-best mode always send all k variants in k strings together.
    pub multi_pv: usize,
    /// The score of the best move found from the root position
    pub score: Value,
    /// The number of nodes that was searched for this depth
    pub nodes: u32,
    /// number 0-1000 of how full the transposition table is
    pub hashfull: usize,
    /// how many table base hits were made during the search
    pub tb_hits: u32,
    /// The time it took to search this depth
    pub time: Duration,
    /// The principal variation
    pub pv: Vec<MV>,
}

/// information for a search root to pass to its children, in order to inform
/// dependent heuristics.
#[derive(Debug, Copy, Clone, Default)]
pub struct SearchOptions {
    /// how many times have we already extended the search? this is necessary to
    /// ensure the recursion terminates, and to prevent stack overflow.
    pub extensions: Depth,

    /// previously played position that would cause draw by threefold repetition
    pub history: [u64; 6],
}

/// wrapper around [`SEARCH_UNTIL`]
pub fn search_until() -> Option<Instant> {
    *SEARCH_UNTIL
        .read()
        .map_err(|e| anyhow!("SEARCH_UNTIL [fn,read] lock error: {e}"))
        // SAFETY: this only panics if another thread has already panicked,
        // and if any thread panics then the process exits anyway,
        // so this situation is unreachable.
        .unwrap()
}

/// has the exit condition been reached?
pub fn exit_condition() -> bool {
    if EXIT.load(Ordering::Relaxed) || search_until().is_some_and(|u| u < Instant::now()) {
        SEARCHING.store(false, Ordering::Relaxed);
        true
    } else {
        false
    }
}

/// shortcut for sending UCI info to the main thread
#[allow(clippy::too_many_arguments)]
fn info(
    publisher: &mut Sender<Message>,
    target_depth: Depth,
    best_value: Value,
    total_nodes: u32,
    el: Duration,
    hashfull: usize,
    tb_hits: u32,
    sel_depth: Depth,
    multi_pv: usize,
    pv: &[MV],
) {
    if let Err(e) = publisher.send(Message::Info(SearchInfo {
        depth: target_depth,
        sel_depth,
        multi_pv,
        score: best_value,
        nodes: total_nodes,
        time: el,
        hashfull,
        tb_hits,
        pv: pv.to_vec(),
    })) {
        debug!("error sending info message: {:?}", e);
    }
}

/// shortcut for sending a message to the main thread
fn send(publisher: &mut Sender<Message>, msg: Message) {
    if let Err(e) = publisher.send(msg) {
        debug!("error sending message: {:?}", e);
    }
}

impl Neg for SearchResult {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.next_position_value = -self.next_position_value;
        self
    }
}

impl Display for MV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{:+}]", self.0, self.1.0)
    }
}

impl Default for MV {
    fn default() -> Self {
        MV(ChessMove::default(), Value::ZERO)
    }
}
