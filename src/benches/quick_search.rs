//! Quick search benchmarks for the sandy engine
#![allow(missing_docs)]
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::opts::Opts;
use sandy_engine::search::negamax::ng_test;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::util::bench_positions;

/// Search benchmarks with different starting positions
fn search_benches(c: &mut Criterion) {
    let positions = bench_positions();
    let samples = [0, 5, 29];

    let mut group = c.benchmark_group("search_bench");

    for p_idx in samples {
        let startpos = positions[p_idx];
        group.bench_function(format!("ngm_pos_{}_depth_6", p_idx), |b| {
            b.iter(|| {
                ng_test(
                    black_box(startpos),
                    black_box(Depth(6)),
                    black_box(Value::MIN),
                    black_box(Value::MAX),
                    Opts::bench(),
                )
            })
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = search_benches
}
criterion_main!(benches);
