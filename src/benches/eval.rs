//! Benchmarking the evaluation function
#![allow(missing_docs)]
use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use sandy_engine::opts::Opts;
use sandy_engine::search::moveordering::ordered_moves;
use sandy_engine::util::bench_positions;

/// Benchmark the evaluation function
fn evaluation_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_bench");

    group.noise_threshold(0.05);

    group.bench_function("eval_full", |b| {
        b.iter(|| {
            for pos in bench_positions() {
                for _ in 0..20 {
                    // 20 iterations on 50 positions = 1_000 executions.
                    let _ = sandy_engine::evaluation::eval(
                        black_box(&pos),
                        &ordered_moves(&pos),
                        Opts::bench(),
                    );
                }
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = evaluation_benches
}
criterion_main!(benches);
