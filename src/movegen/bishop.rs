use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::misc::Push;
use crate::movegen::types::GeneratedMove;
use crate::piece::Species::Bishop;
use crate::movegen::slider::movegen_sliders;
use crate::movegen::types::MGContext;

pub fn movegen_bishops(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    movegen_sliders::<ProdiagonalMajorCS>(ctx, Bishop);
    movegen_sliders::<AntidiagonalMajorCS>(ctx, Bishop);
}
