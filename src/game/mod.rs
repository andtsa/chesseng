use crate::game::evaluate::Stockfish;
use crate::game::game_object::Game;

mod making_moves;
mod evaluate;
mod game_object;

pub fn run() {
    let game: Game<Stockfish> = Game::new();
    
}