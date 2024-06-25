use crate::gamestate::SpecialPieceMove;
use crate::piece::Species;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;

// # `MGPieceMove`

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

// # `MGAnyMove`

#[derive(Clone, Copy)]
pub enum MGAnyMove {
    Piece(MGPieceMove),
    Castle(FileDirection)
}
