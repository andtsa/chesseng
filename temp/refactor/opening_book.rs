
pub struct OpeningBook{
    positions : HashMap<Board, Vec<(ChessMove, f64)>>
}

impl OpeningBook {
    pub fn new() -> Self {
        OpeningBook {
            positions : HashMap::from([
                (Board::default(), vec![(ChessMove::from_san(&Board::default(), "e4").unwrap(), 1.0)])])
        }
    }

    pub fn next(&self, board : &Board) -> Option<ChessMove> {
        if let Some(moves) = self.positions.get(board) {
            // Select a move based on random selection
            return Some(moves[rand::thread_rng().gen_range(0..moves.len())].0);
            // Return the chosen move
        }
        None
    }
}