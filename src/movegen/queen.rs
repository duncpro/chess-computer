use crate::gamestate::GameState;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::RankMajorCS;
use crate::coordinates::FileMajorCS;
use crate::piece::Species::Queen;
use crate::movegen::moveset::MoveSet;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_queens(state: &GameState, moves: &mut Vec<MGPieceMove>) {
    movegen_sliders::<FileMajorCS>(state, Queen, moves); 
    movegen_sliders::<RankMajorCS>(state, Queen, moves);
    movegen_sliders::<ProdiagonalMajorCS>(state, Queen, moves);
    movegen_sliders::<AntidiagonalMajorCS>(state, Queen, moves);
}
