use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::gamestate::FastPosition;
use crate::misc::Push;
use crate::piece::Species::Bishop;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_bishops(state: &FastPosition, moves: &mut impl Push<MGPieceMove>) {
    movegen_sliders::<ProdiagonalMajorCS>(state, Bishop, moves);
    movegen_sliders::<AntidiagonalMajorCS>(state, Bishop, moves);
}
