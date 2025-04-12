//! Benchmark the move generation
#![allow(missing_docs)]
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::move_generation::ordered_moves;
use sandy_engine::move_generation::unordered_moves;
use sandy_engine::position::Position;
use sandy_engine::util::bench_positions;

/// Benchmark the move generation
fn move_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_gen_bench");

    group.noise_threshold(0.08);

    group.bench_function("unordered", |b| {
        b.iter(|| {
            for _ in 0..20 {
                // 20 iterations on 50 positions = 1_000 executions.
                for startpos in bench_positions() {
                    let _ = unordered_moves(black_box(&startpos));
                }
            }
        })
    });

    group.bench_function("ordered", |b| {
        b.iter(|| {
            for _ in 0..20 {
                // 20 iterations on 50 positions = 1_000 executions.
                for startpos in bench_positions() {
                    let _ = ordered_moves(black_box(&Position::from(startpos)));
                }
            }
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = move_generation
}
criterion_main!(benches);
