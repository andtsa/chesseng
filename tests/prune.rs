//! a single-threaded depth test, to check pruning
use crate::shared::test_uci;

/// get testing functions
mod shared;

/// a depth test
#[test]
fn main() {
    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "setoption name bench_log value on",
        "setoption name threads value 1",
        "setoption name hash value 128",
        "isready",
        "position startpos",
        "go depth 7",
    ];

    test_uci(&sequence);
}
