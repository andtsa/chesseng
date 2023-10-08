use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use chess::{Board, ChessMove};
use chess::BoardStatus::{Checkmate, Stalemate};
use chess::Color::{Black, White};
use chess::Piece::Pawn;
use crate::Engine;
use crate::engine::parameters::{BoardControl, Coordination, GameStateValues, KingSafety, Mobility, PawnStructure, PiecePositions, PieceValues, TacticalFeatures};

///
// #[derive(PartialEq)]
pub struct Bot {
    pub(crate) pieces : PieceValues,
    pub(crate) positions : PiecePositions,
    pub(crate) game_state : GameStateValues,
    pub(crate) coordination : Coordination,
    pub(crate) tactical : TacticalFeatures,
    pub(crate) pawn_structure : PawnStructure,
    pub(crate) mobility : Mobility,
    pub(crate) king_safety : KingSafety,
    pub(crate) board_control : BoardControl,
    pub search_time : u64,
    pub transposition_table : Arc<RwLock<HashMap<u64,(i32,f64)>>>,
    pub idle_moves : i32,
}
impl Bot {
    pub fn new() -> Self {
        Bot {
            pieces : PieceValues::new(),
            positions : PiecePositions::new(),
            game_state : GameStateValues::new(),
            coordination: Coordination::new(),
            tactical: TacticalFeatures::new(),
            pawn_structure: PawnStructure::new(),
            mobility: Mobility::new(),
            king_safety: KingSafety::new(),
            board_control: BoardControl::new(),
            search_time : 5000,
            transposition_table : Arc::new(RwLock::new(HashMap::new())),
            idle_moves : 0,
        }
    }
}

impl Engine for Bot {
    /// The evaluation function of our bot.
    /// takes in a board object and returns:
    /// - positive if white is winning
    /// - negative if black is winning
    /// measured in pawn-values. One pawn is worth 1.0
    /// TODO:
    /// break soon if the advantage is very big (save as much computation time as possible)
    /// use neural network for 50% of the score.
    fn evaluate(&self, board: &Board) -> f64 {
        // repeatedly add evaluation components in order of importance and
        // check if it's over a threshold for "difference big enough that
        // we don't care about precision so much"
        let mut eval: f64 = 0.0;
        let side_multiplier = -2.0*(board.side_to_move().to_index()as f64-0.5);
        // first the game state: a checkmate is worth more than a million pieces
        if board.status() == Checkmate {
            return -1.0 * side_multiplier * self.game_state.checkmate;
        }

        eval += self.get_material_value(board, White);
        eval -= self.get_material_value(board, Black);

        // a stalemate is a win for the side with the material disadvantage
        if board.status() == Stalemate {
            return self.game_state.stalemate * eval;
        }

        // there is more than a queen difference, position isn't going
        // to make a world of difference. We quickly break here
        if f64::abs(eval) >= 8.0 {
            return eval;
        }


        if self.idle_moves > 20 {
            eval *= self.game_state.draw;
        }
        eval
    }

    /// If the evaluation function is the heart of our bot, this is the brain
    /// By judging all our options for the next moves, we must pick one as the best
    /// TODO:
    /// search breadth-first with two levels of depth-first
    /// (iterative deepening) such that
    /// a) we always calculate our move last
    /// b) we can stop whenever and return the best move from the last fully calculated level.
    fn next_move(&mut self, board: &Board) -> ChessMove {
        let new_move = self.iterative_deepening_minimax(board, board.side_to_move()==White, Duration::from_millis(self.search_time));
        let captures_exist = board.piece_on(new_move.get_dest()) != None;
        let is_pawn_move = board.piece_on(new_move.get_source()) == Option::from(Pawn);
        if captures_exist || is_pawn_move {
            self.idle_moves = 0;
        } else {
            self.idle_moves += 1;
        }
        new_move
    }
}

// some util functions for the bot struct
impl Bot {
    /// Set thinking time per move, in milliseconds
    pub fn thinking_time(mut self, time : u64) -> Self {
        self.search_time = time;
        self
    }
}