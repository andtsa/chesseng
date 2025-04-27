//! Negamax search algorithm.
//!
//! https://en.wikipedia.org/wiki/Negamax
#![allow(unused_labels)]

use std::sync::atomic::Ordering;

use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;

use super::SearchOptions;
use crate::evaluation::evaluate;
use crate::move_generation::prio_iterator;
use crate::optlog;
use crate::opts::Opts;
use crate::opts::opts;
use crate::opts::setopts;
use crate::position::Position;
use crate::search::MV;
use crate::search::SEARCH_TO;
use crate::search::SEARCHING;
use crate::search::SearchResult;
use crate::search::quiescence::quiescence;
use crate::setup::depth::Depth;
use crate::setup::depth::ONE_PLY;
use crate::setup::values::Value;
use crate::transposition_table::EvalBound;
use crate::transposition_table::ShareImpl;
use crate::transposition_table::TEntry;
use crate::transposition_table::TT;
use crate::transposition_table::TableAccess;
use crate::transposition_table::TranspositionTable;

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
    Ok(negamax(
        position,
        to_depth,
        alpha,
        beta,
        Default::default(),
        &opt,
        &tt.get(),
    ))
}

/// mmmmmmmmmmmmm
pub fn negamax(
    pos: Position,
    to_depth: Depth,
    mut alpha: Value,
    mut beta: Value,
    mut search_options: SearchOptions,
    opts: &Opts,
    table: &ShareImpl,
) -> SearchResult {
    optlog!(search;trace;"ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}");

    let current_hash = pos.chessboard.get_hash();
    if search_options
        .history
        .iter()
        .filter(|x| **x == current_hash)
        .count()
        >= 2
    {
        // threefold repetition
        let ev = evaluate(&pos, true);
        return SearchResult {
            pv: vec![],
            next_position_value: ev,
            nodes_searched: 1,
            tb_hits: 0,
            depth: ONE_PLY,
        };
    }

    // the initial move generator
    let mut base_gen = MoveGen::new_legal(&pos.chessboard);
    // slice for already generated moves.
    let mut pre_generated: [Option<ChessMove>; 4] = [None; 4];

    // we are mated!
    let out_of_moves = base_gen.len() == 0;

    /* source: https://en.wikipedia.org/wiki/Negamax */
    let alpha_orig = alpha;
    if opts.use_tt {
        if let Ok(Some(tt_entry)) = table.read().map(|l| l.get(current_hash)) {
            if tt_entry.is_valid() {
                if tt_entry.depth() >= to_depth {
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
                pre_generated[0] = Some(tt_entry.mv());
                base_gen.remove_move(tt_entry.mv());
            }
        }
    }

    if to_depth == Depth::ZERO {
        // leaf node → do quiescence, not raw eval
        return quiescence(pos, alpha, beta, search_options, opts);
    }

    // ordering wrapper around the move generation iterator
    let mut mgen = prio_iterator(base_gen, &pos.chessboard, &[]);

    if out_of_moves {
        let ev = evaluate(&pos, out_of_moves);
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
    let next_depth = if search_options.extensions >= Depth::MAX_EXTEND {
        Depth::ZERO
    } else {
        // check the first 3 moves generated from the current position,
        pre_generated[1] = mgen.next();
        pre_generated[2] = mgen.next();
        // if the 4th one is [`None`], then <=> moves.len() <= 3,
        pre_generated[3] = mgen.next();
        // depth implements addition with booleans.
        to_depth + (pre_generated[3].is_none()) - 1
        // if theres 3 moves or less, search +1 level deeper
    };

    search_options.history.rotate_right(1);
    search_options.history[0] = current_hash;
    search_options = SearchOptions {
        extensions: search_options
            .extensions
            .max(search_options.extensions + next_depth + 1 - to_depth),
        history: search_options.history,
    };

    let mut best = None;
    let mut pv = vec![];
    let mut total_nodes = 0;
    let mut tb_hits = 0;
    let mut max_depth = Depth::ZERO;

    'next_moves: for mv in pre_generated.into_iter().flatten().chain(mgen) {
        let mut deeper = -negamax(
            pos.make_move(mv),
            next_depth,
            -beta,
            -alpha,
            search_options,
            opts,
            table,
        );
        total_nodes += deeper.nodes_searched + 1;
        tb_hits += deeper.tb_hits;
        max_depth = max_depth.max(deeper.depth);

        if !searching() {
            optlog!(search;trace;"searching() == false, breaking early");
            deeper.next_position_value = evaluate(&pos, out_of_moves);
            deeper.nodes_searched = total_nodes;
            return deeper;
        }

        if best
            .as_ref()
            .is_none_or(|b: &MV| b.1 < deeper.next_position_value)
        {
            best = Some(MV(mv, deeper.next_position_value));
            // Build the principal variation by prepending the current move
            pv = vec![MV(mv, deeper.next_position_value)];
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
