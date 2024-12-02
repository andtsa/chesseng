//! Benchmarks for depth 6 search
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

/// Search benchmarks with different engine options
fn search_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_bench");

    group.bench_function("ngm_pos_0_depth_6_tt_false", |b| {
        b.iter(|| {
            for startpos in bench_positions() {
                let _ = ng_test(
                    black_box(startpos),
                    black_box(Depth(6)),
                    black_box(Value::MIN),
                    black_box(Value::MAX),
                    Opts::bench().tt(false),
                );
            }
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = search_benches
}
criterion_main!(benches);
