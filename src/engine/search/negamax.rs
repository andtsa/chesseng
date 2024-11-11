use chess::Board;

use crate::evaluation::eval;
use crate::search::moveordering::ordered_moves;
use crate::search::SearchResult;
use crate::search::MV;
use crate::search::SEARCHING;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::Opts;
use crate::transposition_table::TranspositionTable;

fn tt() -> Box<TranspositionTable> {
    todo!()
}

#[inline(always)]
fn searching() -> bool {
    unsafe { SEARCHING }
}

#[inline(always)]
#[allow(dead_code)]
pub fn ngm(pos: Board, to_depth: Depth, alpha: Value, beta: Value) -> SearchResult {
    negamax(pos, to_depth, alpha, beta, Opts::new())
}

pub fn negamax(
    pos: Board,
    to_depth: Depth,
    mut alpha: Value,
    beta: Value,
    db: Opts,
) -> SearchResult {
    let original_alpha = alpha;
    let tt_box = tt();
    let entry = tt_box.get(&pos.get_hash());
    
    let moves = ordered_moves(&pos);
    
    db.stp(&format!("ng: {pos}, td: {to_depth:?}, a: {alpha:?}, b: {beta:?}"));
    db.stp(&format!("moves: {}", moves));
    
    if to_depth == Depth::ZERO || moves.is_empty() {
        let ev = eval(&pos, &moves);
        db.stp(&format!("return eval: {:?}", ev));
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
        let mut deeper = -negamax(pos.make_move_new(*mv), to_depth - 1, -beta, -alpha, db);
        total_nodes += deeper.nodes_searched + 1;

        if !searching() {
            db.stp("searching() == false, breaking early");
            deeper.new_eval(eval(&pos, &moves));
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

        if db.ab && alpha >= beta {
            db.stp(&format!("alpha {alpha:?} >= beta {beta:?}"));
            break;
        }
    }

    db.stp(&format!("return max_val: {:?}", best));

    SearchResult {
        pv,
        next_position_value: best.as_ref().map_or(Value::MIN, |b| b.1),
        nodes_searched: total_nodes,
    }
}

#[cfg(test)]
#[path = "tests/negatest.rs"]
mod tests;
