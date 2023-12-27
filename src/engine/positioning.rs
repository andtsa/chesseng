use std::ops::BitAnd;
use chess::{Board, Color};
use chess::Color::Black;
use chess::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::bot::Bot;
use crate::util::fen_to_str;

impl Bot {

    /// The actual implementation is going to be quite ugly
    /// but hopefully quite fast. Since there's little to be
    /// said about how piece positions are found, and there
    /// aren't going to be any new pieces soon, we can
    /// hardcode everything except for the position tables -
    /// which in this case is what actually matters
    pub fn get_piece_position_values(&self, board : &Board, side : Color) -> f64 {
        let mut return_value = 0;
        let s = board.color_combined(side);
        let offset = if side == Black { 32 } else { 0 };
        // println!("offset = {}", offset);
        for p in board.pieces(Pawn).bitand(s).into_iter() {
            return_value += self.positions.pawn[(p.to_index() as i32 - offset).abs() as usize];
            // println!("return_value = {}, index = {}", return_value, (p.to_index() as i32 - offset) as usize);
        }
        for n in board.pieces(Knight).bitand(s).into_iter() {
            return_value += self.positions.knight[(n.to_index() as i32 - offset).abs() as usize];
        }
        for b in board.pieces(Bishop).bitand(s).into_iter() {
            return_value += self.positions.bishop[(b.to_index() as i32 - offset).abs() as usize];
        }
        for r in board.pieces(Rook).bitand(s).into_iter() {
            return_value += self.positions.rook[(r.to_index() as i32 - offset).abs() as usize];
        }
        for q in board.pieces(Queen).bitand(s).into_iter() {
            return_value += self.positions.queen[(q.to_index() as i32 - offset).abs() as usize];
        }
        for k in board.pieces(King).bitand(s).into_iter() {
            return_value += self.positions.king[(k.to_index() as i32 - offset).abs() as usize];
        }
        // println!("return_value = {}", return_value);
        return return_value as f64 * self.positions.factor;
    }
}


#[cfg(test)]
mod tests {
    use chess::{Board, Color, MoveGen, Piece};
    use chess::Color::{Black, White};
    use crate::bot::Bot;
    use crate::util::{fen_to_str, Stringify};

    // const GAME : Game = Game::new();

    #[test]
    fn start_evals_are_equal() {
        let board : Board = Board::default();
        let bot = Bot::new();
        assert_eq!(bot.get_piece_position_values(&board, White), bot.get_piece_position_values(&board, Black));
    }
    #[test]
    fn one_move_evals_are_equal() {
        let mut board : Board = Board::default();
        let bot = Bot::new();
        board = board.make_move_new("e2e4".parse().unwrap());
        board = board.make_move_new("e7e5".parse().unwrap());
        assert_eq!(bot.get_piece_position_values(&board, White), bot.get_piece_position_values(&board, Black));
    }
    #[test]
    fn two_move_evals_are_equal() {
        let mut board : Board = Board::default();
        let bot = Bot::new();
        board = board.make_move_new("e2e4".parse().unwrap());
        board = board.make_move_new("e7e5".parse().unwrap());
        board = board.make_move_new("b1c3".parse().unwrap());
        board = board.make_move_new("b8c6".parse().unwrap());
        assert_eq!(bot.get_piece_position_values(&board, White), bot.get_piece_position_values(&board, Black));
    }
    #[test]
    fn test_one_white_pawn_position_value() {
        let mut board : Board = Board::default();
        let bot = Bot::new();
        let starting_eval = bot.get_piece_position_values(&board, White)/bot.positions.factor;
        board = board.make_move_new("e2e4".parse().unwrap());
        let moved_eval = bot.get_piece_position_values(&board, White)/bot.positions.factor;
        let difference = (bot.positions.pawn[/* end position */ 3 + 3 * 8] - bot.positions.pawn[/* start position */ 3 + 1 * 8]) as f64;
        assert_eq!(starting_eval + difference, moved_eval);
    }

    #[test]
    fn test_one_black_pawn_position_value() {
        let mut board : Board = Board::default();
        let bot = Bot::new();
        let starting_eval = bot.get_piece_position_values(&board, White)/bot.positions.factor;
        board = board.null_move().unwrap(); // please remember in the future that if it is white's turn and you make a black move the board will NOT be legal or make sense.... took a while to debug my intelligence deficit
        board = board.make_move_new("e7e5".parse().unwrap());
        let moved_eval = bot.get_piece_position_values(&board, Black)/bot.positions.factor;
        let difference = (bot.positions.pawn[/* end position */ 3 + 3 * 8] - bot.positions.pawn[/* start position */ 3 + 1 * 8]) as f64;
        assert_eq!(starting_eval + difference, moved_eval);
    }

    #[test]
    fn test_all_first_moves() {
        let board : Board = Board::default();
        let bot = Bot::new();
        let starting_eval = bot.get_piece_position_values(&board, White)/bot.positions.factor;
        println!("starting eval = {}", starting_eval);
        for side in [White, Black] {
            for mv in MoveGen::new_legal(&board) {
                let b = match side {
                    White => { board.make_move_new(mv) }
                    Black => { board.null_move().unwrap().make_move_new(mv) }
                };
                let values = match board.piece_on(mv.get_source()).unwrap() {
                    Piece::Pawn => { bot.positions.pawn }
                    Piece::Knight => { bot.positions.knight }
                    Piece::Bishop => { bot.positions.bishop }
                    Piece::Rook => { bot.positions.rook }
                    Piece::Queen => { bot.positions.queen }
                    Piece::King => { bot.positions.king }
                };
                let end = if mv.get_dest().to_index() <= 31 { mv.get_dest().to_index() } else { 63 - mv.get_dest().to_index() };
                let start = if mv.get_source().to_index() <= 31 { mv.get_source().to_index() } else { 63 - mv.get_source().to_index() };
                let diff = (values[end] - values[start]) as f64;
                println!("testing move {}", mv.to_string());
                println!("{}", fen_to_str(b.to_string()));
                assert_eq!(starting_eval + diff, bot.get_piece_position_values(&b, side)/bot.positions.factor, "Move {} should have given {} for {}, but instead gave {}. Start index = {}, start value = {} end index = {}, end value = {}", mv.to_string(), starting_eval + diff, side.stringify(), bot.get_piece_position_values(&board, side)/bot.positions.factor, start, values[start], end, values[end]);
                println!("move {} passed, {}={}", mv.to_string(), starting_eval + diff, bot.get_piece_position_values(&b, side)/bot.positions.factor);
            }
        }
    }
}




fn get_positions_from_bitboard(v : &mut Vec<u64>, mut bb : usize) {
    let mut pointer = 0u64;
    while bb > 0 {
        if bb & 1<<pointer != 0 {
            bb = bb ^ 1<<pointer;
            v.push(pointer);
        }
        pointer += 1;
    }
}