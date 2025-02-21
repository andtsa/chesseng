//! a timed depth test
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Instant;

use colored::Colorize;

/// how long to let the test run for before killing it, in ms
pub const TEST_DURATION: u64 = 90_000;
/// slack around the test duration
pub const STRICTNESS_THRESHOLD: u64 = 1_000;

/// a depth test of 6
#[test]
fn main() {
    depth_test(6);
}

/// Test the engine at a certain depth
pub fn depth_test(depth: usize) {
    let exec = PathBuf::from(env!("CARGO_BIN_EXE_chesseng"));

    let mut cmd = Command::new(exec);

    let go_command = format!("go depth {depth}");

    let sequence = [
        "uci",
        "setoption name use_tt value on",
        "setoption name bench_log value on",
        "setoption name threads value 8",
        "isready",
        "position startpos",
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

    let _killer = thread::spawn(move || {
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

    let mut nodes_searched = 0;
    let start = Instant::now();

    let print_results = |nodes| {
        println!(
            "{}",
            format!(" duration: {}ms ", start.elapsed().as_millis())
                .black()
                .bold()
                .on_bright_green()
        );
        println!(
            "{}",
            format!(" total nodes searched: {} ", nodes)
                .black()
                .bold()
                .on_bright_green()
        );
    };

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() > 1 && parts[0] == "bestmove" {
            println!("Best move: {}", parts[1]);
            print_results(nodes_searched);
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
            if let Some(idx) = parts.iter().position(|&x| x == "nodes") {
                let nodes = parts[idx + 1].parse::<u64>().unwrap();
                nodes_searched += nodes;
            }
        } else if parts.iter().any(|x| x.eq_ignore_ascii_case("quitting")) {
            print_results(nodes_searched);
            break;
        }
    }

    println!("killing child");
    child.kill().unwrap();
}
