use chess::Board;
use sandy_engine::opts::Opts;
use sandy_engine::position::Position;
use sandy_engine::search::negamax::ng_bench;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::transposition_table::TT;
use sandy_engine::util::bench_positions;
use std::str::FromStr;

#[test]
fn one_shot_depth6() {
    println!("bench_positions len = {}", bench_positions().len());
    for (name, fen) in [
        (
            "startpos",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ),
        (
            "kiwipete",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10",
        ),
    ] {
        let board = Board::from_str(fen).unwrap();
        // one cold search straight to depth 6, exactly what the bench does
        let tt = TT::new();
        let start = std::time::Instant::now();
        let r = ng_bench(
            Position::from(board),
            Depth(6),
            Value::MIN,
            Value::MAX,
            Opts::bench(),
            &tt,
        )
        .unwrap();
        println!(
            "{name}: cold depth 6 -> nodes={} elapsed={:?}",
            r.nodes_searched,
            start.elapsed()
        );

        // the same, but iteratively deepened into a shared table, like the real search
        let tt = TT::new();
        let start = std::time::Instant::now();
        let mut nodes = 0;
        for d in 1..=6 {
            let r = ng_bench(
                Position::from(board),
                Depth(d),
                Value::MIN,
                Value::MAX,
                Opts::bench(),
                &tt,
            )
            .unwrap();
            nodes += r.nodes_searched;
        }
        println!(
            "{name}: iterative 1..6 -> nodes={nodes} elapsed={:?}",
            start.elapsed()
        );
    }
}
