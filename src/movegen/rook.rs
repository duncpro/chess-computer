use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::FastPosition;
use crate::misc::Push;
use crate::piece::Species::Rook;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_rooks(state: &FastPosition, moves: &mut impl Push<MGPieceMove>) {
    movegen_sliders::<FileMajorCS>(state, Rook, moves);
    movegen_sliders::<RankMajorCS>(state, Rook, moves);
}

