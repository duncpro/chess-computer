use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::gamestate::GameState;
use crate::misc::PieceSpecies::Bishop;
use crate::movegen::slider::movegen_sliders;

pub fn movegen_bishops(state: &GameState) {
    movegen_sliders::<ProdiagonalMajorCS>(state, Bishop);
    movegen_sliders::<AntidiagonalMajorCS>(state, Bishop);
}
