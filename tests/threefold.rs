//! test if the engine will correctly avoid draw by repetition
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Instant;

use colored::Colorize;

/// how long to let the test run for before killing it, in ms
pub const TEST_DURATION: u64 = 10_000;
/// how long the engine is given to think, in ms
pub const MOVE_TIME: u64 = 3_000;

/// the engine is a queen up and has already shuffled back to the starting
/// position once. repeating the shuffle a third time is a draw, so any move
/// that walks back into the repetition must be rejected in favour of the win.
#[test]
fn avoids_repetition() {
    // white: Kh1 + Qa1, black: Ke8. white to move, black not in check.
    let winning = "4k3/8/8/8/8/8/8/Q6K w - - 0 1";
    // a1/a2 d8/e8 shuffle, returning to the position above a second time.
    let shuffle = "a1a2 e8d8 a2a1 d8e8";
    // playing this now would repeat a position for the third time
    let repeating_move = "a1a2";

    let (best_move, last_score) = run_engine(&format!("position fen {winning} moves {shuffle}"));

    assert_ne!(
        best_move, repeating_move,
        "engine walked into the repetition instead of playing for the win",
    );

    // the draw score must not be propagating up as the score of the whole
    // search: a queen up, this should be a mate score or a large advantage.
    let winning_score = last_score.starts_with("mate ")
        || last_score
            .strip_prefix("cp ")
            .and_then(|cp| cp.parse::<i32>().ok())
            .is_some_and(|cp| cp > 200);
    assert!(
        winning_score,
        "expected a winning score, got `{last_score}` (best move {best_move})",
    );
}

/// drive the engine over UCI with `position_command`, and return the move it
/// picks along with the score of the last `info` line it sent.
fn run_engine(position_command: &str) -> (String, String) {
    let exec = PathBuf::from(env!("CARGO_BIN_EXE_chesseng"));

    let mut cmd = Command::new(exec);

    let go_command = format!("go movetime {MOVE_TIME}");

    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "debug off",
        "isready",
        "ucinewgame",
        position_command,
        &go_command,
    ];

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    #[allow(clippy::zombie_processes)]
    let mut child = cmd.spawn().unwrap();

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    let mut reader = std::io::BufReader::new(stdout);
    let mut writer = std::io::BufWriter::new(stdin);

    for seq in sequence.iter() {
        writer.write_all(seq.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(TEST_DURATION));
        if let Err(e) = writer.write_all(b"quit\n") {
            eprintln!("killer encountered error: {e}");
        }
        if let Err(e) = writer.flush() {
            eprintln!("killer encountered error: {e}");
        }
    });

    let start = Instant::now();
    let mut best_move = String::new();
    let mut last_score = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            // the engine exited without answering. reading on would spin
            // forever on an empty stream.
            panic!("engine closed its output before sending a bestmove");
        }
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() > 1 && parts[0] == "bestmove" {
            best_move = parts[1].to_string();
            println!("Best move: {best_move}");
            break;
        } else if parts.len() > 1 && parts[0] == "info" {
            if let Some(at) = parts.iter().position(|p| *p == "score") {
                last_score = parts[at + 1..at + 3].join(" ");
            }
            println!(
                "{}",
                format!(
                    "engine_info ({}ms): {}",
                    start.elapsed().as_millis(),
                    line.trim()
                )
                .black()
                .on_cyan()
            );
        } else if parts.iter().any(|x| x.eq_ignore_ascii_case("quitting")) {
            break;
        }
    }

    child.kill().unwrap();

    (best_move, last_score)
}
