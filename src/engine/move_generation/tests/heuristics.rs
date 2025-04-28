//! tests for the heuristics

use chess::ChessMove;
use chess::MoveGen;

use super::mvv_lva_score;
use crate::util::bench_positions;

#[test]
fn assert_total_order() {
    let boards = bench_positions();

    for b in boards {
        let mut mgen = MoveGen::new_legal(&b);
        mgen.set_iterator_mask(*b.color_combined(!b.side_to_move()));
        let moves: Vec<ChessMove> = mgen.collect();

        let mvv_lva = |mv| mvv_lva_score(&b, mv);

        for i in 0..moves.len() {
            for j in 0..moves.len() {
                let a = mvv_lva(&moves[i]);
                let b = mvv_lva(&moves[j]);

                // Total order requires:
                // 1. a <= b || a >= b (comparability)
                // 2. if a == b then b == a (symmetry)
                // 3. if a <= b && b <= c then a <= c (transitivity) - checked separately

                assert!(a <= b || a >= b, "Incomparable: {a:?} vs {b:?}");
                if a == b {
                    assert_eq!(b, a, "Equality not symmetric: {a:?} vs {b:?}");
                }
            }
        }

        // Check transitivity separately
        for i in 0..moves.len() {
            for j in 0..moves.len() {
                for k in 0..moves.len() {
                    let a = mvv_lva(&moves[i]);
                    let b = mvv_lva(&moves[j]);
                    let c = mvv_lva(&moves[k]);

                    if a <= b && b <= c {
                        assert!(
                            a <= c,
                            "Transitivity failed: {a:?} <= {b:?} <= {c:?} but {a:?} !<= {c:?}",
                        );
                    }
                }
            }
        }
    }
}
