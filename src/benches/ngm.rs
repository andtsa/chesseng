use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::search::negamax::ngm;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::util::bench_positions;

fn search_benches(c: &mut Criterion) {
    let positions = bench_positions();
    // let depths = vec![Depth(3), Depth(5), Depth(6)];

    let mut group = c.benchmark_group("search_bench");

    for (p_idx, startpos) in positions.into_iter().enumerate() {
        group.bench_function(format!("ngm_pos_{}_depth_3", p_idx), |b| {
            b.iter(|| {
                ngm(
                    black_box(startpos),
                    black_box(Depth(3)),
                    black_box(Value::MIN),
                    black_box(Value::MAX),
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
