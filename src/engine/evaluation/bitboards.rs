//! hardcoded bitboard values used for the evaluation function
#![allow(dead_code)]

use chess::BitBoard;
use chess::Piece;

/// a type for one pesto table, basically an 8 by 8 grid of [`Value`]s
pub type PestoTable = [[i16; 8]; 8];

/// a list of pieces who's position is considered in evaluation (all of them)
pub const POS_PIECE_TYPES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

/// Midgame pesto table for the pawn piece type
pub const MG_PAWN_TABLE: PestoTable = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [98, 134, 61, 95, 68, 126, 34, -11],
    [-6, 7, 26, 31, 65, 56, 25, -20],
    [-14, 13, 6, 21, 23, 12, 17, -23],
    [-27, -2, -5, 12, 17, 6, 10, -25],
    [-26, -4, -4, -10, 3, 3, 33, -12],
    [-35, -1, -20, -23, -15, 24, 38, -22],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

/// Endgame pesto table for the pawn piece type
pub const EG_PAWN_TABLE: PestoTable = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [178, 173, 158, 134, 147, 132, 165, 187],
    [94, 100, 85, 67, 56, 53, 82, 84],
    [32, 24, 13, 5, -2, 4, 17, 17],
    [13, 9, -3, -7, -7, -8, 3, 3],
    [4, 7, -6, 1, 0, -5, -1, -8],
    [13, 8, 8, 10, 13, 0, 2, -7],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

/// Midgame pesto table for the knight piece type
pub const MG_KNIGHT_TABLE: PestoTable = [
    [-167, -89, -34, -49, 61, -97, -15, -107],
    [-73, -41, 72, 36, 23, 62, 7, -17],
    [-47, 60, 43, 59, 11, 31, 49, 3],
    [-9, 17, 19, 53, 37, 69, 18, 22],
    [-13, 4, 16, 13, 28, 19, 21, -8],
    [-23, -9, 12, 10, 19, 17, 25, -16],
    [-29, -53, -12, 10, 13, 4, -16, -22],
    [-105, -21, -58, -33, -17, -28, -19, -23],
];

/// Endgame pesto table for the knight piece type
pub const EG_KNIGHT_TABLE: PestoTable = [
    [-58, -38, -13, -28, 8, -33, -23, -45],
    [-28, -10, 13, 9, 10, 4, -9, -20],
    [-25, 17, 19, 10, 23, 17, 25, 5],
    [-13, 4, 16, 13, 28, 19, 21, -8],
    [-27, -11, 8, 2, 7, 6, 10, -15],
    [-53, -34, -23, -15, -22, -18, -31, -52],
    [-75, -6, -27, -16, -17, -20, -31, -65],
    [-59, -36, -18, -16, -37, -10, -42, -67],
];

/// Midgame pesto table for the bishop piece type
pub const MG_BISHOP_TABLE: PestoTable = [
    [-29, 4, -82, -37, -25, -42, 7, -8],
    [-26, 16, -18, -13, 30, 59, 18, -47],
    [-16, 13, 13, 1, 2, 1, 6, -2],
    [3, 19, 24, 15, 15, 18, 10, -7],
    [-9, 39, 39, 47, 31, 27, 43, 4],
    [-13, 25, 13, 32, 19, 13, 10, 2],
    [5, 19, 10, 23, 17, 16, 20, -22],
    [-23, -15, 2, 5, 13, 4, -16, -27],
];

/// Endgame pesto table for the bishop piece type
pub const EG_BISHOP_TABLE: PestoTable = [
    [-14, -21, -11, -8, -7, -9, -17, -24],
    [-8, -4, 7, -12, -3, -13, -2, -9],
    [2, -8, 0, -1, -2, 6, 0, 1],
    [-3, 9, 12, 9, 14, 10, 3, 2],
    [-6, 3, 13, 19, 7, 10, 20, 0],
    [-13, 0, 18, 10, 2, 12, 3, -9],
    [-16, 0, 4, 4, 5, 0, 4, -14],
    [-19, -13, 1, 17, 9, 16, 7, -15],
];

/// Midgame pesto table for the rook piece type
pub const MG_ROOK_TABLE: PestoTable = [
    [32, 42, 32, 51, 63, 9, 31, 43],
    [27, 32, 58, 62, 80, 67, 26, 44],
    [-5, 19, 26, 36, 17, 45, 61, 16],
    [-24, -11, 7, 26, 24, 35, -8, -20],
    [-36, -26, -12, -1, 9, -7, 6, -23],
    [-45, -25, -16, -17, 3, 0, -5, -33],
    [-44, -16, -20, 6, 13, 5, -20, -41],
    [-7, 2, 13, 27, 13, 26, -2, -9],
];

