//! Negamax search algorithm.
//!
//! https://en.wikipedia.org/wiki/Negamax

use std::sync::atomic::Ordering;
use std::sync::Arc;

use anyhow::Result;
use chess::Board;

use crate::evaluation::evaluate;
use crate::optlog;
use crate::opts::opts;
use crate::opts::setopts;
use crate::opts::Opts;
use crate::position::Position;
use crate::search::moveordering::ordered_moves;
use crate::search::SearchResult;
use crate::search::MV;
use crate::search::SEARCHING;
use crate::search::SEARCH_TO;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::transposition_table::TableImpl;
use crate::transposition_table::TT;

/// wrapper around [`SEARCHING`]
#[inline(always)]
pub fn searching() -> bool {
    SEARCHING.load(Ordering::Relaxed)
}

/// wrapper around [`SEARCH_TO`]
#[inline(always)]
pub fn search_to() -> Depth {
    Depth(SEARCH_TO.load(Ordering::Relaxed))
}

/// same as [`negamax`] but it sets the global options before running.
/// should be used for tests.
#[inline(always)]
pub fn ng_test(
    board: Board,
    to_depth: Depth,
    alpha: Value,
    beta: Value,
    set_opts: Opts,
) -> Result<SearchResult> {
    {
        setopts(set_opts)?;
    }
    let opt = opts()?;
    let table = TT::new();
    let position = Position::from(board);
    Ok(negamax(
        position,
        to_depth,
        alpha,
        beta,
        &opt,
        table.get_arc(),
    ))
}

/// mmmmmmmmmmmmm
pub fn negamax(
    pos: Position,
    to_depth: Depth,
    mut alpha: Value,
    beta: Value,
    opts: &Opts,
    table: Arc<TableImpl>,
) -> SearchResult {
    let moves = ordered_moves(&pos.chessboard);

    optlog!(search;trace;"ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}");
    optlog!(search;trace;"moves: {}", moves);

    /* source: https://en.wikipedia.org/wiki/Negamax
    (* Transposition Table Lookup; node is the lookup key for ttEntry *)
    ttEntry := transpositionTableLookup(node)
    if ttEntry.is_valid and ttEntry.depth ≥ depth then
        if ttEntry.flag = EXACT then
            return ttEntry.value
        else if ttEntry.flag = LOWERBOUND then
            α := max(α, ttEntry.value)
        else if ttEntry.flag = UPPERBOUND then
            β := min(β, ttEntry.value)

        if α ≥ β then
            return ttEntry.value
    */

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
        let mut deeper = -negamax(
            pos.make_move(*mv),
            to_depth - 1,
            -beta,
            -alpha,
            opts,
            table.clone(),
        );
        total_nodes += deeper.nodes_searched + 1;

        if !searching() {
            optlog!(search;trace;"searching() == false, breaking early");
            deeper.next_position_value = evaluate(&pos, &moves);
            deeper.nodes_searched = total_nodes;
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

        if opts.use_ab && alpha >= beta {
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
