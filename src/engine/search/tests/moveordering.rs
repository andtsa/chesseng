use std::str::FromStr;
use std::time::Instant;

use chess::Board;

use crate::opts::Opts;
use crate::opts::setopts;
use crate::search::moveordering::ordered_moves;
use crate::search::moveordering::pv_ordered_moves;
use crate::search::moveordering::unordered_moves;

#[test]
fn ordered_same_as_mg() {
    let boards = [
        Board::default(),
        Board::from_str("8/8/8/6Q1/8/8/8/5K1k b - - 0 1").unwrap(),
    ];

    for b in boards {
        let ordered = ordered_moves(&b);

        let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

        assert_eq!(ordered.len(), mg.len());
        for (i, m) in ordered.into_iter().enumerate() {
            assert!(mg.contains(&m), "move {} not found in mg", i);
        }
    }
}

#[test]
fn pv_ordered_same_as_mg() {
    let boards = [
        Board::default(),
        Board::from_str("8/8/8/6Q1/8/8/8/5K1k b - - 0 1").unwrap(),
        Board::from_str("8/8/8/6Q1/8/8/8/4K2k w - - 0 1").unwrap(),
        Board::from_str("r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1")
            .unwrap(),
        Board::from_str("8/8/8/3k4/8/8/8/5RQK w - - 0 1").unwrap(),
    ];
    for b in boards {
        let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

        setopts(Opts::new().pv(true)).unwrap();

        for m in &mg {
            let pv_ordered = pv_ordered_moves(&b, m);
            assert_eq!(
                pv_ordered.len(),
                mg.len(),
                "\npv: {pv_ordered}, \nmg: {}",
                mg.iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            assert_eq!(pv_ordered.0.first(), Some(m), "pv: {}", pv_ordered);
            for (i, m) in pv_ordered.0.iter().enumerate() {
                assert!(mg.contains(m), "move {} not found in mg", i);
            }
        }
    }
}

#[test]
fn profile_move_ordering() {
    let duration = 5_000;
    let b = Board::default();
    let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

    let start_b = Instant::now();
    for _ in 0..duration {
        for _m in &mg {
            let _m = ordered_moves(&b);
        }
    }
    let elapsed_b = start_b.elapsed();

    let start_a = Instant::now();
    for _ in 0..duration {
        for m in &mg {
            let _m = pv_ordered_moves(&b, m);
        }
    }
    let elapsed_a = start_a.elapsed();

    let start_c = Instant::now();
    for _ in 0..duration {
        for _m in &mg {
            let _m = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();
        }
    }
    let elapsed_c = start_c.elapsed();

    let start_d = Instant::now();
    for _ in 0..duration {
        for _m in &mg {
            let _m = unordered_moves(&b);
        }
    }
    let elapsed_d = start_d.elapsed();

    eprintln!(
        "pv: {:?}, normal: {:?}, mg: {:?}, uo: {:?}",
        elapsed_a, elapsed_b, elapsed_c, elapsed_d
    );
    assert!(
        elapsed_a < 2 * elapsed_b,
        "pv: {:?}, normal: {:?}, mg: {:?}",
        elapsed_a,
        elapsed_b,
        elapsed_c
    );
    assert!(
        elapsed_a < 3 * elapsed_c,
        "pv: {:?}, normal: {:?}, mg: {:?}",
        elapsed_a,
        elapsed_b,
        elapsed_c
    );
}
