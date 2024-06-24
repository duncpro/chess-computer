use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::gamestate::GameState;
use crate::piece::Species::Bishop;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MoveSet;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_bishops(state: &GameState, moves: &mut Vec<MGPieceMove>) {
    movegen_sliders::<ProdiagonalMajorCS>(state, Bishop, moves);
    movegen_sliders::<AntidiagonalMajorCS>(state, Bishop, moves);
}
