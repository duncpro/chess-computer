use crate::gamestate::SpecialPieceMove;
use crate::piece::Species;
use crate::grid::StandardCoordinate;

// # `MSPieceMove`

#[derive(Clone, Copy)]
pub struct MGPieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub special: Option<SpecialPieceMove>,
    pub promote: Option<Species>
}

impl MGPieceMove {
    pub fn normal(origin: StandardCoordinate, destin: StandardCoordinate) -> Self {
        Self { 
            origin, 
            destin,
            target: destin,
            special: None,
            promote: None
        }
    }
}

// # `MoveSet`

pub struct MoveSet {
    pub castle_queenside: bool,
    pub castle_kingside: bool,
    pub pmoves: Vec<MGPieceMove>
}
