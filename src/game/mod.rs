use std::process::exit;
// use chess::Board;
use crate::bot::Bot;
use crate::game::game_object::Game;
use crate::util::Stringify;

// use crate::game::stockfish_evaluation::eval;
//
pub mod stockfish_evaluation;
pub mod game_object;
pub mod making_moves;

const USAGE : &str = "Usage: play -w [p/b] -b [p/b] --continue <fen>
    -w [p/b] : set white to be a player (p) or bot (b)
    -b [p/b] : set black to be a player (p) or bot (b)
    --continue <fen> : continue a game from a given fen string\n";

pub fn run(args : Vec<String>) {
    if args.contains(&String::from("continue")){
        println!("Continuing game");
    } else {
        println!("Starting new game");
    }
    let mut game = Game::new();
    let mut i = 2usize;
    let mut thinking_time = 12000;
    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                thinking_time = args[i+1].parse::<u64>().unwrap();;
                i += 1;
            },
            "-w" => {
                match args[i+1].as_str() {
                    "p" => game.player_white(),
                    "b" | "c" => game.set_white(Bot::new().thinking_time(thinking_time)),
                    _ => { println!("unrecognised pattern `{} {}`.\n{}", args[i], args[i+1], USAGE); exit(1) },
                }
                i += 1;
            },
            "-b" => {
                match args[i+1].as_str() {
                    "p" => game.player_black(),
                    "b" | "c" => game.set_black(Bot::new().thinking_time(thinking_time)),
                    _ => { println!("unrecognised pattern `{} {}`.\n{}", args[i], args[i+1], USAGE); exit(1) },
                }
                i += 1;
            },
            "--continue" => {
                game.load(args[i+1..args.len()].join(" ").as_str());
                println!("{} to move", game.board.side_to_move().stringify());
                break;
            },
            _ => { println!("unrecognised pattern `{} {}`.\n{}", args[i], args[i+1], USAGE); exit(1) },
        }
        i += 1;
    }

    game.play();
}

pub fn run_default() {
    println!("Starting new game");
    let mut game = Game::new();
    game.set_black(Bot::new().thinking_time(12000));
    game.player_white();
    game.play();
}