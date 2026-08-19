//! Benchmarks for the negamax search with different depths
#![allow(missing_docs)]
use criterion::BatchSize;
use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use sandy_engine::opts::Opts;
use sandy_engine::position::Position;
use sandy_engine::search::negamax::ng_bench;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::transposition_table::TT;
use sandy_engine::transposition_table::TranspositionTable;
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
            .map(Position::from)
            .collect::<Vec<Position>>();

        group.bench_function(format!("ngm_full_depth_{d_idx}"), |b| {
            b.iter_batched(
                // for correctness, don't reuse the entries from a previous run!
                // however, we still need to use the same table allocation, as reallocating
                // takes a significant amount of time, that isn't representative of the speed we
                // want to bench, which is that of a single search.
                //
                // clearing happens here rather than inside the measured closure
                // because emptying every entry is not part of what is being
                // timed. entries shared between two different positions of the
                // same run are just ordinary table traffic.
                || table.get().write().unwrap().clear(),
                |_| {
                    for startpos in positions.iter() {
                        let _ = ng_bench(
                            black_box(startpos.clone()),
                            black_box(Depth(d_idx)),
                            black_box(Value::MIN),
                            black_box(Value::MAX),
                            Opts::bench(),
                            &table,
                        );
                    }
                },
                BatchSize::PerIteration,
            )
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    // one iteration is a full sweep of every bench position at the given
    // depth, which is seconds rather than microseconds. criterion's default of
    // 100 samples would put this group in the tens of minutes, and would fail
    // to collect even two samples inside its default budget.
    //
    // the measurement time is left at its default: raising it only pads out
    // the shallow depths, while the deep ones are bounded by the sample count
    // either way (criterion warns that it overran, which is expected here).
    config = Criterion::default().sample_size(10);
    targets = negamax_benches
}
criterion_main!(benches);
