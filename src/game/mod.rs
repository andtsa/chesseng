// use chess::Board;
use crate::bot::Bot;
use crate::game::game_object::Game;

// use crate::game::stockfish_evaluation::eval;
//
pub mod stockfish_evaluation;
pub mod game_object;
pub mod making_moves;
pub fn run() {
    // let game: Game<Stockfish> = Game::new();
    println!("Starting new game");
    let mut white = Bot::new().thinking_time(20000);
    let mut black = Bot::new().thinking_time(20000);

    let mut game = Game::new();
    game.set_black(black);
    game.set_white(white);

    game.play();
}