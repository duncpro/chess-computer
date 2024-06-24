use crate::grid::StandardCoordinate;
use crate::piece::Species;

// # `MSPieceMove`

pub struct MSPieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub is_pdj: bool,
    pub promote: Option<Species>
}

impl MSPieceMove {
    pub fn normal(origin: StandardCoordinate, destin: StandardCoordinate) -> Self {
        Self { 
            origin, 
            destin,
            target: destin,
            is_pdj: false,
            promote: None
        }
    }
}

// # `MoveSet`

pub struct MoveSet {
    pub castle_queenside: bool,
    pub castle_kingside: bool,
    pub pmoves: Vec<MSPieceMove>
}
