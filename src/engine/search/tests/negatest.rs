use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::time::Duration;

use chess::Board;
use chess::BoardStatus;
use chess::Color;

use crate::Engine;
use crate::debug::DebugLevel::debug;
use crate::opts::opts;
use crate::opts::setopts;
use crate::position::Position;
use crate::search::SEARCHING;
use crate::search::moveordering::ordered_moves;
use crate::search::negamax::Opts;
use crate::search::negamax::ng_test;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::util::Print;
use crate::util::short_benches;

#[test]
fn startpos_is_positive() {
    let pos = Board::default();
    SEARCHING.store(true, Ordering::Relaxed);
    assert!(
        ng_test(pos, Depth(4), Value::MIN, Value::MAX, Opts::new())
            .unwrap()
            .next_position_value
            > Value::ZERO
    );
}

#[test]
fn mate_is_mate() {
    let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
    for x in 1..10 {
        SEARCHING.store(true, Ordering::Relaxed);
        // println!("x: {}", x);
        assert_eq!(pos.side_to_move(), Color::Black);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            -Value::MATE
        );
    }
}

#[test]
fn mate_in_1_is_mate() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..5 {
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 1,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn will_mate_in_1_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for d in 1..4 {
        let mut engine = Engine::new().unwrap();
        engine.board = pos.into();
        let mut opts = opts().unwrap();
        opts.search = debug;
        opts.use_ab = false;
        opts.use_pv = false;
        opts.threads = 1;
        {
            setopts(opts).unwrap();
        }

        eprintln!(
            "all possible moves: {}",
            ordered_moves(&engine.board.chessboard)
        );

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(2000))
            .unwrap_or_else(|e| panic!("died at depth {d}: {e}"));
        engine.board = engine.board.make_move(mv);

        assert_eq!(
            engine.board.chessboard.status(),
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
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 1,
            "depth = {x} pos={}",
            Position::from(pos).print()
        );
    }
}

#[test]
fn mate_in_2_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/4K2k w - - 0 1").unwrap();
    SEARCHING.store(true, Ordering::Relaxed);
    assert_ne!(
        ng_test(pos, Depth(1), Value::MIN, Value::MAX, Opts::new())
            .unwrap()
            .next_position_value,
        Value::MATE,
        "depth = 1 pos={}",
        pos.print()
    );
    assert_ne!(
        ng_test(pos, Depth(2), Value::MIN, Value::MAX, Opts::new())
            .unwrap()
            .next_position_value,
        Value::MATE,
        "depth = 2 pos={}",
        pos.print()
    );
    for x in 3..5 {
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE - 3,
            "depth = {x} pos={}",
            pos.print(),
        );
    }
}

#[test]
fn will_mate_in_2_() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k b - - 0 1").unwrap();
    for d in 5..6 {
        let mut engine = Engine::new().unwrap();

        setopts(Opts::new().tt(true).search(debug)).unwrap();
        engine.board = pos.into();

        let mv1 = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        engine.board = engine.board.make_move(mv1);

        eprintln!("made first move in mating sequence: {}", mv1);

        assert_eq!(
            engine.board.chessboard.status(),
            BoardStatus::Ongoing,
            "depth=1 mv={mv1} pos={}",
            pos.print()
        );

        let mv2 = engine
            .best_move(Depth(d), Duration::from_millis(10000))
            .unwrap();
        let board_before = engine.board.clone();
        engine.board = engine.board.make_move(mv2);
        let board_after = engine.board.clone();

        assert_ne!(board_before, board_after);
        assert_ne!(board_before.chessboard, board_after.chessboard);

        // panic!("{}", engine.table);

        assert_eq!(
            engine.board.chessboard.status(),
            BoardStatus::Checkmate,
            "depth=1 mv2={mv2} pos={}",
            engine.board.print()
        );
    }
}

#[test]
fn score_same_with_or_without_ab_pv() {
    for pos in short_benches().into_iter() {
        for x in 1..4 {
            SEARCHING.store(true, Ordering::SeqCst);
            // println!("testing pos_{p_idx}_depth_{x}");
            assert_eq!(
                ng_test(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(false).pv(false)
                )
                .unwrap()
                .next_position_value,
                ng_test(
                    pos,
                    Depth(x),
                    Value::MIN,
                    Value::MAX,
                    Opts::new().ab(true).pv(true)
                )
                .unwrap()
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
    engine.board = pos.into();

    let mv = engine
        .best_move(Depth(2), Duration::from_millis(10000))
        .unwrap();
    engine.board = engine.board.make_move(mv);

    assert_eq!(
        engine.board.chessboard.status(),
        BoardStatus::Checkmate,
        "depth=2 move={mv} pos={}",
        pos.print()
    );
}
