use std::str::FromStr;

use chess::Board;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sandy_engine::search::negamax::ngm;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::util::bench_positions;

#[allow(dead_code)]
fn short_benches() -> Vec<Board> {
    let positions = vec![
        // starting position
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // 5-man positions
        "8/8/8/8/5kp1/P7/8/1K1N4 w - - 0 1", // Kc2 - mate
        // 6-man positions
        "8/8/1P6/5pr1/8/4R3/7k/2K5 w - - 0 1", // Re5 - mate
        // 7 man position
        "8/R7/2q5/8/6k1/8/1P5p/K6R w - - 0 124",
    ];
    positions
        .iter()
        .map(|p| Board::from_str(p).unwrap())
        .collect()
}

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
