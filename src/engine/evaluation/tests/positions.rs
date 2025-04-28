use std::str::FromStr;

use chess::Board;
use chess::Color;
use chess::Square;

use crate::evaluation::bitboards::EG_PAWN_TABLE;
use crate::evaluation::material::interpolate;
use crate::evaluation::position::piece_position_benefit_for_side;
use crate::evaluation::position::sq_pi;
use crate::setup::values::Value;

#[test]
fn test_correct_indices() {
    let sq = Square::from_str("h7").unwrap();
    let (row, col) = sq_pi(sq, Color::White);
    assert_eq!(row, 1, "row={row}");
    assert_eq!(col, 7, "col={col}");
    assert_eq!(EG_PAWN_TABLE[row][col], 187, "{}", EG_PAWN_TABLE[row][col]);

    let sq = Square::from_str("h2").unwrap();
    let (row, col) = sq_pi(sq, Color::Black);
    assert_eq!(row, 1, "{row}");
    assert_eq!(col, 7, "{col}");
    assert_eq!(EG_PAWN_TABLE[row][col], 187, "{}", EG_PAWN_TABLE[row][col]);
}

// pub const EG_PAWN_TABLE: PestoTable = [
//     [0, 0, 0, 0, 0, 0, 0, 0],
//     [178, 173, 158, 134, 147, 132, 165, 187],
//     [94, 100, 85, 67, 56, 53, 82, 84],
//     [32, 24, 13, 5, -2, 4, 17, 17],
//     [13, 9, -3, -7, -7, -8, 3, 3],
//     [4, 7, -6, 1, 0, -5, -1, -8],
//     [13, 8, 8, 10, 13, 0, 2, -7],
//     [0, 0, 0, 0, 0, 0, 0, 0],
// ];

#[test]
fn test_single_white_pawn() {
    let pos = Board::from_str("8/P7/8/2k2K2/8/8/8/8 w - - 0 1").unwrap();
    let interp = interpolate(&pos);
    let eval = piece_position_benefit_for_side(&pos, Color::White, interp);
    assert_eq!(eval, Value(178), "{eval}");
}

#[test]
fn test_single_black_pawn() {
    let pos = Board::from_str("8/8/8/2k2K2/8/8/p7/8 b - - 0 1").unwrap();
    let interp = interpolate(&pos);
    let eval = piece_position_benefit_for_side(&pos, Color::Black, interp);
    assert_eq!(eval, Value(178), "{eval}");
}

#[test]
fn check_mirror_positions() {
    let position = Board::from_str("8/ppp1pppp/3p4/8/2k2K2/8/8/8 w - - 0 1").unwrap();
    // let mirrored = Board::from_str("6K1/6P1/6pP/PPP5/3PN2p/1p3k2/1pp2p2/3n4 w - -
    // 0 1").unwrap();
    let mirrored = Board::from_str("8/8/8/2K2k2/8/4P3/PPPP1PPP/8 w - - 0 1").unwrap();

    // panic!("{} {}", position.print(), mirrored.print());

    let interp_a = interpolate(&position);
    let interp_b = interpolate(&mirrored);
    assert_eq!(interp_a, interp_b, "{interp_a:?} {interp_b:?}");

    let eval = piece_position_benefit_for_side(&position, Color::White, interp_a);
    let eval_mirrored = piece_position_benefit_for_side(&mirrored, Color::Black, interp_b);
    assert_eq!(eval, eval_mirrored, "{eval}");
}
