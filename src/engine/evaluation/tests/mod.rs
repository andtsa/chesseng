use std::str::FromStr;

use chess::Board;

use crate::evaluation;
use crate::search::moveordering::ordered_moves;
use crate::setup::values::Value;
use crate::util::Print;

#[test]
fn startpos_is_0() {
    let pos = Board::default();
    let moves = ordered_moves(&pos);
    assert_eq!(evaluation::evaluate(&pos, &moves), Value::ZERO);
}

#[test]
fn mate_is_mate() {
    let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
    let moves = ordered_moves(&pos);
    assert_eq!(
        evaluation::evaluate(&pos, &moves),
        -Value::MATE,
        "{}",
        pos.print()
    );
}
