use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::opts::Opts;
use sandy_engine::search::negamax::ng_test;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::util::bench_positions;

fn search_benches(c: &mut Criterion) {
    let depths = [3, 4];
    let mut group = c.benchmark_group("search_bench");

    for d_idx in depths {
        group.bench_function(format!("ngm_full_depth_{}", d_idx), |b| {
            b.iter(|| {
                for startpos in bench_positions() {
                    let _ = ng_test(
                        black_box(startpos),
                        black_box(Depth(d_idx)),
                        black_box(Value::MIN),
                        black_box(Value::MAX),
                        Opts::bench(),
                    );
                }
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
