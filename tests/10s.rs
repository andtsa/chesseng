//! How deep can we reach in 10 seconds?
use crate::shared::test_uci;

/// get testing functions
mod shared;

/// run the engine for 10 seconds
#[test]
fn main() {
    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "setoption name bench_log value on",
        "setoption name threads value 8",
        "isready",
        "position startpos",
        "go movetime 10000",
    ];

    test_uci(&sequence);
}
