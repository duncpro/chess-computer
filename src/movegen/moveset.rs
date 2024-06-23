use crate::gamestate::PieceMoveKind;
use crate::grid::StandardCoordinate;
use crate::misc::OptionPieceSpecies;

// # `MSPieceMove`

pub struct MSPieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub kind: PieceMoveKind    
}

impl MSPieceMove {
    pub fn normal(origin: StandardCoordinate, destin: StandardCoordinate) -> Self {
        let target = destin;
        Self { origin, destin, target, kind: PieceMoveKind::Normal }
    }
}

// # `MoveSet`

pub struct MoveSet {
    pub castle_queenside: bool,
    pub castle_kingside: bool,
    pub pmoves: Vec<MSPieceMove>
}
