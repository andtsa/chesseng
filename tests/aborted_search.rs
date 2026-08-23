//! a search that is cut off must say so, rather than reporting a
//! static evaluation as though it were a real result.
use std::str::FromStr;
use std::sync::atomic::Ordering;

use chess::Board;
use sandy_engine::opts::Opts;
use sandy_engine::position::Position;
use sandy_engine::search::SEARCHING;
use sandy_engine::search::SearchOptions;
use sandy_engine::search::negamax::negamax;
use sandy_engine::setup::depth::Depth;
use sandy_engine::setup::values::Value;
use sandy_engine::transposition_table::TT;

#[test]
fn aborted_searches_are_flagged() {
    let board =
        Board::from_str("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 1")
            .unwrap();

    // the search bails out of every node
    SEARCHING.store(false, Ordering::SeqCst);

    let tt = TT::new();
    let result = negamax(
        Position::from(board),
        Depth(4),
        Value::MIN,
        Value::MAX,
        SearchOptions::default(),
        &Opts::new().engine_opts,
        &tt.get(),
    );

    assert!(
        result.aborted,
        "a search cut off before finishing reported itself as a real result"
    );
}
