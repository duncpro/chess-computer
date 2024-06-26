use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::gamestate::FastPosition;
use crate::misc::Push;
use crate::piece::Species::Bishop;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::PMGMove;
use crate::movegen::types::PMGContext;

pub fn movegen_bishops(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    movegen_sliders::<ProdiagonalMajorCS>(ctx, Bishop);
    movegen_sliders::<AntidiagonalMajorCS>(ctx, Bishop);
}
