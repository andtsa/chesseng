//! the actual logic of move ordering

use std::ops::BitOr;
use std::ops::Not;

use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use chess::EMPTY;

use crate::evaluation::bitboards::king_attacks;
use crate::evaluation::bitboards::CENTER_4;
use crate::evaluation::bitboards::CENTER_FILES;
use crate::evaluation::bitboards::PROMOTION_COMBINED;
use crate::evaluation::material::INITIAL_VALUES;
use crate::setup::values::Value;

/// the actual logic of move ordering
pub fn move_gen_ordering(b: &Board, mut mg: MoveGen, buf: &mut Vec<ChessMove>) {
    let mut promotions = vec![];
    let mut captures = vec![];
    let mut other = vec![];

    // first get all moves that capture a piece,
    // prioritising the most valuable pieces
    let squares_with_pieces = b
        .color_combined(b.side_to_move())
        .bitor(b.color_combined(b.side_to_move().not()));
    mg.set_iterator_mask(squares_with_pieces.bitor(PROMOTION_COMBINED));
    for m in mg.by_ref() {
        if m.get_promotion().is_some() {
            promotions.push(m);
        } else if b.piece_on(m.get_dest()).is_some() {
            captures.push(m);
        } else {
            other.push(m);
        }
    }

    // a capture that promotes is probably the best move
    buf.append(&mut promotions);

    // sort captures by MVV-LVA
    captures.sort_unstable_by_key(|m| mvv_lva_score(b, m));
    buf.append(&mut captures);

    // get the area around the opponent king. we probably want to go there
    let opponent_king = b.king_square(b.side_to_move().not());

    let area_masks = [king_attacks(opponent_king), CENTER_4, CENTER_FILES];

    // then get all moves that move to a central square
    for &mask in &area_masks {
        mg.set_iterator_mask(mask);
        buf.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());
    }

    // then get all other moves
    buf.append(&mut other);

    mg.set_iterator_mask(!EMPTY);
    buf.extend(mg);
}

/// Assigns a score to a capture move based on MVV-LVA
fn mvv_lva_score(b: &Board, mv: &ChessMove) -> Value {
    match (b.piece_on(mv.get_source()), b.piece_on(mv.get_dest())) {
        (Some(capturing_piece), Some(captured_piece)) => {
            let capturing_value = INITIAL_VALUES[capturing_piece as usize];
            let captured_value = INITIAL_VALUES[captured_piece as usize];
            captured_value - capturing_value
        }
        _ if cfg!(debug_assertions) => unreachable!("mvv_lva_score called with a non-capture move"),
        _ => Value::ZERO,
    }
}
