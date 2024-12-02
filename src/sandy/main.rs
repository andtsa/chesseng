//! # Sandy Chess Engine CLI
//! This binary serves as a command line interface for the engine.
//!
//! It can be used in two modes:
//! * play in terminal mode
//! * UCI mode

#![deny(rustdoc::broken_intra_doc_links)]

use std::io::stdin;
use std::ops::BitOr;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use chess::Board;
use chess::Color;
use log::info;
use log::warn;
use sandy_engine::util::bitboard_to_fen;
use sandy_engine::util::fen_to_str;
use sandy_engine::util::Print;
use sandy_engine::Engine;

use crate::player::terminal_loop;
use crate::uci::uci_loop;

/// Interacting with human players
pub mod player;
/// UCI protocol handling
mod uci;

fn main() -> Result<()> {
    println!("Sandy Chess Engine v0.0.0");

    colog::basic_builder()
        .filter(None, log::LevelFilter::Trace)
        .init();

    let engine = Engine::new()?;

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
            ("bitboard" | "bb", mut bb) => {
                info!("Displaying bitboard");
                let b = u64::from_str(bb.next().unwrap())
                    .map_err(|e| anyhow!("bitboard error: {e}"))?;
                info!("{}", bitboard_to_fen(b));
            }
            ("bbx", mut bb) => {
                info!("Displaying bitboard");
                let b = u64::from_str_radix(bb.next().unwrap().trim_start_matches("0x"), 16)
                    .map_err(|e| anyhow!("bitboard error: {e}"))?;
                info!("{}", bitboard_to_fen(b));
            }
            ("f2b", _) => {
                info!("Bitboard from FEN");
                let board = Board::from_str(read_line.trim_start_matches("f2b").trim()).unwrap();
                let bitboard = board
                    .color_combined(Color::White)
                    .bitor(board.color_combined(Color::Black))
                    .0;
                info!("{}", bitboard_to_fen(bitboard));
                info!("{:#016x}", bitboard);
                info!("{}", bitboard);
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
