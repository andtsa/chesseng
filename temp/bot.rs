use std::collections::HashMap;
use std::fs;
use std::ops::BitAnd;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use chess::{BitBoard, Board, Color, Piece};
use chess::Color::{Black, White};
use chess::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use rand::{Rng, thread_rng};
//use crate::evaluation::{check_val, knight_moves, piece_value_weight, piece_values, pinning_val, possible_moves_val, squares_covered_by_side};

/// An instance of a chess engine.
/// Provides methods ``new()``, `eval(&Board)`, `get_search_depth()`, `mutate(Degree:f64)`, `printout()`.
///
/// `PARAMS` is stored as a vector, with each attribute of the engine's evaluation having a set position in the vector.
///
/// [\[0\]] : pawn_val  = value of 1 pawn
///
/// [\[1\]] : rook_val  = value of 1 rook
///
/// [\[2\]] : knight_val  = value of 1 knight
///
/// [\[3\]] : bishop_val  = value of 1 bishop
///
/// [\[4\]] : queen_val  = value of 1 queen
///
/// [\[5\]] : king_val  = value of each king
///
/// [\[6\]] : piece_value_weight  = how much do (total) piece values affect the total evaluation
///
/// [\[7\]] : n_of_moves_weight  = how much does being able to make a lot of moves influence eval
///
/// [\[8\]] : covered_squares_weight  = weight param for number of squares attacked (covered) by a side
///
/// [\[9\]] : pinning_val  = how much is pinning an opponent's piece worth
///
/// [\[10\]] : check_val  = how much is putting the other king in check worth
///
/// [\[11\]] : check_piece_multiplier  = at how many pieces is a check worth 1.0?
///
/// [\[12\]] : knight_movement_base  = default number of possible knight moves
///
/// [\[13\]] : knight_movement_coef  = coefficient for knight movement in eval
///
/// [\[14\]] : stalemate_val  = how much is a stalemate worth for the winning side? (should be negative, winner doesn't want a stalemate, loser does)
///
/// [\[15\]] : checkmate_val  = how much is checkmate worth? (A LOT)
///
/// [\[16\]] : hanging_piece_val  = how much is a hanging piece worth?
///
/// [\[17\]] : piece_position_coef  = how much is each piece position worth?
///
/// [\[18\]] : piece_position_value  = how much is having a good (overall) piece position worth?
///
/// [\[19\]] : x  =
///

#[derive(PartialEq)]
pub struct Bot {
    pub params: Vec<f64>,

    pub piece_values_map: HashMap<Piece, f64>,
    pub search_depth: u32,
}

impl Bot {
    pub fn new() -> Self {
        let mut r = Bot {
            params: vec![
                1.0,    // pawn_val
                5.0,    // rook_val
                3.0,    // knight_val
                3.0,    // bishop_val
                10.0,   // queen_val
                25.0,   // king_val
                3.0,    // piece_value_weight
                0.001,  // n_of_moves_weight
                0.03,   // covered_squares_weight
                0.01,   // pinning_val
                0.4,    // check_val
                16.0,   // check_piece_multiplier
                12.0,   // knight_movement_base
                0.1,    // knight_movement_coef
                -20.0,  // stalemate_val
                10000.0,// checkmate_val
                0.8,    // hanging_piece_val
                0.1,    // piece_position_coef
                0.05,   // piece_position_value
            ],

            piece_values_map: HashMap::new(),
            search_depth: 0,
        };

        r.piece_values_map.insert(chess::Piece::Pawn, r.params[0]);
        r.piece_values_map.insert(chess::Piece::Rook, r.params[1]);
        r.piece_values_map.insert(chess::Piece::Knight, r.params[2]);
        r.piece_values_map.insert(chess::Piece::Bishop, r.params[3]);
        r.piece_values_map.insert(chess::Piece::Queen, r.params[4]);
        r.piece_values_map.insert(chess::Piece::King, r.params[5]);

        return r;
    }
    pub fn eval(&self, board : Board) -> f64 {
        match board.status() {
            chess::BoardStatus::Stalemate => {return self.params[14] * piece_values(board, self.piece_values_map.clone());}
            chess::BoardStatus::Checkmate => {return (board.side_to_move().to_index()as f64 - 0.5) * 2.0 * self.params[15]}
            _ => {}
        }

        let mut e : f64 = 0.0;
        e += piece_value_weight(board, self.params[6]) * piece_values(board, self.piece_values_map.clone());
        e += self.params[7] * possible_moves_val(board);
        e += pinning_val(board, self.params[9], self.piece_values_map.clone());
        e += check_val(board, self.params[11], self.params[10]);
        e += self.params[8] * squares_covered_by_side(board, White);
        e -= self.params[8] * squares_covered_by_side(board, Black);
        e += self.params[13] * (knight_moves(board,White) - self.params[12]);
        e -= self.params[13] * (knight_moves(board,Black) - self.params[12]);
        e += self.params[18] * piece_position_values(board, self.params[18], White);
        e -= self.params[18] * piece_position_values(board, self.params[18], Black);
        return e;
    }

