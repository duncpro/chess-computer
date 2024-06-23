use crate::gamestate::PieceMoveKind;
use crate::grid::StandardCoordinate;
use crate::misc::OptionPieceSpecies;

// # `MSPieceMove`

pub struct MSPieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub kind: PieceMoveKind,
    pub promote: OptionPieceSpecies  
}

impl MSPieceMove {
    pub fn normal(origin: StandardCoordinate, destin: StandardCoordinate) -> Self {
        Self { 
            origin, 
            destin,
            target: destin,
            kind: PieceMoveKind::Normal,
            promote: OptionPieceSpecies::None
        }
    }
}

// # `MoveSet`

pub struct MoveSet {
    pub castle_queenside: bool,
    pub castle_kingside: bool,
    pub pmoves: Vec<MSPieceMove>
}
