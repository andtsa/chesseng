use std::str::FromStr;

use chess::ChessMove;
use chess::Piece;
use chess::Square;

use crate::setup::depth::Depth;
use crate::setup::values::Value;
use crate::transposition_table::bounds::EvalBound;
use crate::transposition_table::TableEntry;

#[test]
fn test_pack_then_unpack() {
    let key = 0x1234567890abcdef;
    let eval = Value(0x1234);
    let depth = Depth(0x1234);
    let mv = ChessMove::new(
        Square::from_str("a7").unwrap(),
        Square::from_str("a8").unwrap(),
        Some(Piece::Queen),
    );
    let bound = EvalBound::Exact;
    let entry = TableEntry::pack(key, eval, depth, mv, bound, true);
    assert_eq!(eval, entry.eval());
    assert_eq!(depth, entry.depth());
    assert_eq!(mv, entry.mv());
    assert_eq!(bound, entry.bound());
    assert!(entry.is_pv());
}

#[test]
fn test_pack_then_unpack_with_promotion() {
    let key = 0x1234567890abcdef;
    let eval = Value(0x1234);
    let depth = Depth(0x1234);
    let mv = ChessMove::new(
        Square::from_str("a7").unwrap(),
        Square::from_str("a8").unwrap(),
        Some(Piece::Queen),
    );
    let bound = EvalBound::Exact;
    let entry = TableEntry::pack(key, eval, depth, mv, bound, true);
    assert_eq!(eval, entry.eval());
    assert_eq!(depth, entry.depth());
    assert_eq!(mv, entry.mv());
    assert_eq!(bound, entry.bound());
    assert!(entry.is_pv());
}

#[test]
fn test_pack_then_unpack_with_no_promotion() {
    let key = 0x1234567890abcdef;
    let eval = Value(0x1234);
    let depth = Depth(0x1234);
    let mv = ChessMove::new(
        Square::from_str("a7").unwrap(),
        Square::from_str("a8").unwrap(),
        None,
    );
    let bound = EvalBound::Exact;
    let entry = TableEntry::pack(key, eval, depth, mv, bound, false);
    assert_eq!(eval, entry.eval());
    assert_eq!(depth, entry.depth());
    assert_eq!(mv, entry.mv());
    assert_eq!(bound, entry.bound());
    assert!(!entry.is_pv());
}

#[test]
fn test_pack_then_unpack_edge_case_values() {
    let key = 0x1234567890abcdef;
    let eval = Value(0x7fff);
    let depth = Depth(0xffff);
    let mv = ChessMove::new(
        Square::from_str("h8").unwrap(),
        Square::from_str("h1").unwrap(),
        Some(Piece::Queen),
    );
    let bound = EvalBound::UpperBound;
    let entry = TableEntry::pack(key, eval, depth, mv, bound, true);
    assert_eq!(eval, entry.eval());
    assert_eq!(depth, entry.depth());
    assert_eq!(mv, entry.mv());
    assert_eq!(bound, entry.bound());
    assert!(entry.is_pv());
}
