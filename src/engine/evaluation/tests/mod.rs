use std::str::FromStr;

use chess::Board;

use crate::evaluation;
use crate::opts::Opts;
use crate::search::moveordering::ordered_moves;
use crate::setup::values::Value;

// #[test]
// fn startpos_is_tempo() {
//     let pos = Board::default();
//     let moves = ordered_moves(&pos);
//     assert_eq!(
//         evaluation::evaluate(&pos, &moves, DbOpt { debug: true }),
//         TEMPO
//     );
// }

// #[test]
// fn mate_is_mate() {
//     let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
//     let moves = ordered_moves(&pos);
//     assert_eq!(
//         evaluation::evaluate(&pos, &moves, DbOpt { debug: true }),
//         -Value::MATE,
//         "{}",
//         pos.print()
//     );
// }

#[test]
fn white_completely_winning() {
    let pos = Board::from_str("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1").unwrap();
    let moves = ordered_moves(&pos);
    assert!(evaluation::eval(&pos, &moves, Opts::new()).unwrap() > Value::ZERO);
}

#[test]
fn black_completely_losing() {
    let pos = Board::from_str("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR b KQ - 0 1").unwrap();
    let moves = ordered_moves(&pos);
    assert!(evaluation::eval(&pos, &moves, Opts::new()).unwrap() < Value::ZERO);
}

#[test]
fn white_completely_losing() {
    let pos = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPP3/RNBQK3 w Qkq - 0 1").unwrap();
    let moves = ordered_moves(&pos);
    assert!(evaluation::eval(&pos, &moves, Opts::new()).unwrap() < Value::ZERO);
}

#[test]
fn black_completely_winning() {
    let pos = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPP3/RNBQK3 b Qkq - 0 1").unwrap();
    let moves = ordered_moves(&pos);
    assert!(evaluation::eval(&pos, &moves, Opts::new()).unwrap() > Value::ZERO);
}
