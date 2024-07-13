use crate::coordinates::FileMajorCS;
use crate::coordinates::RankMajorCS;
use crate::misc::Push;
use crate::movegen::types::GeneratedMove;
use crate::piece::Species::Rook;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::MGContext;

pub fn movegen_rooks(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    movegen_sliders::<FileMajorCS>(ctx, Rook);
    movegen_sliders::<RankMajorCS>(ctx, Rook);
}

