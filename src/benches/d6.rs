use chess::Board;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::opts::Opts;
use sandy_engine::search::negamax::ng_test;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;

fn search_benches(c: &mut Criterion) {
    let startpos = Board::default();

    let mut group = c.benchmark_group("search_bench");

    group.bench_function("ngm_pos_0_depth_6_tt_false", |b| {
        b.iter(|| {
            ng_test(
                black_box(startpos),
                black_box(Depth(6)),
                black_box(Value::MIN),
                black_box(Value::MAX),
                Opts::bench().tt(false),
            )
        })
    });

    group.bench_function("ngm_pos_0_depth_6_tt_true", |b| {
        b.iter(|| {
            ng_test(
                black_box(startpos),
                black_box(Depth(6)),
                black_box(Value::MIN),
                black_box(Value::MAX),
                Opts::bench().tt(true),
            )
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
