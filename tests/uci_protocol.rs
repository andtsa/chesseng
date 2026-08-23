//! the engine has to answer every `go` with exactly one bestmove, survive
//! whatever a GUI sends it, and not throw away material at the root.
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

/// how long to let the engine run before giving up on it, in ms
const TEST_DURATION: u64 = 15_000;

/// drive the engine with `commands` and collect the lines it prints
fn run_engine(commands: &[&str]) -> Vec<String> {
    let exec = PathBuf::from(env!("CARGO_BIN_EXE_chesseng"));

    let mut cmd = Command::new(exec);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    #[allow(clippy::zombie_processes)]
    let mut child = cmd.spawn().unwrap();

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    let mut reader = std::io::BufReader::new(stdout);
    let mut writer = std::io::BufWriter::new(stdin);

    for line in commands {
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }

    // hold stdin open so the engine keeps searching, then close it to finish
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(TEST_DURATION));
        let _ = writer.write_all(b"quit\n");
        let _ = writer.flush();
    });

    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break; // engine closed its output
        }
        let line = line.trim_end().to_string();
        let done = line.starts_with("bestmove");
        println!("{line}");
        lines.push(line);
        if done {
            break;
        }
    }

    child.kill().unwrap();
    lines
}

/// preamble that puts the engine in a known state
fn go(position: &str, movetime: u64) -> Vec<String> {
    run_engine(&[
        "uci",
        "debug off",
        "isready",
        "ucinewgame",
        position,
        &format!("go movetime {movetime}"),
    ])
}

/// however little time it is given, a `go` owes the GUI exactly one bestmove.
/// below roughly 20ms the engine used to print a bare empty line instead, which
/// leaves a GUI waiting forever.
#[test]
fn every_go_answers_with_one_bestmove() {
    for movetime in [1, 3, 5, 10, 50] {
        let lines = go("position startpos moves e2e4 e7e5", movetime);
        let bestmoves: Vec<_> = lines.iter().filter(|l| l.starts_with("bestmove")).collect();

        assert_eq!(
            bestmoves.len(),
            1,
            "movetime {movetime}ms produced {} bestmove lines, not 1",
            bestmoves.len()
        );
        assert!(
            bestmoves[0]
                .split_whitespace()
                .nth(1)
                .is_some_and(|m| m.len() >= 4),
            "movetime {movetime}ms gave a malformed bestmove: {:?}",
            bestmoves[0]
        );
    }
}

/// the root used to pass its own alpha-beta window to the children without
/// negating it, which inverted every cutoff below the root. in this position
/// that made the engine play e2e3 and leave the pawn on f4 alone.
#[test]
fn root_takes_the_free_pawn() {
    let lines = go(
        "position fen 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 11",
        2000,
    );
    let bestmove = lines
        .iter()
        .find(|l| l.starts_with("bestmove"))
        .expect("no bestmove");

    assert_eq!(
        bestmove.split_whitespace().nth(1),
        Some("b4f4"),
        "expected the rook to take on f4, got {bestmove:?}"
    );
}

/// a GUI can send a value the engine cannot parse. that used to panic, and the
/// panic hook takes the whole process down with it.
#[test]
fn a_bad_option_value_does_not_kill_the_engine() {
    let lines = run_engine(&[
        "uci",
        "debug off",
        "isready",
        "setoption name hash value abc",
        "position startpos",
        "go movetime 100",
    ]);

    assert!(
        lines.iter().any(|l| l.starts_with("bestmove")),
        "engine did not survive a non-numeric option value"
    );
}

/// `hash` is advertised as accepting 0, which used to build a zero-length table
/// and then divide by zero on the first probe. it now means "no table".
#[test]
fn hash_of_zero_disables_the_table() {
    let lines = run_engine(&[
        "uci",
        "debug off",
        "isready",
        "setoption name hash value 0",
        "position startpos",
        "go movetime 300",
    ]);

    assert!(
        lines.iter().any(|l| l.starts_with("bestmove")),
        "engine did not survive a hash size of 0"
    );
}
