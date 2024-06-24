use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::gamestate::GameState;
use crate::piece::Species::Bishop;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MoveSet;

pub fn movegen_bishops(state: &GameState, moves: &mut MoveSet) {
    movegen_sliders::<ProdiagonalMajorCS>(state, Bishop, moves);
    movegen_sliders::<AntidiagonalMajorCS>(state, Bishop, moves);
}
