use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::SegVec;
use crate::piece::Species::Rook;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_rooks(state: &GameState, moves: &mut SegVec<MGPieceMove>) {
    movegen_sliders::<FileMajorCS>(state, Rook, moves);
    movegen_sliders::<RankMajorCS>(state, Rook, moves);
}

