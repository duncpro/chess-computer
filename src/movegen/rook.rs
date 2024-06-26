use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::FastPosition;
use crate::misc::Push;
use crate::piece::Species::Rook;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::PMGMove;
use crate::movegen::types::PMGContext;

pub fn movegen_rooks(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    movegen_sliders::<FileMajorCS>(ctx, Rook);
    movegen_sliders::<RankMajorCS>(ctx, Rook);
}

