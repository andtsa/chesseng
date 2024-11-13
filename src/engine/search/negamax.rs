use std::sync::atomic::Ordering;

use anyhow::Result;
use chess::Board;

use crate::evaluation::evaluate;
use crate::optlog;
use crate::opts::opts;
use crate::opts::setopts;
use crate::opts::Opts;
use crate::search::moveordering::ordered_moves;
use crate::search::SearchResult;
use crate::search::MV;
use crate::search::SEARCHING;
use crate::search::SEARCH_TO;
use crate::setup::depth::Depth;
use crate::setup::values::Value;

#[inline(always)]
pub fn searching() -> bool {
    SEARCHING.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn search_to() -> Depth {
    Depth(SEARCH_TO.load(Ordering::Relaxed))
}

#[inline(always)]
pub fn ng_test(
    pos: Board,
    to_depth: Depth,
    alpha: Value,
    beta: Value,
    opts: Opts,
) -> Result<SearchResult> {
    {
        setopts(opts)?;
    }
    Ok(negamax(pos, to_depth, alpha, beta))
}

pub fn negamax(pos: Board, to_depth: Depth, mut alpha: Value, beta: Value) -> SearchResult {
    let moves = ordered_moves(&pos);

    optlog!(search;trace;"ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}");
    optlog!(search;trace;"moves: {}", moves);

    if to_depth == Depth::ZERO || moves.is_empty() {
        let ev = evaluate(&pos, &moves);
        optlog!(search;trace;"return eval: {:?}", ev);
        return SearchResult {
            pv: vec![],
            next_position_value: ev,
            nodes_searched: 1,
        };
    }

    let mut best = None;
    let mut pv = vec![];
    let mut total_nodes = 0;

    for mv in moves.0.iter() {
        let mut deeper = -negamax(pos.make_move_new(*mv), to_depth - 1, -beta, -alpha);
        total_nodes += deeper.nodes_searched + 1;

        if !searching() {
            optlog!(search;trace;"searching() == false, breaking early");
            deeper.new_eval(evaluate(&pos, &moves));
            deeper.set_nodes(total_nodes);
            return deeper;
        }

        if best
            .as_ref()
            .is_none_or(|b: &MV| b.1 < deeper.next_position_value)
        {
            best = Some(MV(*mv, deeper.next_position_value));
            // Build the principal variation by prepending the current move
            pv = vec![MV(*mv, deeper.next_position_value)];
            pv.extend(deeper.pv);
        }
        alpha = alpha.max(deeper.next_position_value);

        if opts().unwrap().use_ab && alpha >= beta {
            optlog!(search;trace;"alpha {alpha:?} >= beta {beta:?}");
            break;
        }
    }

    optlog!(search;trace;"return max_val: {:?}", best);

    SearchResult {
        pv,
        next_position_value: best.as_ref().map_or(Value::MIN, |b| b.1),
        nodes_searched: total_nodes,
    }
}

#[cfg(test)]
#[path = "tests/negatest.rs"]
mod tests;
