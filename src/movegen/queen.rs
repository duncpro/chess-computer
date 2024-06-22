use crate::gamestate::GameState;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::RankMajorCS;
use crate::coordinates::FileMajorCS;
use crate::misc::PieceSpecies::Queen;

use super::slider::movegen_sliders;

pub fn movegen_queens(state: &GameState) {
    movegen_sliders::<FileMajorCS>(state, Queen); 
    movegen_sliders::<RankMajorCS>(state, Queen);
    movegen_sliders::<ProdiagonalMajorCS>(state, Queen);
    movegen_sliders::<AntidiagonalMajorCS>(state, Queen);
}
