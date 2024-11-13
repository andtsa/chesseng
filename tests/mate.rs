use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Instant;

use colored::Colorize;

pub const TEST_DURATION: u64 = 10_000;
pub const STRICTNESS_THRESHOLD: u64 = 1_000;

#[test]
fn main() {
    let startpos = "1r2k3/8/K3p3/4p3/4q3/8/5bpr/6q1 b - - 0 44";
    let valid_best_moves = ["b8a8", "e4a4", "e4a8", "g1a1"];
    test_mating(startpos, &valid_best_moves);

    let mate_in_2 = "8/1k6/8/8/7n/4Nn2/8/1rq2R1K b - - 0 1";
    let valid_best_moves = ["c1f1"];
    test_mating(mate_in_2, &valid_best_moves);
}

fn test_mating(startpos: &str, valid_mates: &[&str]) {
    let exec = PathBuf::from(env!("CARGO_BIN_EXE_chesseng"));

    let mut cmd = Command::new(exec);

    let start_command = format!("position fen {}", startpos);

    let sequence = [
        "uci",
        // "setoption name use_tt value off",
        "isready",
        &start_command,
        "go movetime 5000",
    ];

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

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
        thread::sleep(std::time::Duration::from_millis(
            TEST_DURATION + STRICTNESS_THRESHOLD,
        ));
        if let Err(e) = writer.write_all(b"quit\n") {
            eprintln!("killer encountered error: {e}");
        }
        if let Err(e) = writer.flush() {
            eprintln!("killer encountered error: {e}");
        }
    });

    let start = Instant::now();
    let mut best_move = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() > 1 && parts[0] == "bestmove" {
            best_move = parts[1].to_string();
            println!("Best move: {}", best_move);
            break;
        } else if parts.len() > 1 && parts[0] == "info" {
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

    // we assert that the engine takes the mate-in-one
    assert!(
        valid_mates.iter().any(|x| *x == best_move),
        "did not find any of the mates [{:?}], instead picked {}",
        valid_mates,
        best_move
    );

    child.kill().unwrap();
}
