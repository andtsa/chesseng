//! Negamax search algorithm.
//!
//! https://en.wikipedia.org/wiki/Negamax

use std::sync::atomic::Ordering;

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
use crate::setup::depth::ONE_PLY;
use crate::setup::values::Value;
use crate::transposition_table::EvalBound;
use crate::transposition_table::ShareImpl;
use crate::transposition_table::TEntry;
use crate::transposition_table::TableAccess;
use crate::transposition_table::TranspositionTable;
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
    ng_bench(position, to_depth, alpha, beta, opt, &table)
}

/// same as [`negamax`], but with a fixed signature to be used across benchmarks
///
/// unlike [`ng_test`], this function does not and should not do any extra work
/// or allocations, in order to preserve benchmark accuracy.
pub fn ng_bench(
    position: Position,
    to_depth: Depth,
    alpha: Value,
    beta: Value,
    opt: Opts,
    tt: &TT,
) -> Result<SearchResult> {
    Ok(negamax(position, to_depth, alpha, beta, &opt, &tt.get()))
}

/// mmmmmmmmmmmmm
pub fn negamax(
    pos: Position,
    to_depth: Depth,
    mut alpha: Value,
    mut beta: Value,
    opts: &Opts,
    table: &ShareImpl,
) -> SearchResult {
    let moves = ordered_moves(&pos.chessboard);

    optlog!(search;trace;"ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}");
    optlog!(search;trace;"moves: {}", moves);

    /* source: https://en.wikipedia.org/wiki/Negamax */
    let alpha_orig = alpha;
    if opts.use_tt {
        let current_hash = pos.chessboard.get_hash(); // change
        if let Ok(Some(tt_entry)) = table.read().map(|l| l.get(current_hash)) {
            if tt_entry.is_valid() && tt_entry.depth() >= to_depth {
                match tt_entry.bound() {
                    EvalBound::Exact => return tt_entry.search_result(),
                    EvalBound::LowerBound => {
                        alpha = alpha.max(tt_entry.search_result().next_position_value)
                    }
                    EvalBound::UpperBound => {
                        beta = beta.min(tt_entry.search_result().next_position_value)
                    }
                }
                if alpha >= beta {
                    return tt_entry.search_result();
                }
            }
        }
    }

    if to_depth == Depth::ZERO || moves.is_empty() {
        let ev = evaluate(&pos, &moves);
        optlog!(search;trace;"return eval: {:?}", ev);
        return SearchResult {
            pv: vec![],
            next_position_value: ev,
            nodes_searched: 1,
            tb_hits: 0,
            depth: ONE_PLY,
        };
    }

    // adjust depth based on heuristics
    let next_depth = to_depth + (moves.0.len() <= 3) + (moves.0.len() <= 5) - 1;

    let mut best = None;
    let mut pv = vec![];
    let mut total_nodes = 0;
    let mut tb_hits = 0;
    let mut max_depth = Depth::ZERO;

    for mv in moves.0.iter() {
        let mut deeper = -negamax(pos.make_move(*mv), next_depth, -beta, -alpha, opts, table);
        total_nodes += deeper.nodes_searched + 1;
        tb_hits += deeper.tb_hits;
        max_depth = max_depth.max(deeper.depth);

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

    let mut best_value = best.as_ref().map_or(Value::MIN, |b| b.1);
    if best_value >= Value::MATE_IN_MAX_PLY {
        best_value.0 = best_value.0.saturating_sub(1);
    } else if best_value <= Value::MATED_IN_MAX_PLY {
        best_value.0 = best_value.0.saturating_add(1);
    }

    let search_result = SearchResult {
        pv,
        next_position_value: best_value,
        nodes_searched: total_nodes,
        tb_hits,
        depth: max_depth + ONE_PLY,
    };

    /* from https://en.wikipedia.org/wiki/Negamax */
    if opts.use_tt {
        let bound = if search_result.next_position_value <= alpha_orig {
            EvalBound::UpperBound
        } else if search_result.next_position_value >= beta {
            EvalBound::LowerBound
        } else {
            EvalBound::Exact
        };
        let current_hash = pos.chessboard.get_hash(); // change
        let entry = TEntry::new_from_result(current_hash, to_depth, &search_result, bound);
        if let Ok(mut lock) = table.share().write() {
            lock.insert(current_hash, entry)
        };
    }

    optlog!(search;trace;"return max_val: {:?}", best);

    search_result
}

#[cfg(test)]
#[path = "tests/negatest.rs"]
mod tests;
