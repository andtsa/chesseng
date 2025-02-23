//! a timed depth test
use crate::shared::test_uci;

/// get testing functions
mod shared;

/// a depth test of 6
#[test]
fn main() {
    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "setoption name bench_log value on",
        "isready",
        "position startpos",
        "go depth 6",
    ];

    test_uci(&sequence);
}
