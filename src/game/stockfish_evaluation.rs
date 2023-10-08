#[allow(unused_mut)]
use std::process::{ChildStdout, Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chess::{Board, ChessMove};
use regex::Regex;
use crate::Engine;

pub fn eval(board : Board) {
    println!("evaluating {}", board.to_string());
}

pub struct Stockfish {
    process: std::process::Child,
    stdin: std::process::ChildStdin,
    pub reader: Arc<Mutex<BufReader<ChildStdout>>>,
    movetime : i32,
}

impl Stockfish {
    pub fn new() -> Self {
        let mut engine = Command::new("/opt/homebrew/Cellar/stockfish/16/bin/stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start engine");

        let mut stdin = engine.stdin.take().unwrap();
        let mut stdout = engine.stdout.take().unwrap();
        let reader = Arc::new(Mutex::new(BufReader::new(stdout)));
        writeln!(stdin, "uci").unwrap();

        Stockfish {
            process: engine,
            stdin,
            reader,
            movetime : 500,
        }
    }
    pub fn quit(mut self) -> std::io::Result<()> {
        writeln!(&self.stdin, "quit").unwrap();
        thread::sleep(Duration::from_millis(100));
        self.process.kill()
    }
}

impl Engine for Stockfish {
    fn evaluate(&self, board : &Board) -> f64 {
        // post board to engine process, get evaluation
        writeln!(&self.stdin, "ucinewgame").unwrap();
        writeln!(&self.stdin, "position fen {}", board.to_string()).unwrap();
        writeln!(&self.stdin, "go movetime {}", self.movetime).unwrap();

        let mut out : f64 = 999900.0;

        let reader_clone = Arc::clone(&self.reader);
        let mut reader = reader_clone.lock().unwrap();
        let mut line = String::new();
        let mut prevline = String::new();
        thread::sleep(Duration::from_millis(2*self.movetime as u64));
        while reader.read_line(&mut line).unwrap() > 0 {
            if line.contains("bestmove ") {
                let re = Regex::new(r"score cp (\d+)").unwrap();
                match re.captures(prevline.as_str()) {
                    Some(caps) => {
                        out = i32::from_str(&caps[1]).unwrap_or(0) as f64;
                    },
                    None => break,
                }
                break;
            } else {
                prevline = line.clone();
            }
        }
        -2.0*(board.side_to_move().to_index()as f64-0.5)*out/100.0
    }

    fn next_move(&mut self, board: &Board) -> ChessMove {
        // post board to engine process, get evaluation
        writeln!(&self.stdin, "ucinewgame").unwrap();
        writeln!(&self.stdin, "position fen {}", board.to_string()).unwrap();
        writeln!(&self.stdin, "go movetime {}", self.movetime).unwrap();

        let mut out : String = String::new();

        let reader_clone = Arc::clone(&self.reader);
        let mut reader = reader_clone.lock().unwrap();
        let mut line = String::new();
        thread::sleep(Duration::from_millis(3*self.movetime as u64));
        while reader.read_line(&mut line).unwrap() > 0 {
            if line.contains("bestmove ") {
                let re = Regex::new(r"bestmove ([a-z0-9]{4})").unwrap();
                match re.captures(line.as_str()) {
                    Some(caps) => {
                        out = caps[1].parse().unwrap();
                    },
                    None => break,
                }
                break;
            }
        }
        ChessMove::from_str(&out).unwrap()
    }
}