/// Endgame pesto table for the rook piece type
pub const EG_ROOK_TABLE: PestoTable = [
    [13, 10, 18, 15, 12, 12, 8, 5],
    [11, 13, 13, 11, -3, 3, 8, 3],
    [7, 7, 7, 5, 4, 6, 10, 5],
    [8, 6, 9, 17, 16, 17, 10, 5],
    [7, 5, 7, 15, 14, 10, 5, 4],
    [13, 10, 13, 10, 13, 8, 3, 3],
    [7, 7, 7, 7, 7, 12, 12, 7],
    [0, 0, 0, 3, 3, 0, 0, 0],
];

/// Midgame pesto table for the queen piece type
pub const MG_QUEEN_TABLE: PestoTable = [
    [-28, 0, 29, 12, 59, 44, 43, 45],
    [-24, -39, 4, 26, 19, 35, 22, 15],
    [-13, -17, 7, 8, 29, 56, 47, 57],
    [-27, -27, -16, -16, -1, 17, -2, 1],
    [-9, -26, -9, -10, -2, -4, 3, -3],
    [-14, 2, -11, -2, -5, 2, 14, 5],
    [-35, -8, 11, 2, 8, 15, -3, 1],
    [-1, -18, -9, 10, -15, -25, -31, -50],
];

/// Endgame pesto table for the queen piece type
pub const EG_QUEEN_TABLE: PestoTable = [
    [-9, 22, 22, 27, 27, 19, 10, 20],
    [-17, 20, 32, 41, 58, 25, 30, 0],
    [-20, 6, 9, 49, 47, 35, 19, 9],
    [3, 22, 24, 45, 57, 40, 57, 36],
    [-18, 28, 19, 47, 31, 34, 39, 23],
    [-16, 13, 28, 19, 13, 19, 21, 7],
    [-3, 8, 23, 16, 30, 23, 22, 7],
    [-20, -13, 6, 15, 14, 23, 15, 3],
];

/// Midgame pesto table for the king piece type
pub const MG_KING_TABLE: PestoTable = [
    [-65, 23, 16, -15, -56, -34, 2, 13],
    [29, -1, -20, -7, -8, -4, -38, -29],
    [-9, 24, 2, -16, -20, 6, 22, -22],
    [-17, 20, -12, -27, -30, -25, -14, -36],
    [-49, -1, -14, -22, -44, -30, -15, -46],
    [-57, -18, -28, -42, -25, -25, -15, -57],
    [-53, -38, -31, -26, -29, -43, -44, -53],
    [-30, -24, 9, 3, -15, -18, 3, -32],
];

/// Endgame pesto table for the king piece type
pub const EG_KING_TABLE: PestoTable = [
    [-72, -48, -43, -15, -15, -43, -48, -72],
    [-72, -36, -18, 0, 0, -18, -36, -72],
    [-72, -36, 0, 36, 36, 0, -36, -72],
    [-72, -36, 0, 36, 36, 0, -36, -72],
    [-36, -18, 0, 36, 36, 0, -18, -36],
    [-36, -54, -18, 0, 0, -18, -54, -36],
    [-54, -72, -36, -18, -18, -36, -72, -54],
    [-54, -54, -54, -54, -54, -54, -54, -54],
];

/// Midgame Pesto Table
pub const MG_PESTO_TABLE: [PestoTable; 6] = [
    MG_PAWN_TABLE,
    MG_KNIGHT_TABLE,
    MG_BISHOP_TABLE,
    MG_ROOK_TABLE,
    MG_QUEEN_TABLE,
    MG_KING_TABLE,
];

/// Endgame Pesto Table
pub const EG_PESTO_TABLE: [PestoTable; 6] = [
    EG_PAWN_TABLE,
    EG_KNIGHT_TABLE,
    EG_BISHOP_TABLE,
    EG_ROOK_TABLE,
    EG_QUEEN_TABLE,
    EG_KING_TABLE,
];

/// the board's center
const CENTER: BitBoard = BitBoard(0x0000001818000000);
/// a bitboard of the queen-side half of the board
const QUEEN_SIDE: BitBoard = BitBoard(0x0f0f0f0f0f0f0f0f);
/// not sure yet
const CENTER_FILES: BitBoard = BitBoard(0x3c3c3c3c3c3c3c3c);
/// a bitboard of the king-side half of the board
const KING_SIDE: BitBoard = BitBoard(0xf0f0f0f0f0f0f0f0);

/// not sure yet
const KING_FLANK: [BitBoard; 8] = [
    QUEEN_SIDE,
    QUEEN_SIDE,
    QUEEN_SIDE,
    CENTER_FILES,
    CENTER_FILES,
    KING_SIDE,
    KING_SIDE,
    KING_SIDE,
];
