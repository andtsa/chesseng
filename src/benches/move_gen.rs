//! Benchmark the move generation
#![allow(missing_docs)]
use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use sandy_engine::move_generation::moveordering::ordered_moves;
use sandy_engine::move_generation::moveordering::unordered_moves;
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
                    let _ = ordered_moves(black_box(&startpos));
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
