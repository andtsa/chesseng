use chess::{BitBoard, Board, ChessMove, EMPTY, MoveGen};
use crate::util::all_moves;

pub const PIECE_VALUES: [i32; 6] = [1, 3, 3, 5, 9, 1000];  // Pawn, Knight, Bishop, Rook, Queen, King

pub fn generate_moves(board : &Board) -> Vec<ChessMove> {
    // return all_moves(board);
    let mut mg = MoveGen::new_legal(board);
    let mut moves: Vec<ChessMove> = Vec::new();

    mg.set_iterator_mask(*board.combined());
    let mut attacking_moves = mg.by_ref().collect::<Vec<ChessMove>>();
    // Inline sorting based on MVV-LVA score
    attacking_moves.sort_unstable_by(|&a, &b| {
        let a_victim_value = PIECE_VALUES[board.piece_on(a.get_dest()).unwrap().to_index()];
        let a_aggressor_value = PIECE_VALUES[board.piece_on(a.get_source()).unwrap().to_index()];
        let b_victim_value = PIECE_VALUES[board.piece_on(b.get_dest()).unwrap().to_index()];
        let b_aggressor_value = PIECE_VALUES[board.piece_on(b.get_source()).unwrap().to_index()];

        let a_score = a_victim_value - a_aggressor_value;
        let b_score = b_victim_value - b_aggressor_value;

        b_score.cmp(&a_score)  // Sort in descending order
    });

    moves.append(&mut attacking_moves);

    mg.set_iterator_mask(BitBoard::new(0b1100000011000000000000000000000000000));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(BitBoard::new(0b1111000011110000111100001111000000000000000000));
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    mg.set_iterator_mask(!EMPTY);
    moves.append(&mut mg.by_ref().collect::<Vec<ChessMove>>());

    moves
}