use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::opts::Opts;
use sandy_engine::search::moveordering::ordered_moves;
use sandy_engine::util::bench_positions;

// fn evaluation_benches(c: &mut Criterion) {
//     let mut group = c.benchmark_group("eval_bench");
//
//     for (p_idx, pos) in bench_positions().iter().enumerate() {
//         group.bench_function(format!("eval_pos_{p_idx}"), |b| {
//             b.iter(|| {
//                 let _ = sandy_engine::evaluation::eval(
//                     black_box(pos),
//                     &ordered_moves(pos),
//                     Opts::bench(),
//                 );
//             })
//         });
//     }
// }

fn evaluation_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_bench");

    group.bench_function("eval_full", |b| {
        b.iter(|| {
            for pos in bench_positions() {
                let _ = sandy_engine::evaluation::eval(
                    black_box(&pos),
                    &ordered_moves(&pos),
                    Opts::bench(),
                );
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
