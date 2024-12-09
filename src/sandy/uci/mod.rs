/// UCI search-controls related functionality
pub mod search_controls;
/// UCI time-control related functionality
pub mod time_control;

use std::io::BufRead;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Result;
use chess::Board;
use log::info;
use log::warn;
use sandy_engine::debug::DebugLevel;
use sandy_engine::optlog;
use sandy_engine::opts::opts;
use sandy_engine::opts::setopts;
use sandy_engine::opts::Opts;
use sandy_engine::position::Position;
use sandy_engine::setup::depth::Depth;
use sandy_engine::util::Print;
use sandy_engine::Engine;
use vampirc_uci::parse_one;
use vampirc_uci::Serializable;
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

    // options
    for opt in Opts::register_options() {
        println!("{}", opt.serialize());
    }

    println!("uciok");

    for line in std::io::stdin().lock().lines() {
        let msg: UciMessage = parse_one(&line?);
        optlog!(uci;trace;"Received message: {}", msg);
        match msg {
            UciMessage::Uci => warn!("already in uci mode!"),
            UciMessage::Debug(value) => {
                let curr = opts()?;
                setopts(curr.debug(if value {
                    DebugLevel::debug
                } else {
                    DebugLevel::info
                }))?;
                info!(
                    "debug mode: {}, max_log_level: {:?}, engine opts: {:?}",
                    value,
                    log::max_level(),
                    opts()
                );
            }
            UciMessage::IsReady => {
                // if this is the first time, do setup such as loading the opening book
                // ...
                println!("readyok");
            }
            UciMessage::SetOption { name, value } => {
                match opts()?.receive_option(&name, value.as_deref()) {
                    Err(e) => optlog!(uci;error;"error setting option: {}", e),
                    Ok(opt) => {
                        setopts(opt)?;
                        engine.resize_table(opt.hash_size)?;

                        optlog!(uci;info;
                             "option {name} set to {}.",
                             value.clone().unwrap_or("None".to_string())
                        );
                    }
                }
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
                    engine.board = Default::default();
                } else if let Some(fen) = fen {
                    engine.board = Position::from(Board::from_str(&fen.0).expect("invalid FEN"));
                }
                for mv in moves {
                    engine.board = engine.board.make_move(mv);
                }
                optlog!(uci;info;"fen position: {}", engine.board);
                optlog!(uci;debug;"{}", engine.board.print());
                optlog!(uci;debug;"{}", engine.board);
            }
            UciMessage::Go {
                time_control,
                search_control,
            } => {
                engine.set_search_to(Depth::MAX);
                if let Some(tc) = time_control {
                    optlog!(uci;debug;"time control: {:?}", tc);
                    engine.time_control(tc)?;
                }
                if let Some(sc) = search_control {
                    optlog!(uci;debug;"search control: {:?}", sc);
                    engine.search_control(sc)?;
                }

                // start the engine!
                engine.uci_go()?;
            }
            UciMessage::Stop => {
                // stop the search
                engine.set_search_until(Instant::now())?;
            }
            UciMessage::PonderHit => {}
            UciMessage::Quit => {
                // clean up and EXIT
                optlog!(uci;info;"quitting");
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
                optlog!(uci;warn;"unrecognised message: {}", msg);
                if let Some(err) = err {
                    if opts()?.comm.debug() {
                        optlog!(uci;error;"{:?}", err);
                    } else {
                        optlog!(uci;error;"{}", err);
                    }
                }
            }
        }
    }

    Ok(())
}
