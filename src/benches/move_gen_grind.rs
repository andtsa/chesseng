//! This file contains valgrind benchmarks for the evaluation function.
use chess::Board;
use chess::MoveGen;
use iai::black_box;
use sandy_engine::move_generation::ordered_moves;
use sandy_engine::move_generation::unordered_moves;
use sandy_engine::position::Position;

/// how many instructions does the library need to generate moves
fn lib_move_gen() {
    let _ = MoveGen::new_legal(&Board::default());
}

/// how many instructions does it take to generate unordered moves
fn unordered_move_gen() {
    let _ = unordered_moves(&Board::default());
}

/// how many instructions does it take to generate ordered moves
fn ordered_move_gen() {
    let _ = ordered_moves(&Position::default());
}

/// how many instructions for the blackboxed version?
fn blackbox_ordered_move_gen() {
    let _ = ordered_moves(black_box(&Position::default()));
}

iai::main!(
    lib_move_gen,
    unordered_move_gen,
    ordered_move_gen,
    blackbox_ordered_move_gen
);
