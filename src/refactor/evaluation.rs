use std::collections::{HashMap};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXor, Not};
use chess::{Board, BitBoard, Piece, Color, get_pawn_attacks, get_king_moves, get_rook_moves, get_bishop_moves, get_knight_moves};
use chess::Color::{Black, White};
use chess::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::move_generation::all_moves;


// /// how many pieces can [side] capture in the next move.
// fn hanging_pieces(board : Board, side : Color) -> usize {
//     if
//     let mut moves = MoveGen::new_legal(&board){};
//     moves.set_iterator_mask(*board.color_combined(side));
//     moves.len();
//
//     return 0;
// }


pub fn squares_covered_by_side(board : Board, side : Color) -> f64 {
    let mut bb = BitBoard::new(0);
    let blockers = board.combined();

    // first, pawns.
    for s in board.pieces(Pawn).bitand(board.color_combined(side)).into_iter() {
        bb.bitor_assign(get_pawn_attacks(s, side, blockers.bitxor(BitBoard::from_square(s))));
    }

    // second, king (easiest ones)
    bb.bitor_assign(get_king_moves(board.king_square(side)));

    for rs in board.color_combined(side).bitand(board.pieces(Rook)).into_iter() {
        bb.bitor_assign(get_rook_moves(rs, blockers.bitxor(BitBoard::from_square(rs))));
    }

    for q in board.color_combined(side).bitand(board.pieces(Queen)).into_iter() {
        bb.bitor_assign(get_rook_moves(q, blockers.bitxor(BitBoard::from_square(q))));
        bb.bitor_assign(get_bishop_moves(q, blockers.bitxor(BitBoard::from_square(q))));
    }

    // almost done, now Bishop diagonals.
    for bs in board.color_combined(side).bitand(board.pieces(Bishop)).into_iter() {
        bb.bitor_assign(get_bishop_moves(bs, blockers.bitxor(BitBoard::from_square(bs))));
    }

    // finally, knights
    for ks in board.pieces(Knight).bitand(board.color_combined(side)).into_iter() {
        bb.bitor_assign(get_knight_moves(ks));
    }

    bb.bitand_assign(board.color_combined(side).not());//.bitxor(BitBoard::new(u64::MAX)));

    return bb.popcnt() as f64;
}


pub fn knight_moves(board:Board, side : Color) -> f64 {
    let mut bb=BitBoard::new(0);
    for k in board.color_combined(side).bitand(board.pieces(Knight)).into_iter() {
        bb.bitor_assign(get_knight_moves(k));
    }

    return bb.popcnt() as f64;
}

pub fn piece_value_weight(board : Board, pvw : f64) -> f64 {
    pvw * (8.0 / (board.combined().popcnt() as f64 + 8.0))
}

pub fn piece_values(board : Board, pv : HashMap<Piece, f64>) -> f64 {
    return one_piece_val(board,Pawn, pv.get(&Pawn).unwrap()) +
        one_piece_val(board,Rook, pv.get(&Rook).unwrap()) +
        one_piece_val(board,Bishop, pv.get(&Bishop).unwrap()) +
        one_piece_val(board,Knight, pv.get(&Knight).unwrap()) +
        one_piece_val(board,King, pv.get(&King).unwrap()) +
        one_piece_val(board,Queen, pv.get(&Queen).unwrap());
}

pub fn one_piece_val(board : Board, piece : Piece, val : &f64) -> f64 {
    return ( val * (board.pieces(piece).bitand(board.color_combined(White)).popcnt() as f64)) +
        ( -val * (board.pieces(piece).bitand(board.color_combined(Black)).popcnt() as f64));
}

pub fn possible_moves_val(board : Board) -> f64 {
    let turn : f64 = if board.side_to_move()==White {1.0} else {-1.0};
    let my_moves = all_moves(&board).len() as f64;
    let b = board.null_move();
    return if b == None {
        (-my_moves) * turn
    } else {
        (my_moves - (all_moves(&b.unwrap()).len() as f64)) * turn
    }
}

pub fn check_val(board : Board, cpm : f64, val : f64) -> f64 {
    (cpm / (board.combined().popcnt() as f64)) * (val * (board.checkers().bitand(board.color_combined(White)).popcnt()as f64)) + (-val *  (board.checkers().bitand(board.color_combined(Black)).popcnt() as f64))
}

pub fn pinning_val(board : Board, pv : f64, spv : HashMap<Piece, f64>) -> f64 {
    let mut r : f64 = 0.0;

    if board.side_to_move() == White {
        r += pv * pin_val(board, White, spv.clone());
        let b = board.null_move();
        if b == None {
            return r;
        } else {
            r -= pv * pin_val(board, Black, spv.clone());
        }
    } else {
        r += pv * pin_val(board, Black, spv.clone());
        let b = board.null_move();
        if b == None {
            return r;
        } else {
            r -= pv * pin_val(board, White, spv.clone());
        }
    }
    return r;
}

pub fn pin_val(board : Board, side : Color, spv : HashMap<Piece, f64>) -> f64 {
    let mut b = board;
    if b.side_to_move()!=side {
        if b.null_move()==None {
            return board.pinned().popcnt() as f64 * if side == White {0.5} else {-0.5};
        } else {
            b = b.null_move().unwrap();
        }
    }
    let mut r : f64 = 0.0;
    for pin in b.pinned().into_iter() {
        match board.piece_on(pin) {
            Some(Pawn) => {r+=spv.get(&Pawn).unwrap()},
            Some(Bishop) => {r+=spv.get(&Bishop).unwrap()},
            Some(Knight) => {r+=spv.get(&Knight).unwrap()},
            Some(Rook) => {r+=spv.get(&Rook).unwrap()},
            Some(Queen) => {r+=spv.get(&Queen).unwrap()},
            _=>{}
        }
    }
    -r
}

