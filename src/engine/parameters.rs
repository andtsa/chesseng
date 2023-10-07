
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
