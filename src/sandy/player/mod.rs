/// Module for playing against the engine in the terminal
mod parse_move;

use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use chess::Board;
use chess::BoardStatus;
use chess::Color;
use log::error;
use log::info;
use sandy_engine::setup::depth::Depth;
use sandy_engine::util::Print;
use sandy_engine::Engine;

use crate::player::parse_move::parse_player_move;

/// Main loop for playing against the engine in the terminal
pub fn terminal_loop(mut engine: Engine) -> Result<()> {
    match inquire::Select::new(
        "Playing game in terminal",
        vec!["new game", "continue existing"],
    )
    .raw_prompt()?
    .index
    {
        0 => engine.board = Board::default(),
        1 => loop {
            let fen = inquire::Text::new("Enter FEN:").prompt()?;
            match Board::from_str(&fen) {
                Ok(b) => {
                    engine.board = b;
                    break;
                }
                Err(e) => {
                    error!("Invalid FEN: {}", e);
                }
            }
        },
        _ => unreachable!(),
    }

    // select player black or white
    let player =
        [Color::White, Color::Black][inquire::Select::new("You play as", vec!["white", "black"])
            .raw_prompt()?
            .index];

    let mut search_depth = Depth::MAX;
    let mut search_time = Duration::from_secs(5);
    if inquire::Confirm::new("Edit engine settings? (default: movetime 5 seconds)")
        .with_default(false)
        .prompt()?
    {
        search_depth = Depth(
            2 * inquire::CustomType::<u16>::new("New max search depth (moves)")
                .with_default(search_depth.0)
                .prompt()?,
        );
        search_time = Duration::from_millis(
            inquire::CustomType::<u64>::new("New max move time (milliseconds)")
                .with_default(search_time.as_millis() as u64)
                .prompt()?,
        );
    }

    info!("{}", engine.board.print());

    loop {
        let mv = if engine.board.side_to_move() == player {
            parse_player_move(&engine.board)?
        } else {
            engine.best_move(search_depth, search_time)?
        };
        let capture = engine.board.piece_on(mv.get_dest()).is_some();
        engine.board = engine.board.make_move_new(mv);

        info!("{}", engine.board.print_move(mv, capture));

        match engine.board.status() {
            BoardStatus::Ongoing => continue,
            BoardStatus::Stalemate => {
                info!("Stalemate!");
                break;
            }
            BoardStatus::Checkmate => {
                info!("Checkmate!");
                break;
            }
        }
    }

    Ok(())
}
