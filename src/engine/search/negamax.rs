use std::ops::Neg;

use chess::Board;

use crate::evaluation::evaluate;
use crate::search::moveordering::ordered_moves;
use crate::search::SEARCHING;
use crate::setup::depth::Depth;
use crate::setup::values::Value;

pub struct SearchResult {
    // pub best_move: Value,
    pub next_position_value: Value,
    pub nodes_searched: usize,
}

/// DeBug OPTions for the search
#[derive(Clone, Copy)]
pub struct DbOpt {
    pub debug: bool,
    pub trace: bool,
    pub ab: bool,
}

#[inline(always)]
fn searching() -> bool {
    unsafe { SEARCHING }
}

#[inline(always)]
#[allow(dead_code)]
pub fn ngm(pos: Board, to_depth: Depth, alpha: Value, beta: Value) -> SearchResult {
    negamax(
        pos,
        to_depth,
        alpha,
        beta,
        DbOpt {
            debug: false,
            trace: false,
            ab: true,
        },
    )
}

pub fn negamax(
    pos: Board,
    to_depth: Depth,
    mut alpha: Value,
    beta: Value,
    db: DbOpt,
) -> SearchResult {
    let moves = ordered_moves(&pos);
    if db.trace {
        println!("ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}");
        println!("moves: {}", moves);
    }
    if to_depth == Depth::ZERO || moves.is_empty() {
        let ev = evaluate(&pos, &moves);
        if db.trace {
            println!("return eval: {:?}", ev);
        }
        return SearchResult {
            next_position_value: ev,
            nodes_searched: 1,
        };
    }

    let mut max_val = Value::MIN;
    let mut total_nodes = 0;

    for mv in moves.0.iter() {
        let mut deeper = -negamax(pos.make_move_new(*mv), to_depth - 1, -beta, -alpha, db);
        total_nodes += deeper.nodes_searched + 1;

        if !searching() {
            if db.trace {
                println!("searching() == false, breaking early");
            }
            deeper.new_eval(evaluate(&pos, &moves));
            deeper.set_nodes(total_nodes);
            return deeper;
        }

        max_val = max_val.max(deeper.next_position_value);
        alpha = alpha.max(deeper.next_position_value);

        if db.ab && alpha >= beta {
            if db.trace {
                println!("alpha {alpha:?} >= beta {beta:?}");
            }
            break;
        }
    }

    if db.trace {
        println!("return max_val: {:?}", max_val);
    }

    SearchResult {
        next_position_value: max_val,
        nodes_searched: total_nodes,
    }
}

impl SearchResult {
    pub fn new_eval(&mut self, ev: Value) {
        self.next_position_value = ev;
    }
    pub fn add_nodes(&mut self, nodes: usize) {
        self.nodes_searched += nodes;
    }
    pub fn set_nodes(&mut self, nodes: usize) {
        self.nodes_searched = nodes;
    }
}

impl Neg for SearchResult {
    type Output = SearchResult;

    fn neg(self) -> Self::Output {
        SearchResult {
            next_position_value: -self.next_position_value,
            nodes_searched: self.nodes_searched,
        }
    }
}

#[cfg(test)]
#[path = "tests/negatest.rs"]
mod tests;
