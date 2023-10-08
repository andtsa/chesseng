
/// The values of each type of chess piece
pub struct PieceValues {
    pawn: f64,
    knight: f64,
    bishop: f64,
    rook: f64,
    queen: f64,
    king: f64,
}
impl PieceValues {
    pub fn new() -> Self {
        PieceValues {
            pawn: 1.0,
            knight: 3.0,
            bishop: 3.0,
            rook: 5.0,
            queen: 10.0,
            king: 100.0,
        }
    }
}

/// Piece positions are stored in a 64-element array, where each element represents the value of the piece at that square.
/// The value of the piece is calculated by multiplying the value of the piece by the value of the square.
/// To reduce space use, each square value is stored as `i32`,
/// which has to be multiplied by the `factor` to get the actual value.
pub struct PiecePositions {
    pawn: [i32; 64],
    knight: [i32; 64],
    bishop: [i32; 64],
    rook: [i32; 64],
    queen: [i32; 64],
    king: [i32; 64],
    factor: f64,
}
impl PiecePositions {
    pub fn new() -> Self {
        Self {
            pawn: [
                0,  0,  0,  0,  0,  0,  0,  0,
                5, 10, 10,-20,-20, 10, 10,  5,
                5, -5,-10,  0,  0,-10, -5,  5,
                0,  0,  0, 20, 20,  0,  0,  0,
                5,  5, 10, 25, 25, 10,  5,  5,
                10, 10, 20, 30, 30, 20, 10, 10,
                50, 50, 50, 50, 50, 50, 50, 50,
                0,  0,  0,  0,  0,  0,  0,  0
            ],
            knight: [
                -50,-40,-30,-30,-30,-30,-40,-50,
                -40,-20,  0,  5,  5,  0,-20,-40,
                -30,  5, 10, 15, 15, 10,  5,-30,
                -30,  0, 15, 20, 20, 15,  0,-30,
                -30,  5, 15, 20, 20, 15,  5,-30,
                -30,  0, 10, 15, 15, 10,  0,-30,
                -40,-20,  0,  0,  0,  0,-20,-40,
                -50,-40,-30,-30,-30,-30,-40,-50
            ],
            bishop: [
                -20,-10,-10,-10,-10,-10,-10,-20,
                -10,  5,  0,  0,  0,  0,  5,-10,
                -10, 10, 10, 10, 10, 10, 10,-10,
                -10,  0, 10, 10, 10, 10,  0,-10,
                -10,  5,  5, 10, 10,  5,  5,-10,
                -10,  0,  5, 10, 10,  5,  0,-10,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -20,-10,-10,-10,-10,-10,-10,-20
            ],
            rook: [
                0,  0,  0,  5,  5,  0,  0,  0,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                5, 10, 10, 10, 10, 10, 10,  5,
                0,  0,  0,  0,  0,  0,  0,  0
            ],
            queen: [
                -20,-10,-10, -5, -5,-10,-10,-20,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -10,  5,  5,  5,  5,  5,  0,-10,
                0,  0,  5,  5,  5,  5,  0, -5,
                -5,  0,  5,  5,  5,  5,  0, -5,
                -10,  0,  5,  5,  5,  5,  0,-10,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -20,-10,-10, -5, -5,-10,-10,-20
            ],
            king: [
                20, 30, 10,  0,  0, 10, 30, 20,
                20, 20,  0,  0,  0,  0, 20, 20,
                -10,-20,-20,-20,-20,-20,-20,-10,
                -20,-30,-30,-40,-40,-30,-30,-20,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30
            ],
            factor: 0.01,
        }
    }
}

pub struct GameStateValues {
    checkmate : f64,
    stalemate : f64,
    draw : f64,
}

pub struct Mobility {
    center_control : f64,
    pawn_moves : f64,
    knight_moves : f64,
    bishop_moves : f64,
    rook_moves : f64,
    queen_moves : f64,
    king_moves : f64,
}

pub struct KingSafety {
    pawn_shield : f64,
    king_tropism : f64,
    edge_proximity : f64,
    castling : f64,
}

pub struct PawnStructure {
    doubled_pawns : f64,
    isolated_pawns : f64,
    backward_pawns : f64,
    passed_pawns : f64,
    pawn_chains : f64,
    pawn_storm : f64,
}

pub struct Coordination {
    bishop_pair : f64,
    rook_pair : f64,
    queen_rook_pair : f64,
    knight_outposts : f64,
    rook_outposts : f64,
    queen_outposts : f64,
    rook_on_open_file : f64,
    queen_on_open_file : f64,
}

pub struct TacticalFeatures {
    pins : f64,
    skewers : f64,
    discovered_attacks : f64,
    double_attacks : f64,
    back_rank_mate : f64,
    knight_forks : f64,
    bishop_forks : f64,
    rook_forks : f64,
    queen_forks : f64,
    king_forks : f64,
}

pub struct BoardControl {
    center_control : f64,
    king_attack : f64,
    king_defense : f64,
    space : f64,
    tempo : f64,
    initiative : f64,
}
