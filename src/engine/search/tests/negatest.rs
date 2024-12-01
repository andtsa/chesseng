use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::time::Duration;

use chess::Board;
use chess::BoardStatus;
use chess::ChessMove;
use chess::Color;

use crate::debug::DebugLevel::debug;
use crate::opts::opts;
use crate::opts::setopts;
use crate::search::moveordering::ordered_moves;
use crate::search::negamax::ng_test;
use crate::search::negamax::Opts;
use crate::search::SEARCHING;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::util::short_benches;
use crate::util::Print;
use crate::Engine;

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
            Value::MATE,
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
        engine.board = pos;
        let mut opts = opts().unwrap();
        opts.search = debug;
        opts.use_ab = false;
        opts.use_pv = false;
        {
            setopts(opts).unwrap();
        }

        eprintln!("all possible moves: {}", ordered_moves(&engine.board));

        let mv = engine
            .best_move(Depth(d), Duration::from_millis(2000))
            .unwrap_or_else(|e| panic!("died at depth {d}: {e}"));
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
        SEARCHING.store(true, Ordering::Relaxed);
        assert_eq!(
            ng_test(pos, Depth(x), Value::MIN, Value::MAX, Opts::new())
                .unwrap()
                .next_position_value,
            Value::MATE,
            "depth = {x} pos={}",
            pos.print()
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
            Value::MATE,
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
    setopts(Opts::new().pv(true)).unwrap();
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

#[test]
fn forced_mating_sequence() {
    // https://lichess.org/study/Vvcgj8pb/AH8Y4XYv
    let pos =
        Board::from_str("4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24").unwrap();
    setopts(Opts::new()).unwrap();
    let mut engine = Engine::new().unwrap();
    engine.board = pos;

    let mv = engine
        .best_move(Depth(6), Duration::from_millis(10000))
        .unwrap();
    assert_eq!(mv.to_string(), "f1f8");
    engine.board = engine.board.make_move_new(mv);

    // "human" moves
    let mv = ChessMove::from_str("e7g8").unwrap();
    engine.board = engine.board.make_move_new(mv);

    let mv = engine
        .best_move(Depth(5), Duration::from_millis(10000))
        .unwrap();
    assert_eq!(mv.to_string(), "d4c5");
    engine.board = engine.board.make_move_new(mv);

    // "human" moves
    let mv = ChessMove::from_str("e8e5").unwrap();
    engine.board = engine.board.make_move_new(mv);

    let mv = engine
        .best_move(Depth(4), Duration::from_millis(10000))
        .unwrap();
    assert_eq!(mv.to_string(), "a1e5");
    engine.board = engine.board.make_move_new(mv);

    // "human" moves
    let mv = ChessMove::from_str("d7g7").unwrap();
    engine.board = engine.board.make_move_new(mv);

    let mv = engine
        .best_move(Depth(3), Duration::from_millis(10000))
        .unwrap();
    assert_eq!(mv.to_string(), "c4g8");
    engine.board = engine.board.make_move_new(mv);

    // "human" moves
    let mv = ChessMove::from_str("h6g8").unwrap();
    engine.board = engine.board.make_move_new(mv);

    let mv = engine
        .best_move(Depth(2), Duration::from_millis(10000))
        .unwrap();
    assert_eq!(mv.to_string(), "g5f7");
    engine.board = engine.board.make_move_new(mv);

    assert_eq!(
        engine.board.status(),
        BoardStatus::Checkmate,
        "pos={}",
        pos.print()
    );
}
