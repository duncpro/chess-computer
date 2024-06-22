use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::PieceSpecies::Bishop;
use crate::misc::PieceSpecies::Rook;
use crate::movegen::slider::movegen_sliders;

pub fn movegen_rooks(state: &GameState) {
    movegen_sliders::<FileMajorCS>(state, Rook);
    movegen_sliders::<RankMajorCS>(state, Rook);
}

