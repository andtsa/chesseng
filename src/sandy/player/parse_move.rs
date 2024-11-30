use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use inquire::Select;
use sandy_engine::search::moveordering::ordered_moves;

/// Parse a player move from the terminal
pub fn parse_player_move(pos: &Board) -> Result<ChessMove> {
    let moves = ordered_moves(pos);
    let move_options = moves.0.iter().map(|m| m.to_string()).collect();
    let move_choice = Select::new("Your move (type to filter)", move_options)
        .raw_prompt()?
        .index;

    Ok(moves.0[move_choice])
}
