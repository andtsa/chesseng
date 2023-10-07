extern crate chesseng;

use chesseng::engine::parameters::{BoardControl, Coordination, GameStateValues, KingSafety, Mobility, PawnStructure, PiecePositions, PieceValues, TacticalFeatures};

pub struct Bot {
    pieces : PieceValues,
    positions : PiecePositions,
    game_state : GameStateValues,
    coordination : Coordination,
    tactical : TacticalFeatures,
    pawn_structure : PawnStructure,
    mobility : Mobility,
    king_safety : KingSafety,
    board_control : BoardControl,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            pieces : PieceValues::new(),
            positions : PiecePositions::new(),
            game_state : GameStateValues::new(),
            coordination: Coordination::new(),
            tactical: TacticalFeatures::new(),
            pawn_structure: PawnStructure::new(),
            mobility: Mobility::new(),
            king_safety: KingSafety::new(),
            board_control: BoardControl::new(),
        }
    }
}

impl Engine for Bot {

}