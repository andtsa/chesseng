use std::str::FromStr;

use chess::Board;
use chess::Color;

use crate::search::negamax::{DbOpt, negamax, ngm};
use crate::search::SEARCHING;
use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::util::{bench_positions, Print};

#[test]
fn startpos_is_0() {
    let pos = Board::default();
    unsafe { SEARCHING = true; }
    assert_eq!(
        ngm(pos, Depth(2), Value::MIN, Value::MAX).next_position_value,
        Value::ZERO
    );
}

#[test]
fn mate_is_mate() {
    let pos = Board::from_str("8/8/8/8/8/8/8/5KQk b - - 0 1").unwrap();
    for x in 1..10 {
        unsafe { SEARCHING = true; }
        println!("x: {}", x);
        assert_eq!(pos.side_to_move(), Color::Black);
        assert_eq!(
            ngm(pos, Depth(x), Value::MIN, Value::MAX).next_position_value,
            -Value::MATE
        );
    }
}

#[test]
fn mate_in_1_is_mate() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..6 {
        unsafe { SEARCHING = true; }
        assert_eq!(
            negamax(pos, Depth(x), Value::MIN, Value::MAX, DbOpt::dt(false)).next_position_value,
            Value::MATE,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn mate_in_1_is_mate_ngm() {
    let pos = Board::from_str("8/8/8/6Q1/8/8/8/5K1k w - - 0 1").unwrap();
    for x in 1..6 {
        unsafe { SEARCHING = true; }
        assert_eq!(
            ngm(pos, Depth(x), Value::MIN, Value::MAX).next_position_value,
            Value::MATE,
            "depth = {x} pos={}",
            pos.print()
        );
    }
}

#[test]
fn score_same_with_or_without_ab() {
    for (p_idx, pos) in bench_positions().into_iter().enumerate() {
        for x in 1..4 {
            unsafe { SEARCHING = true; }
            println!("testing pos_{p_idx}_depth_{x}");
            assert_eq!(
                negamax(pos, Depth(x), Value::MIN, Value::MAX, DbOpt::ab(true)).next_position_value,
                negamax(pos, Depth(x), Value::MIN, Value::MAX, DbOpt::ab(false)).next_position_value,
                "depth = {x} pos={}",
                pos.print()
            );
        }
    }
}