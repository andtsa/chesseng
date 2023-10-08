use chess::Board;

pub mod parameters;
pub mod material;
pub mod search;
pub mod move_generation;
pub fn play() {
    let board = Board::default();
    println!("{}", board.to_string());
}