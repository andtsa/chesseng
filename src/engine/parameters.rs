
/// The values of each type of chess piece
#[derive(PartialEq)]
pub struct PieceValues {
    pub(crate) pawn: f64,
    pub(crate) knight: f64,
    pub(crate) bishop: f64,
    pub(crate) rook: f64,
    pub(crate) queen: f64,
    pub(crate) king: f64,
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
#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub struct GameStateValues {
    pub(crate) checkmate : f64,
    pub(crate) stalemate : f64,
    pub(crate) draw : f64,
}
impl GameStateValues {
    pub fn new() -> Self {
        GameStateValues {
            checkmate : 10000.0,
            stalemate : -50.0,
            draw : -30.0,
        }
    }
}
#[derive(PartialEq)]
pub struct Mobility {
    center_control : f64,
    pawn_moves : f64,
    knight_moves : f64,
    bishop_moves : f64,
    rook_moves : f64,
    queen_moves : f64,
    king_moves : f64,
}
impl Mobility {
    pub fn new() -> Self {
        Self {
            center_control: 1.5,  // Higher value to emphasize control of the center
            pawn_moves: 1.0,      // Base value, pawns generally have limited mobility
            knight_moves: 1.2,    // Knights are more mobile than pawns but less than bishops/rooks
            bishop_moves: 1.3,    // Bishops are valuable for long diagonals
            rook_moves: 1.4,      // Rooks are valuable for files and ranks
            queen_moves: 1.6,     // Queens combine the power of rooks and bishops
            king_moves: 0.8,      // Lower value as king mobility is often risky
        }
    }
}

#[derive(PartialEq)]
pub struct KingSafety {
    pawn_shield : f64,
    king_tropism : f64,
    edge_proximity : f64,
    castling : f64,
}
impl KingSafety {
    pub fn new() -> Self {
        Self {
            pawn_shield: 2.0,     // High value to emphasize the importance of a pawn shield
            king_tropism: -1.5,   // Negative value to indicate that proximity to enemy pieces is generally bad
            edge_proximity: -1.0, // Negative value to indicate that being close to the edge is generally bad
            castling: 1.5,        // Positive value to indicate that the ability to castle is generally good
        }
    }
}
#[derive(PartialEq)]
pub struct PawnStructure {
    doubled_pawns : f64,
    isolated_pawns : f64,
    backward_pawns : f64,
    passed_pawns : f64,
    pawn_chains : f64,
    pawn_storm : f64,
}
impl PawnStructure {
    pub fn new() -> Self {
        Self {
            doubled_pawns: -0.5,    // Negative value to indicate that doubled pawns are generally undesirable
            isolated_pawns: -0.7,   // Negative value to indicate that isolated pawns are generally undesirable
            backward_pawns: -0.6,   // Negative value to indicate that backward pawns are generally undesirable
            passed_pawns: 1.2,      // Positive value to indicate that passed pawns are generally advantageous
            pawn_chains: 0.8,       // Positive value to indicate that pawn chains can be advantageous
            pawn_storm: 0.4,        // Positive but lower value to indicate that pawn storms can be situationally advantageous
        }
    }
}

#[derive(PartialEq)]
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
impl Coordination {
    pub fn new() -> Self {
        Self {
            bishop_pair: 0.7,          // Positive value to indicate that having both bishops is generally advantageous
            rook_pair: 0.3,            // Positive but lower value, as having both rooks is good but not as synergistic as bishop pair
            queen_rook_pair: 0.5,      // Positive value to indicate that a queen and rook can coordinate well
            knight_outposts: 0.6,      // Positive value to indicate that knights on outposts are generally strong
            rook_outposts: 0.4,        // Positive but lower value to indicate that rooks on outposts can be situationally strong
            queen_outposts: 0.2,       // Positive but lower value, as queens are rarely placed on traditional outposts
            rook_on_open_file: 0.8,    // Positive value to indicate that rooks on open files are generally strong
            queen_on_open_file: 0.6,   // Positive value, but slightly lower than for rooks as queens have more mobility options
        }
    }
}

#[derive(PartialEq)]
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
impl TacticalFeatures {
    pub fn new() -> Self {
        Self {
            pins: 1.0,                // Positive value to indicate that pins are generally advantageous
            skewers: 1.1,             // Positive value to indicate that skewers are generally advantageous
            discovered_attacks: 1.2,  // Positive value to indicate that discovered attacks are generally strong
            double_attacks: 1.1,      // Positive value to indicate that double attacks are generally strong
            back_rank_mate: 2.0,      // High value to indicate the critical nature of back-rank mate threats
            knight_forks: 1.3,        // Positive value to indicate that knight forks are generally strong
            bishop_forks: 1.1,        // Positive value to indicate that bishop forks are generally strong
            rook_forks: 1.2,          // Positive value to indicate that rook forks are generally strong
            queen_forks: 1.4,         // Positive value to indicate that queen forks are generally strong
            king_forks: 0.5,          // Lower value, as king forks are rare but can be situationally useful
        }
    }
}

#[derive(PartialEq)]
pub struct BoardControl {
    center_control : f64,
    king_attack : f64,
    king_defense : f64,
    space : f64,
    tempo : f64,
    initiative : f64,
}
impl BoardControl {
    pub fn new() -> Self {
        Self {
            center_control: 1.5,  // Positive value to emphasize the importance of controlling the center
            king_attack: -1.0,    // Negative value to indicate that attacks on the king are generally bad for the defending side
            king_defense: 1.2,    // Positive value to indicate the importance of defending the king
            space: 0.8,           // Positive value to indicate that controlling more squares is generally advantageous
            tempo: 0.6,           // Positive value to indicate that faster development and piece activity are beneficial
            initiative: 0.7,      // Positive value to indicate that having the initiative (ability to make threats) is advantageous
        }
    }
}