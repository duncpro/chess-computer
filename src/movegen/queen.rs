use crate::gamestate::ChessGame;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::RankMajorCS;
use crate::coordinates::FileMajorCS;
use crate::misc::Push;
use crate::piece::Species::Queen;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::PMGMove;
use crate::movegen::types::PMGContext;

pub fn movegen_queens(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    movegen_sliders::<FileMajorCS>(ctx, Queen); 
    movegen_sliders::<RankMajorCS>(ctx, Queen);
    movegen_sliders::<ProdiagonalMajorCS>(ctx, Queen);
    movegen_sliders::<AntidiagonalMajorCS>(ctx, Queen);
}
