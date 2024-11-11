pub mod search_controls;
pub mod time_control;

use std::io::BufRead;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Result;
use chess::Board;
use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use sandy_engine::setup::depth::Depth;
use sandy_engine::util::Print;
use sandy_engine::DebugLevel::Debug;
use sandy_engine::DebugLevel::Info;
use sandy_engine::Engine;
use vampirc_uci::parse_one;
use vampirc_uci::UciMessage;

use crate::uci::search_controls::SearchControl;
use crate::uci::time_control::TimeControl;

/// UCI main loop
///
/// Receives permanent ownership of the [`Engine`] for this execution of the
/// CLI.
pub fn uci_loop(mut engine: Engine) -> Result<()> {
    println!("id name Sandy Chess Engine");
    println!("id author {}", env!("CARGO_PKG_AUTHORS"));

    // options, todo.

    println!("uciok");

    for line in std::io::stdin().lock().lines() {
        let msg: UciMessage = parse_one(&line?);
        trace!("Received message: {}", msg);
        match msg {
            UciMessage::Uci => warn!("already in uci mode!"),
            UciMessage::Debug(value) => {
                engine.opts.debug(if value { Debug } else { Info });
                info!(
                    "debug mode: {}, log_level: {:?}, engine opts: {:?}",
                    value,
                    log::max_level(),
                    engine.opts
                );
            }
            UciMessage::IsReady => {
                // if this is the first time, do setup such as loading the opening book
                // ...
                println!("readyok");
            }
            UciMessage::SetOption { name, value } => {
                println!("unrecognised option: {}, ignoring value {value:?}", name)
            }
            UciMessage::Register { .. } => {}
            UciMessage::UciNewGame => {
                // clear any existing game state, such as transposition tables
                // or search history ...
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                if startpos {
                    engine.board = Board::default();
                } else if let Some(fen) = fen {
                    engine.board = Board::from_str(&fen.0).expect("invalid FEN");
                }
                for mv in moves {
                    engine.board = engine.board.make_move_new(mv);
                }
                info!("fen position: {}", engine.board);
                if engine.opts.cd() {
                    info!("{}", engine.board.print());
                    debug!("{}", engine.board);
                }
            }
            UciMessage::Go {
                time_control,
                search_control,
            } => {
                engine.set_search_to(Depth::MAX);
                if let Some(tc) = time_control {
                    debug!("time control: {:?}", tc);
                    engine.time_control(tc)?;
                }
                if let Some(sc) = search_control {
                    debug!("search control: {:?}", sc);
                    engine.search_control(sc)?;
                }

                // start the engine!
                engine.uci_go()?;
            }
            UciMessage::Stop => {
                // stop the search
                engine.set_search_until(Instant::now());
            }
            UciMessage::PonderHit => {}
            UciMessage::Quit => {
                // clean up and EXIT
                info!("quitting");
                break;
            }
            UciMessage::Id { .. } => {}
            UciMessage::UciOk => {}
            UciMessage::ReadyOk => {}
            UciMessage::BestMove { .. } => {}
            UciMessage::CopyProtection(_) => {}
            UciMessage::Registration(_) => {}
            UciMessage::Option(_) => {}
            UciMessage::Info(_) => {}
            UciMessage::Unknown(msg, err) => {
                if msg.trim().is_empty() {
                    continue;
                }
                warn!("unrecognised message: {}", msg);
                if let Some(err) = err {
                    if engine.opts.cd() {
                        error!("{:?}", err);
                    } else {
                        error!("{}", err);
                    }
                }
            }
        }
    }

    Ok(())
}
