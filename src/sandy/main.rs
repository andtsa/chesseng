//! # Sandy Chess Engine CLI
//! This binary serves as a command line interface for the engine.
//!
//! It can be used in two modes:
//! * play in terminal mode
//! * UCI mode

#![deny(rustdoc::broken_intra_doc_links)]

use std::io::stdin;
use std::str::FromStr;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use chess::Board;
use log::info;
use log::warn;
use sandy_engine::opts::Opts;
use sandy_engine::setup::depth::Depth;
use sandy_engine::util::fen_to_str;
use sandy_engine::util::Print;
use sandy_engine::Engine;

use crate::player::terminal_loop;
use crate::uci::uci_loop;

pub mod player;
mod uci;

fn main() -> Result<()> {
    println!("Sandy Chess Engine v0.0.0");

    #[cfg(debug_assertions)]
    colog::basic_builder()
        .filter(None, log::LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertions))]
    colog::basic_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    debug_assert!(
        size_of::<Opts>() <= 24,
        "does Opts really need to be {} bytes?",
        size_of::<Opts>()
    );

    let mut engine = Engine::new()?;

    let mut read_line = String::new();
    loop {
        read_line.clear();
        stdin().read_line(&mut read_line)?;
        let command = read_line.to_ascii_lowercase();
        if command.trim().is_empty() {
            continue;
        }
        let mut parts = command.split_ascii_whitespace();
        match (parts.next().unwrap(), parts) {
            ("uci", _) => {
                info!("Entering UCI mode");
                uci_loop(engine)?;
                break;
            }
            ("play", _) => {
                info!("Starting in the terminal");
                terminal_loop(engine)?;
                break;
            }
            ("quit" | "stop" | "EXIT" | "end", _) => {
                info!("Quitting");
                break;
            }
            ("board", x) => {
                info!("Displaying board");
                let b = Board::from_str(x.collect::<String>().trim_matches([' ', '\n', '"']))
                    .map_err(|e| anyhow!("board error: {e}"))?;
                info!("{}", b.print());
            }
            ("display" | "fen", y) => {
                info!("(unchecked) fen display");
                let b = fen_to_str(
                    y.collect::<String>()
                        .trim_matches([' ', '\n', '"'])
                        .to_string(),
                );
                info!("{}", b);
            }
            ("other", _) => {
                // used for testing/prototyping snippets
            }
            ("debug", _) => {
                println!(
                    "depth 6 best move: {}",
                    engine.best_move(Depth(6), Duration::from_secs(60))?
                );
            }
            _ => {
                warn!("unrecognised command {command:?}");
            }
        }
    }

    Ok(())
}