    pub fn get_search_depth(&self, board : &Board) -> u32 {
        if self.search_depth == 0 {
            let d= board.color_combined(board.side_to_move()).popcnt();
            match d {
                14..=16 => { 5 }
                12..=13 => { 5 }
                10..=11 => { 4 }
                8..=9 => { 4 }
                6..=7 => { 5 }
                1..=5 => { 6 }
                _ => { 5 }
            }
        } else {
            self.search_depth
        }
    }

    pub fn printout(&self) -> String {
        let mut r = String::new();
        r += "params: vec![";
        for p in &self.params {
            r += &*(p.to_string() + ", ");
        }
        r += "];\n";
        return r;
    }

    pub fn mutate_options(&mut self, ax : f64, b : f64, n : u32) {
        let range_to_mutate = 0..=n;
        for x in range_to_mutate {
            self.params[x as usize] += dx(ax) + b;
        }
    }

    pub fn mutate(&mut self) {
        let mut gn = thread_rng();
        let genes : [usize; 3] = gn.gen();
        for x in genes {
            if x < self.params.len() as usize {
                self.params[x] += dx(dx(1.0));
            }
        }
    }

    /// mutate the parameter vector by multiplying it with a mutation matrix.
    ///
    /// matrix should have dimensions m x n where n=dim(params)
    // pub fn matrix_mutate(&mut self, m : Array2<f64>) {
    //     let p = Array1::from(self.params.clone());
    //     let r = m.dot(&p);
    //     if r.len() != self.params.len(){panic!("Mutation changed param vector dimensions! (Old:{},New:{}) Try resizing mutation matrix.", self.params.len(), r.len());}
    //     self.params = r.to_vec()
    // }

    pub fn save(&self, board : Board) {
        // let s : u64 = (self.params.clone().into_iter().sum::<f64>()*10000.0) as u64;
        let s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let name : String = "saves/".to_owned() + &s.to_string() + ".txt";
        println!("{name}");
        let content = self.printout() + &*board.to_string();
        fs::write(name, content).expect("Unable to write file :( ");
    }
}




pub fn dx(deg:f64) ->f64{
    let mut gn = rand::thread_rng();
    let x : f64 = gn.gen();
    (x-0.5)*deg
}

impl Clone for Bot {
    fn clone(&self) -> Self {
        let mut r = Bot {
            params: vec![
                self.params[0],
                self.params[1],
                self.params[2],
                self.params[3],
                self.params[4],
                self.params[5],
                self.params[6],
                self.params[7],
                self.params[8],
                self.params[9],
                self.params[10],
                self.params[11],
                self.params[12],
                self.params[13],
                self.params[14],
                self.params[15],
                self.params[16],
                self.params[17],
                self.params[18],
            ],

            piece_values_map: HashMap::new(),
            search_depth: self.search_depth,
        };

        r.piece_values_map.insert(chess::Piece::Pawn, r.params[0]);
        r.piece_values_map.insert(chess::Piece::Rook, r.params[1]);
        r.piece_values_map.insert(chess::Piece::Knight, r.params[2]);
        r.piece_values_map.insert(chess::Piece::Bishop, r.params[3]);
        r.piece_values_map.insert(chess::Piece::Queen, r.params[4]);
        r.piece_values_map.insert(chess::Piece::King, r.params[5]);

        return r;
    }
}