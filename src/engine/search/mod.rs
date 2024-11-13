mod main_search;
pub mod moveordering;
pub mod negamax;

use std::fmt::Display;
use std::ops::Neg;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

use anyhow::anyhow;
use chess::Board;
use chess::ChessMove;
use lockfree::channel::spsc::Sender;
use log::debug;

use crate::setup::depth::Depth;
use crate::setup::values::Value;

pub const SEARCH_THREADS: usize = 1;

pub static SEARCH_UNTIL: RwLock<Option<Instant>> = RwLock::new(None);
pub static SEARCH_TO: AtomicU16 = AtomicU16::new(0);
pub static SEARCHING: AtomicBool = AtomicBool::new(false);
pub static EXIT: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
pub struct MV(pub ChessMove, pub Value);

pub struct RootNode {
    pub board: Board,
    pub pv: Vec<MV>,
    pub eval: Value,
    pub previous_eval: Value,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub pv: Vec<MV>,
    pub next_position_value: Value,
    pub nodes_searched: u32,
    pub tt_hits: u32,
}

#[derive(Debug)]
pub enum Message {
    BestMove(MV),
    Ponder(MV),
    BestGuess(MV),
    Info(SearchInfo),
}

#[derive(Debug)]
pub struct SearchInfo {
    pub depth: Depth,
    pub score: Value,
    pub nodes: u32,
    pub time: Duration,
    pub pv: Vec<MV>,
    pub tt_hits: u32,
}

pub fn exit_condition() -> bool {
    if EXIT.load(Ordering::Relaxed)
        || SEARCH_UNTIL
            .try_read()
            .map_err(|e| anyhow!("SEARCH_UNTIL lock error: {e}"))
            .unwrap()
            .is_some_and(|u| u < Instant::now())
    {
        SEARCHING.store(false, Ordering::Relaxed);
        true
    } else {
        false
    }
}

fn info(
    publisher: &mut Sender<Message>,
    target_depth: Depth,
    best_value: Value,
    total_nodes: u32,
    tt_hits: u32,
    el: Duration,
    pv: &[MV],
) {
    if let Err(e) = publisher.send(Message::Info(SearchInfo {
        depth: target_depth,
        score: best_value,
        nodes: total_nodes,
        time: el,
        pv: pv.to_vec(),
        tt_hits,
    })) {
        debug!("error sending info message: {:?}", e);
    }
}

fn send(publisher: &mut Sender<Message>, msg: Message) {
    if let Err(e) = publisher.send(msg) {
        debug!("error sending message: {:?}", e);
    }
}

impl SearchResult {
    pub fn new_eval(&mut self, ev: Value) {
        self.next_position_value = ev;
    }
    pub fn add_nodes(&mut self, nodes: u32) {
        self.nodes_searched += nodes;
    }
    pub fn add_tt_hits(&mut self, nodes: u32) {
        self.tt_hits += nodes;
    }
    pub fn set_nodes(&mut self, nodes: u32) {
        self.nodes_searched = nodes;
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
        write!(f, "{} [{:+}]", self.0, self.1 .0)
    }
}

impl Default for MV {
    fn default() -> Self {
        MV(ChessMove::default(), Value::ZERO)
    }
}
