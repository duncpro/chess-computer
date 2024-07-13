use crate::bitboard::Bitboard;
use crate::coordinates::StandardCS;
use crate::coordinates::CoordinateSystem;
use crate::grid::StandardCoordinate;
use crate::laneutils::lanescan;
use crate::misc::Push;
use crate::mov::PieceMove;
use crate::piece::Species;
use crate::movegen::types::{GeneratedMove, MGContext};

pub fn movegen_sliders<C>(ctx: &mut MGContext<impl Push<GeneratedMove>>,
                          species: Species) where C: CoordinateSystem
{
    let mut bb: Bitboard<StandardCS> = 
        ctx.class(ctx.active_player(), species);
    
    for origin in bb.scan() {
        movegen_slider::<C>(ctx, origin.into());
    }
}

fn movegen_slider<C>(ctx: &mut MGContext<impl Push<GeneratedMove>>,
                     origin: StandardCoordinate) where C: CoordinateSystem
{
    let mut bb = ctx.inspect(|s| lanescan::<C>(&s.bbs, origin));
    
    let friendly_bb = ctx.inspect(|s| 
        s.bbs.affilia_bbs[s.active_player()].get::<C>());
    bb &= !friendly_bb;
    
    for destin in bb.scan() {
        ctx.push_p(PieceMove::new_basic(origin, destin.into()));
    }
}
