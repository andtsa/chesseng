use std::sync::atomic::Ordering;
use std::sync::RwLock;

use anyhow::Result;
use chess::Board;
use log::error;

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
use crate::transposition_table::bounds::EvalBound;
use crate::transposition_table::{table_mut_ref, TT_INITIALISED};
use crate::transposition_table::table_ref;
use crate::transposition_table::TableEntry;
use crate::transposition_table::TranspositionTable;
use crate::transposition_table::TT;

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

pub fn negamax(pos: Board, to_depth: Depth, mut alpha: Value, mut beta: Value) -> SearchResult {
    let original_alpha = alpha;
    let original_beta = beta;
    let mut tt_hits = 0;

    if opts().unwrap().use_tt {
        {
            match table_ref().try_read() {
                Ok(lock) => {
                    if let Some(hit) = lock.lookup(pos.get_hash()) {
                        if hit.depth() >= to_depth {
                            tt_hits += 1;
                            match hit.bound() {
                                EvalBound::Exact => {
                                    return {
                                        SearchResult {
                                            pv: if hit.is_pv() {
                                                vec![hit.mv_struct()]
                                            } else {
                                                vec![]
                                            },
                                            next_position_value: hit.eval(),
                                            nodes_searched: 1,
                                            tt_hits,
                                        }
                                    }
                                }
                                EvalBound::LowerBound => alpha = alpha.max(hit.eval()),
                                EvalBound::UpperBound => beta = beta.min(hit.eval()),
                            }
                            if alpha >= beta {
                                return SearchResult {
                                    pv: if hit.is_pv() {
                                        vec![hit.mv_struct()]
                                    } else {
                                        vec![]
                                    },
                                    next_position_value: hit.eval(),
                                    nodes_searched: 1,
                                    tt_hits,
                                };
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("tt lock poisoned: {e}");
                }
            }
        }
    }

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
            tt_hits,
        };
    }

    let mut best = None;
    let mut pv = vec![];
    let mut total_nodes = 0;

    for mv in moves.0.iter() {
        let mut deeper = -negamax(pos.make_move_new(*mv), to_depth - 1, -beta, -alpha);
        total_nodes += deeper.nodes_searched + 1;
        tt_hits += deeper.tt_hits;

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

    let this_search_result = SearchResult {
        pv,
        next_position_value: best.as_ref().map_or(Value::MIN, |b| b.1),
        nodes_searched: total_nodes,
        tt_hits,
    };

    if opts().unwrap().use_tt {
        if let Some(b) = best {
            let bound = if this_search_result.next_position_value <= original_alpha {
                EvalBound::UpperBound
            } else if this_search_result.next_position_value >= original_beta {
                EvalBound::LowerBound
            } else {
                EvalBound::Exact
            };

            {
                match table_mut_ref().try_write() {
                    Ok(mut lock) => lock.insert(
                        pos.get_hash(),
                        TableEntry::pack(
                            pos.get_hash(),
                            this_search_result.next_position_value,
                            to_depth,
                            b.0,
                            bound,
                            false,
                        ),
                    ),
                    Err(e) => {
                        error!("tt write lock poisoned: {e}");
                    }
                }
            }
        }
    }

    opts().stp(&format!("return max_val: {:?}", best));
    this_search_result
}

#[cfg(test)]
#[path = "tests/negatest.rs"]
mod tests;
