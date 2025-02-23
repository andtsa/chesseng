//! How deep can we reach in 1.5 minute
use crate::shared::test_uci;

/// get testing functions
mod shared;

/// run the engine for 90 seconds
#[test]
fn main() {
    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "setoption name bench_log value on",
        "setoption name threads value 8",
        "isready",
        "position startpos",
        "go movetime 90000",
    ];

    test_uci(&sequence);
}
