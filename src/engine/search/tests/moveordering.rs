use std::time::Instant;

use chess::Board;

use crate::opts::setopts;
use crate::opts::Opts;
use crate::search::moveordering::ordered_moves;
use crate::search::moveordering::pv_ordered_moves;

#[test]
fn ordered_same_as_mg() {
    let b = Board::default();
    let ordered = ordered_moves(&b);

    let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

    assert_eq!(ordered.len(), mg.len());
    for (i, m) in ordered.into_iter().enumerate() {
        assert!(mg.contains(&m.0), "move {} not found in mg", i);
    }
}

#[test]
fn pv_ordered_same_as_mg() {
    let b = Board::default();
    let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

    setopts(Opts::default().pv(true)).unwrap();

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
        assert_eq!(pv_ordered.pv.unwrap().0, *m);
    }
}

#[test]
fn profile_move_ordering() {
    let b = Board::default();
    let mg = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();

    let start_b = Instant::now();
    for _ in 0..5_000 {
        for _m in &mg {
            let _m = ordered_moves(&b);
        }
    }
    let elapsed_b = start_b.elapsed();

    let start_a = Instant::now();
    for _ in 0..5_000 {
        for m in &mg {
            let _m = pv_ordered_moves(&b, m);
        }
    }
    let elapsed_a = start_a.elapsed();

    let start_c = Instant::now();
    for _ in 0..5_000 {
        for _m in &mg {
            let _m = chess::MoveGen::new_legal(&b).collect::<Vec<_>>();
        }
    }
    let elapsed_c = start_c.elapsed();

    eprintln!(
        "pv: {:?}, normal: {:?}, mg: {:?}",
        elapsed_a, elapsed_b, elapsed_c
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
