use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use inquire::Select;
use sandy_engine::search::moveordering::ordered_moves;

/// Parse a player move from the terminal
pub fn parse_player_move(pos: &Board) -> Result<ChessMove> {
    let move_options = ordered_moves(pos)
        .into_iter()
        .map(|m| m.to_string())
        .collect();
    let move_choice = Select::new("Your move (type to filter)", move_options)
        .raw_prompt()?
        .index;

    ordered_moves(pos).get(move_choice).map(|m| m.0)
}
