//! Benchmarks for the iterative deepening search.
//! Here we are allowed to reuse transposition tables across different depths,
//! since that's the most realistic real-world scenario.
#![allow(missing_docs)]

use std::time::Duration;
use std::time::Instant;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use sandy_engine::Engine;
use sandy_engine::opts::Opts;
use sandy_engine::opts::setopts;
use sandy_engine::setup::depth::Depth;
use sandy_engine::util::short_benches;

/// Search benchmarks with different depths
fn search_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_bench");

    group.noise_threshold(0.08);

    for (p_idx, startpos) in short_benches().iter().enumerate() {
        let mut engine = Engine::new().unwrap();
        engine.set_search_to(Depth(5));
        engine.board = (*startpos).into();
        setopts(Opts::bench().tt(true)).unwrap();

        group.bench_function(format!("id_pos_{p_idx}"), |b| {
            engine
                .set_search_until(Instant::now() + Duration::from_millis(10000))
                .unwrap();
            b.iter(|| {
                let _ = engine.begin_search();
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
