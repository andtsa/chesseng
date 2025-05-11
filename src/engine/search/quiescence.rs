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
use crate::engine_opts::EngineOpts;
use crate::evaluation::evaluate;
use crate::move_generation::prio_iterator;
use crate::position::Position;
use crate::setup::depth::Depth;
use crate::setup::values::Value;

/// make sure that we only statically evaluate after all capture moves have been
/// played (end of piece exchange)
///
/// NOTE: this function does not search moves that put a player into check,
/// even though they are usually considered strategic (non-quiet) moves!
/// This is solely because I currently have no efficient way of generating
/// checks, while generating captures can be done independently of quiet moves.
pub fn quiescence(
    pos: Position,
    mut alpha: Value,
    beta: Value,
    _search_options: SearchOptions,
    _opts: &EngineOpts,
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
