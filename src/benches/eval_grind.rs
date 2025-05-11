//! This file contains valgrind benchmarks for the evaluation function.
use chess::Board;
use iai::black_box;
use sandy_engine::move_generation::ordering::unordered_moves;

/// how many instructions does it take to set up a board
fn board_setup() {
    let _ = Board::default();
}

/// how many instructions does it take to generate moves
fn move_gen() {
    let _ = unordered_moves(&Board::default());
}

/// how many instructions does the evaluation function use?
/// to get the correct value, subtract [`move_gen`] and [`board_setup`]
fn evaluation_benches() {
    let pos = Board::default();
    sandy_engine::evaluation::evaluate(&pos.into(), false);
}

/// black boxed version of the same fn
fn blackbox_evaluation_benches() {
    let pos = Board::default();
    sandy_engine::evaluation::evaluate(black_box(&pos.into()), black_box(false));
}

iai::main!(
    board_setup,
    move_gen,
    evaluation_benches,
    blackbox_evaluation_benches
);
