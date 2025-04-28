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

use anyhow::Result;
use anyhow::anyhow;
use chess::Board;
use log::error;
use log::info;
use log::warn;
use sandy_engine::Engine;
use sandy_engine::util::Print;
use sandy_engine::util::fen_to_str;

use crate::player::terminal_loop;
use crate::uci::uci_loop;

/// Interacting with human players
pub mod player;
/// UCI protocol handling
mod uci;

fn main() -> Result<()> {
    println!(
        "Sandy Chess Engine v{} [{}]",
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) || cfg!(test) {
            "DEBUG"
        } else {
            "RELEASE"
        }
    );

    colog::basic_builder()
        .filter(
            None,
            if cfg!(test) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            },
        )
        .init();

    // take the default panic hook, and make sure that the *entire* process is
    // terminated on panic. this significantly improves debugging across
    // multiple threads.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler
        default_hook(panic_info);
        // human readable error
        error!("panic occurred: {panic_info}, exiting sandy engine.");
        // UCI readable error
        println!("info string panic occurred: {panic_info}");
        // and exit the process
        std::process::exit(1);
        // NOTE: this *will* leak resources.
        // since the process is killed though, the OS will clean up after us,
        // eventually.
    }));

    let mut engine: Engine = Engine::new()?;

    if cfg!(feature = "perf") {
        let x = engine.best_move(
            sandy_engine::setup::depth::Depth(9),
            Duration::from_secs(60),
        )?;
        println!("bestmove {x}");
        return Ok(());
    }

    let mut read_line = String::new();
    loop {
        read_line.clear();
        stdin().read_line(&mut read_line)?;
        let command = read_line.to_ascii_lowercase();
        if command.trim().is_empty() {
            continue;
        }
        let mut parts = command.split_ascii_whitespace();
        let cmd_name = parts.next().unwrap();
        let cmd_body = read_line.trim_start_matches(cmd_name).trim();
        match (cmd_name, parts) {
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
            ("board", _) => {
                info!("Displaying board");
                let b = Board::from_str(cmd_body).map_err(|e| anyhow!("board error: {e}"))?;
                info!("{}", b.print());
            }
            ("display" | "fen", _) => {
                info!("(unchecked) fen display");
                let b = fen_to_str(cmd_body.to_string());
                info!("{b}");
            }
            ("other", _) => {
                // used for testing/prototyping snippets
            }
            _ => {
                warn!("unrecognised command {command:?}");
            }
        }
    }

    Ok(())
}
