use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::str::FromStr;
use chess::Board;
use regex::Regex;

pub trait Engine {
    fn evaluate(&self, board : Board) -> f64;
}
pub struct Stockfish {
    process: std::process::Child,
    stdin: std::process::ChildStdin,
    stdout: std::process::ChildStdout,
}

impl Stockfish {
    pub fn new() -> Self {
        let mut engine = Command::new("/opt/homebrew/Cellar/stockfish/16/bin/stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start engine");

        let mut stdin = engine.stdin.take().unwrap();
        let stdout = engine.stdout.take().unwrap();

        writeln!(stdin, "uci").unwrap();

        Stockfish {
            process: engine,
            stdin,
            stdout,
        }
    }
}
impl Engine for Stockfish {
    fn evaluate(&self, board : Board) -> f64 {
        // post board to engine process, get evaluation
        writeln!(self.stdin, "eval {}", board.to_string()).unwrap();

        let mut out : f64 = 0.0;
        let reader = BufReader::new(self.stdout);
        for line in reader.lines() {
            let l = line.unwrap();
            if l.starts_with("Final evaluation") {
                let re = Regex::new(r"Final evaluation\s+([\+\-]\d+\.\d+)").unwrap();
                match re.captures(l.as_str()) {
                    Some(caps) => {
                       out = f64::from_str(&caps[1]).unwrap_or(0.0);
                    },
                    None => break,
                }
                break;
            }
        }
        out
    }
}
