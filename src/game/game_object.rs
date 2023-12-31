use std::cell::Ref;
use std::io::{Error, ErrorKind};
use std::process::exit;
use std::str::FromStr;
use std::time::{Instant, SystemTime};
use chess::{Board, BoardStatus, ChessMove};
use chess::BoardStatus::Checkmate;
use chess::Color::{Black, White};
use crate::bot::Bot;
use crate::Engine;
use crate::game::stockfish_evaluation::Stockfish;
use crate::util::{fen_to_str, fen_to_string_highlighted};
use chess::Piece::Pawn;

pub struct Game {
    pub board : Board,
    pub(crate) white : Option<Bot>,
    pub(crate) black : Option<Bot>,
    start : Instant,
    moves_since_capture : u32,
    turn_count : u32,
    pub ongoing : bool,
    engine : Box<dyn Engine>,
}

impl Game {
    fn end_game(&mut self, message : Option<&str>) {
        println!("{}\nGame lasted for {} seconds", message.unwrap_or(""), self.start.elapsed().as_secs_f64());
        println!("\nFinal Board:\n{}", fen_to_str(self.board.to_string()));
        // TODO print out the parameters of the bots
        self.ongoing = false;
    }

    pub fn new() -> Self {
        Game {
            board : Board::default(),
            white : Option::from(Bot::new()),
            black : Option::from(Bot::new()),
            start : Instant::now(),
            moves_since_capture : 0,
            turn_count : 0,
            ongoing : true,
            engine: Box::new(Stockfish::new()),
        }
    }

    pub fn load(&mut self, fen : &str) {
        self.board = Board::from_str(fen).unwrap();
        if !self.board.is_sane() {
            println!("Invalid FEN string :(");
            exit(1);
        }
    }

    pub fn player_white(&mut self) {
        self.white = None;
    }
    pub fn player_black(&mut self) {
        self.black = None;
    }
    pub fn set_white(&mut self, bot : Bot) {
        self.white = Some(bot);
    }
    pub fn set_black(&mut self, bot : Bot) {
        self.black = Some(bot);
    }

    pub fn null_move(&mut self) -> Result<(),Error> {
        let new_board = self.board.null_move();
        return if new_board == None {
            Err(Error::new(ErrorKind::PermissionDenied, "You are in check!"))
        } else {
            self.board = new_board.unwrap();
            Ok(())
        }
    }
    pub fn next_move(&mut self) {
        if !self.ongoing { return; }
        self.check_state();
        if !self.ongoing { return; }
        let new_move : ChessMove;
        match self.board.side_to_move() {
            White => {
                if self.white.is_none() {
                    new_move = self.player_move();
                } else {
                    new_move = self.bot_move(White);
                }
            }
            Black => {
                if self.black.is_none() {
                    new_move = self.player_move();
                } else {
                    new_move = self.bot_move(Black);
                }
            }
        }

        // When moving we deal with move-counting for draws
        let captures_exist = self.board.piece_on(new_move.get_dest()) != None;
        let is_pawn_move = self.board.piece_on(new_move.get_source()) == Option::from(Pawn);
        if captures_exist || is_pawn_move {
            self.moves_since_capture = 0;
            if self.board.combined().popcnt() < 3 {
                self.end_game(Some("Draw by insufficient material"));
            }
        } else {
            self.moves_since_capture += 1;
        }

        let mut e = 0.0;
        e = self.engine.evaluate(&self.board);

        // finally, we make the move.
        self.board = self.board.make_move_new(new_move);

        println!("\n{}\nTurn #{} Move #{}. [{}{}]", fen_to_string_highlighted(self.board.to_string(), new_move, captures_exist, is_pawn_move), self.turn_count>>1, self.turn_count, if e>0.0{"+"}else{""}, e);
        self.turn_count+=1;
    }

    /// Two checks (so far):
    /// - Game state (checkmate / stalemate)
    /// - draw by 50 move rule
    fn check_state(&mut self) {
        if self.moves_since_capture >= 50 {
            self.end_game(Some("Draw by 50-move rule."));
        }
        if self.board.status() == Checkmate {
            if self.board.side_to_move() == Black {
                println!("White won");
            } else {
                println!("Black won");
            }
            self.end_game(Some("Checkmate!"));
        } else if self.board.status() == BoardStatus::Stalemate {
            self.end_game(Some("Stalemate!"));
        }
    }

    /// Play a game.
    pub fn play(&mut self) {
        self.start = Instant::now();
        println!("Game Started at {}\n{}", chrono::offset::Local::now().time().to_string(), fen_to_str(self.board.to_string()));
        while self.ongoing {
            self.next_move();
        }
        println!("Game Ended.");
    }
}


