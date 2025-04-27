//! a quiescence search implementation, used to ensure the static evaluation
//! isn't ran on active positions with lots of exchanges
//!
//! + https://en.wikipedia.org/wiki/Quiescence_search
//! + https://www.chessprogramming.org/Quiescence_Search
//! + https://www.chessprogramming.org/Horizon_Effect

use chess::MoveGen;

use super::MV;
use super::SearchOptions;
use super::SearchResult;
use crate::evaluation::evaluate;
use crate::move_generation::prio_iterator;
use crate::opts::Opts;
use crate::position::Position;
use crate::setup::depth::Depth;
use crate::setup::values::Value;

///    nodes ← 1
///    stand_pat ← EVALUATE(position, inCheck = false)
///
///    // 1) stand-pat test
///    if stand_pat ≥ β:
///        return SearchResult(value=stand_pat, nodes=nodes)
///
///    α ← max(α, stand_pat)
///
///    // 2) generate only tactical moves (captures, promotions, checks-only if
/// you like)    captures ← GENERATE_CAPTURES(position)
///    ORDER_MOVES(captures, heuristics)
///
///    for move in captures:
///        childPos ← position.make_move(move)
///        // recurse, note the negation
///        childRes ← QUIESCENCE(childPos, -β, -α, searchOptions, opts)
///        nodes += childRes.nodes
///        score ← -childRes.value
///
///        if score ≥ β:
///            return SearchResult(value=score, nodes=nodes)
///
///        α ← max(α, score)
///
///    return SearchResult(value=α, nodes=nodes)
pub fn quiescence(
    pos: Position,
    mut alpha: Value,
    beta: Value,
    _search_options: SearchOptions,
    _opts: &Opts,
) -> SearchResult {
    let mut nodes = 1;

    let mgen = MoveGen::new_legal(&pos.chessboard);
    let mut pgen = prio_iterator(mgen, &pos.chessboard, &[]);

    let first_move = pgen.next();
    let stand_pat = evaluate(&pos, first_move.is_none());

    // 1. stand-pat test
    if stand_pat >= beta {
        return SearchResult {
            pv: Vec::new(),
            next_position_value: stand_pat,
            nodes_searched: nodes,
            tb_hits: 0,
            depth: Depth::ZERO,
        };
    }

    alpha = alpha.max(stand_pat);

    // 2. generate only tactical moves
    let captures = pgen.generate_captures();

    let mut pv = None;
    let mut max_depth = Depth::ZERO;
    for mv in first_move.iter().chain(captures.iter()) {
        let child = -quiescence(pos.make_move(*mv), -beta, -alpha, _search_options, _opts);

        max_depth = max_depth.max(child.depth);
        nodes += child.nodes_searched;

        if child.next_position_value >= beta {
            return SearchResult {
                pv: vec![MV(*mv, child.next_position_value)],
                next_position_value: child.next_position_value,
                nodes_searched: nodes,
                depth: max_depth + 1,
                tb_hits: 0,
            };
        }

        if child.next_position_value >= alpha {
            alpha = child.next_position_value;
            pv = Some(*mv);
        }
    }

    let pv_move = if let Some(pm) = pv {
        vec![MV(pm, alpha)]
    } else {
        vec![]
    };

    SearchResult {
        pv: pv_move,
        next_position_value: alpha,
        nodes_searched: nodes,
        depth: max_depth,
        tb_hits: 0,
    }
}
