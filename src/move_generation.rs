use std::ops::{BitAnd, BitOr, Not};
use chess::{BitBoard, Board, ChessMove, EMPTY, MoveGen};
use chess::BoardStatus::Ongoing;
use chess::Color::{White};
use chess::Piece::{Bishop, Knight, Pawn, Queen, Rook};
use crate::bot::Bot;

pub fn all_moves(board : &Board) -> Vec<ChessMove> {
    let mg = MoveGen::new_legal(board);
    return mg.collect();
}

pub fn ordered_moves(board : &Board) -> Vec<ChessMove> {
    let mut mg = MoveGen::new_legal(board);
    let masks = vec![
        board.pieces(Queen).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Rook).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Bishop).bitor(board.pieces(Knight)).bitand(board.color_combined(board.side_to_move().not())),
        board.pieces(Pawn).bitand(board.color_combined(board.side_to_move().not())),
        BitBoard::new(0b1100000011000000000000000000000000000),
        BitBoard::new(0b1111000011110000111100001111000000000000000000),
        !EMPTY
    ];
    let mut r : Vec<ChessMove> = Vec::new();
    for m in masks {
        mg.set_iterator_mask(m);
        let mvs = mg.by_ref().collect::<Vec<ChessMove>>();
        r.append(&mut mvs.clone());
    }
    return r;
}


pub fn compute_best_move(board : &Board, depth : u32, bot : &Bot) -> ChessMove {
    let maximizing = board.side_to_move() == White;
    let mut best_move = None;
    let mut best_value = if maximizing {f64::MIN} else {f64::MAX};
    let legal_moves = all_moves(board);

    for &mv in &legal_moves {
        let bd = board.make_move_new(mv);

        let value = minimax(&bd, depth - 1, maximizing, f64::MIN, f64::MAX, bot);

        if (value > best_value && maximizing) || (value < best_value && !maximizing) {
            best_value = value;
            best_move = Some(mv);
        }
    }

    best_move.unwrap() // Return the best move found
}


fn minimax(board : &Board, depth : u32, maximizing : bool, alpha : f64, beta : f64, bot : &Bot) -> f64 {
    if depth <= 0 || board.status()!=Ongoing {
        return bot.eval(*board);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    return if maximizing {
        let mut best_value = f64::MIN;
        let legal_moves = all_moves(board);

        for mv in legal_moves {
            let bd = board.make_move_new(mv);

            let value = minimax(&bd, depth - 1, false, alpha, beta, bot);
            best_value = if best_value < value {value} else {best_value};
            alpha = if best_value < alpha {alpha} else {best_value};
            if beta <= alpha {
                break
            }
        }

        best_value
    } else {
        let mut best_value = f64::MAX;
        let legal_moves = all_moves(board);

        for mv in legal_moves {
            let bd = board.make_move_new(mv);

            let value = minimax(&bd, depth - 1, true, alpha, beta, bot);
            best_value = if best_value > value {value} else {best_value};
            beta = if best_value > beta {beta} else {best_value};
            if beta <= alpha {
                break
            }
        }

        best_value
    }
}



