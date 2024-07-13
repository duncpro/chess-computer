use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::RankMajorCS;
use crate::coordinates::FileMajorCS;
use crate::misc::Push;
use crate::movegen::types::GeneratedMove;
use crate::piece::Species::Queen;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::MGContext;

pub fn movegen_queens(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    movegen_sliders::<FileMajorCS>(ctx, Queen); 
    movegen_sliders::<RankMajorCS>(ctx, Queen);
    movegen_sliders::<ProdiagonalMajorCS>(ctx, Queen);
    movegen_sliders::<AntidiagonalMajorCS>(ctx, Queen);
}
