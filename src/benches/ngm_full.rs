//! Benchmarks for the negamax search with different depths
#![allow(missing_docs)]
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::opts::Opts;
use sandy_engine::position::Position;
use sandy_engine::search::negamax::ng_bench;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::transposition_table::TranspositionTable;
use sandy_engine::transposition_table::TT;
use sandy_engine::util::bench_positions;

/// Search benchmarks with different depths
fn negamax_benches(c: &mut Criterion) {
    let depths = [3, 4, 5, 6];
    let mut group = c.benchmark_group("negamax_bench");

    group.noise_threshold(0.08);

    for d_idx in depths {
        let table = TT::new();
        let positions = bench_positions()
            .into_iter()
            .chain(bench_positions().into_iter())
            .map(Position::from)
            .collect::<Vec<Position>>();

        group.bench_function(format!("ngm_full_depth_{}", d_idx), |b| {
            b.iter(|| {
                for startpos in positions.iter() {
                    // run 100 positions
                    let _ = ng_bench(
                        black_box(startpos.clone()),
                        black_box(Depth(d_idx)),
                        black_box(Value::MIN),
                        black_box(Value::MAX),
                        Opts::bench(),
                        &table,
                    );
                    // for correctness, don't reuse the entries from a previous run!
                    // however, we still need to use the same table allocation, as reallocating
                    // takes a significant amount of time, that isn't representative of the speed we
                    // want to bench, which is that of a single search.
                    table.get().write().unwrap().clear();
                }
            })
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = negamax_benches
}
criterion_main!(benches);
