//! a quiescence search implementation, used to ensure the static evaluation
//! isn't ran on active positions with lots of exchanges
//!
//! + https://en.wikipedia.org/wiki/Quiescence_search
//! + https://www.chessprogramming.org/Quiescence_Search
//! + https://www.chessprogramming.org/Horizon_Effect

use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::Piece;

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
    _search_options: SearchOptions<'_>,
    opts: &EngineOpts,
) -> SearchResult {
    let mut nodes = 1;

    let mgen = MoveGen::new_legal(&pos.chessboard);
    let mut pgen = prio_iterator(mgen, &pos.chessboard, &[]);

    // the first move is generated to tell a mate or stalemate apart from a
    // position with moves available. it is the highest priority move overall,
    // which is a capture only if one exists.
    let mut current_move = pgen.next();
    let stand_pat = evaluate(&pos, current_move.is_none());

    // 1. stand-pat test
    if opts.use_ab && stand_pat >= beta {
        return SearchResult {
            pv: Vec::new(),
            next_position_value: stand_pat,
            nodes_searched: nodes,
            tb_hits: 0,
            depth: Depth::ZERO,
            from_draw: false,
            aborted: false,
        };
    }

    alpha = alpha.max(stand_pat);

    let mut pv = None;
    let mut max_depth = Depth::ZERO;
    while let Some(mv) = current_move {
        // only captures are searched. a quiet move leaks out of the generator
        // both when the position has no captures at all and at the end of the
        // capture masks, and recursing into one would not terminate: every
        // position has a quiet move, so the chain never runs out.
        if !is_capture(&pos.chessboard, mv) {
            break;
        }

        let child = -quiescence(pos.make_move(mv), -beta, -alpha, _search_options, opts);

        max_depth = max_depth.max(child.depth);
        nodes += child.nodes_searched;

        if opts.use_ab && child.next_position_value >= beta {
            return SearchResult {
                pv: vec![MV(mv, child.next_position_value)],
                next_position_value: child.next_position_value,
                nodes_searched: nodes,
                depth: max_depth + 1,
                tb_hits: 0,
                from_draw: child.from_draw,
                aborted: false,
            };
        }

        if child.next_position_value >= alpha {
            alpha = child.next_position_value;
            pv = Some(mv);
        }

        current_move = pgen.generate_captures();
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
        from_draw: false,
        aborted: false,
    }
}

/// does `mv` capture a piece? covers en passant, where the destination square
/// is empty but the moving pawn changes file.
fn is_capture(board: &Board, mv: ChessMove) -> bool {
    board.piece_on(mv.get_dest()).is_some()
        || (board.piece_on(mv.get_source()) == Some(Piece::Pawn)
            && mv.get_source().get_file() != mv.get_dest().get_file())
}
