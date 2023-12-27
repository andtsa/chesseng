use std::io::stdin;
use std::process::exit;
use std::time::Instant;
use chess::{ChessMove, Color, MoveGen};
use crate::Engine;
use crate::game::game_object::Game;
use crate::game::stockfish_evaluation::Stockfish;
use crate::util::{all_moves, Stringify};

// TODO make these methods not ugly
impl Game {
    pub(crate) fn player_move(&self) -> ChessMove {
        let move_list = &all_moves(&self.board);
        if move_list.len() <= 1 {
            return move_list[0];
        }
        print!("enter move in SAN notation: ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("IOError: failed to read line");
        let res = buffer.trim_end();
        if res == "" { exit(0) } else if res == "pos" { println!("{}", self.board.to_string()) } else if res == "list" { MoveGen::new_legal(&self.board).for_each(|m| print!("{}, ", m.to_string())) }
        let mut mv = ChessMove::from_san(&self.board, res);
        let mut repeat_q: bool = mv.is_err() || !self.board.legal(mv.clone().unwrap());
        while repeat_q {
            println!("invalid move, retry: ");
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("IOError: failed to read line");
            let res = buffer.trim_end();
            mv = ChessMove::from_san(&self.board, res);
            repeat_q = mv.is_err() || !self.board.legal(mv.clone().unwrap());
        }
        return mv.clone().unwrap();
    }

    pub(crate) fn bot_move(&mut self, side : Color) -> ChessMove {
        let bot = match side {
            Color::White => self.white.as_mut().unwrap(),
            Color::Black => self.black.as_mut().unwrap(),
        };
        let now = Instant::now();
        println!("computing best move for {}", self.board.side_to_move().stringify());
        let mut mv = bot.next_move(&self.board);
        let mut repeat_q: bool = !self.board.legal(mv);
        while repeat_q {
            println!("Computer made illegal move {}, retrying", mv.to_string());
            mv = bot.next_move(&self.board);
            repeat_q = !self.board.legal(mv);
        }

        println!("bot computed move in {} milliseconds", now.elapsed().as_millis());
        return mv;
    }
}
