use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::PieceSpecies::Bishop;
use crate::misc::PieceSpecies::Rook;
use crate::movegen::slider::movegen_sliders;

use super::moveset::MoveSet;

pub fn movegen_rooks(state: &GameState, moves: &mut MoveSet) {
    movegen_sliders::<FileMajorCS>(state, Rook, moves);
    movegen_sliders::<RankMajorCS>(state, Rook, moves);
}

