// #![feature(const_trait_impl)]
/*! **TODO**
- [ ] benchmark different search function setups to optimise for move generation.
- [ ] research move ordering techniques and benchmark implementations to find best end-result speed
- [ ] find more ways to prune search tree branches for unpromising paths
- [ ] implement iterative deepening on top of DFS for minimax
- [ ] set iterative deepening end to timestamp instead of depth, to let engine think deeper when time is available
- [ ] create each search branch on a different thread in compute_best_move to make things faster

- [ ] add more theory to existing evaluation function
- [ ] create environment for genetic algorithm optimisation of parameters
- [ ] look into gradient descent optimisation for tuning evaluation parameters

- [ ] research optimal CNN setup
- [ ] implement neural network
- [ ] train using test.db lichess database
- [ ] see if LSTM or recurrent NN is viable for this purpose
bitboards for NN:
white pawns    |  black pawns
white knights  |  black knights
white bishops  |  black bishops
white rooks    |  black rooks
white queens   |  black queens
white kings    |  black kings
 */
extern crate chesseng;
extern crate chess;

use chess::{Board};
use chesseng::{experiments, game};
use chesseng::bot::Bot;
use chesseng::trial;
use chesseng::engine;
use chesseng::game::game_object::Game;
// use chesseng::game::stockfish_evaluation::{eval, Stockfish};
use chesseng::util::fen_to_str;

fn main() {
    println!("Sandy Chess Engine v0.0");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "test" => { trial::run(); },
            "engine" => { engine::play() }
            "default" | "def" | "d" => { game::run_default() }
            // "uci" => { uci::run(); },
            "try" | "experiment" | "experiments" => { experiments::run() }
            "play" | "game" | "continue" | "resume" => { game::run(args) }
            _ => {
                println!("Hello World!");
            },
        }
    } else {
        println!("Default execution");
        let mut game = Game::new();
        game.set_black(Bot::new().thinking_time(5000));
        game.set_white(Bot::new().thinking_time(5000));
        game.play();
        // println!("{}",fen_to_str(Board::default().to_string()));
    }
}


