use std::str::FromStr;
use std::time::Duration;

use chess::Board;
use chess::BoardStatus;
use chess::Color;

use crate::search::moveordering::ordered_moves;
use crate::search::negamax::negamax;
use crate::search::negamax::ngm;
use crate::search::negamax::Opts;
use crate::search::SEARCHING;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::util::short_benches;
use crate::util::Print;
use crate::DebugLevel::Trace;
use crate::Engine;

#[test]
fn startpos_is_positive() {
    let pos = Board::default();
    unsafe {
        SEARCHING = true;
    }
    assert!(ngm(pos, Depth(4), Value::MIN, Value::MAX).next_position_value > Value::ZERO);
}

#[test]
fn mate_is_mate() {
    let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
    for x in 1..10 {
        unsafe {
            SEARCHING = true;
        }
        // println!("x: {}", x);
        assert_eq!(pos.side_to_move(), Color::Black);
        assert_eq!(
            ngm(pos, Depth(x), Value::MIN, Value::MAX).next_position_value,
            -Value::MATE
        );
    }
}

// #[test]
// fn mate_in_1_is_mate() {
//     let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
//     for x in 1..5 {
//         unsafe {
//             SEARCHING = true;
//         }
//         assert_eq!(
//             negamax(pos, Depth(x), Value::MIN, Value::MAX,
// DbOpt::dt(false)).next_position_value,             Value::MATE,
//             "depth = {x} pos={}",
//             pos.print()
//         );
//     }
// }

#[test]
fn will_mate_in_1_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for d in 1..4 {
        let mut engine = Engine::new().unwrap();
        engine.board = pos;
        engine.opts.search = Trace;
        engine.opts.ab = false;
        engine.opts.use_pv = false;

        eprintln!("all possible moves: {}", ordered_moves(&engine.board));

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(2000))
            .expect(&format!("died at depth {d}"));
        engine.board = engine.board.make_move_new(mv);

        assert_eq!(
            engine.board.status(),
            BoardStatus::Checkmate,
            "depth={d} mv={mv} pos={}",
            pos.print()
        );
    }
}

#[test]
fn mate_in_1_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..5 {
        unsafe {
            SEARCHING = true;
        }
        assert_eq!(
            ngm(pos, Depth(x), Value::MIN, Value::MAX).next_position_value,
            Value::MATE,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn mate_in_2_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/4K2k w - - 0 1").unwrap();
    unsafe {
        SEARCHING = true;
    }
    assert_ne!(
        ngm(pos, Depth(1), Value::MIN, Value::MAX).next_position_value,
        Value::MATE,
        "depth = 1 pos={}",
        pos.print()
    );
    assert_ne!(
        ngm(pos, Depth(2), Value::MIN, Value::MAX).next_position_value,
        Value::MATE,
        "depth = 2 pos={}",
        pos.print()
    );
    for x in 3..5 {
        unsafe {
            SEARCHING = true;
        }
        assert_eq!(
            ngm(pos, Depth(x), Value::MIN, Value::MAX).next_position_value,
            Value::MATE,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn will_mate_in_2_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k b - - 0 1").unwrap();
    for d in 5..6 {
        let mut engine = Engine::new().unwrap();
        engine.board = pos;

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        engine.board = engine.board.make_move_new(mv);

        eprintln!("made first move in mating sequence: {}", mv);

        assert_eq!(
            engine.board.status(),
            BoardStatus::Ongoing,
            "depth=1 mv={mv} pos={}",
            pos.print()
        );

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        engine.board = engine.board.make_move_new(mv);

        assert_eq!(
            engine.board.status(),
            BoardStatus::Checkmate,
            "depth=1 mv={mv} pos={}",
            pos.print()
        );
    }
}

#[test]
fn score_same_with_or_without_ab_pv() {
    for (_p_idx, pos) in short_benches().into_iter().enumerate() {
        for x in 1..4 {
            unsafe {
                SEARCHING = true;
            }
            // println!("testing pos_{p_idx}_depth_{x}");
            assert_eq!(
                negamax(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(false).pv(false)
                )
                .next_position_value,
                negamax(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(true).pv(true)
                )
                .next_position_value,
                "depth = {x} pos={}",
                pos.print()
            );
        }
    }
}

#[test]
fn checkmate_the_author() {
    let pos = Board::from_str("1n1k4/r1pp1p2/7p/8/1p1q4/6r1/4q3/1K6 b - - 0 1").unwrap();
    let mut engine = Engine::new().unwrap();
    engine.board = pos;

    let mv = engine
        .best_move(Depth(2), Duration::from_millis(10000))
        .unwrap();
    engine.board = engine.board.make_move_new(mv);

    assert_eq!(
        engine.board.status(),
        BoardStatus::Checkmate,
        "depth=2 move={mv} pos={}",
        pos.print()
    );
}
