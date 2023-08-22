use std::str::FromStr;
use chess::Board;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::bot::Bot;
use crate::move_generation::{all_moves, compute_best_move, ordered_moves};

#[path = "../src/main.rs"]
mod main;
#[path = "../src/move_generation.rs"]
mod move_generation;
#[path = "../src/util.rs"]
mod util;
#[path = "../src/bot.rs"]
mod bot;
#[path = "../src/evaluation.rs"]
mod evaluation;


fn full_move_generation_benchmark(c : &mut Criterion) {
    let board = black_box(
        Board::from_str("r1b2r1k/4qp1p/p1Nppb1Q/4nP2/1p2P3/2N5/PPP4P/2KR1BR1 b - - 5 18").unwrap()
    );
    let bot = black_box(
        Bot::new()
    );
    c.bench_function(
        "final move generation benchmark",
        |b| b.iter(|| compute_best_move(&board, 4, &bot))
    );
}

criterion_group!(benches, full_move_generation_benchmark);
criterion_main!(benches);