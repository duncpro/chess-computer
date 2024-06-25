use crate::bitboard::Bitboard;
use crate::coordinates::StandardCS;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::FastPosition;
use crate::grid::StandardCoordinate;
use crate::lane::lanescan;
use crate::misc::Push;
use crate::piece::Species;
use crate::movegen::moveset::MGPieceMove;

pub fn movegen_sliders<C: CoordinateSystem>(state: &FastPosition, kind: Species,
    moves: &mut impl Push<MGPieceMove>)
{
    let mut bb: Bitboard<StandardCS> = 
        state.bbs.class(state.active_player(), kind);
    
    for origin in bb.scan() {
        let origin_stdc: StandardCoordinate = origin.into();
        movegen_slider::<C>(state, origin_stdc, moves);
    }
}

fn movegen_slider<C: CoordinateSystem>(state: &FastPosition, origin: StandardCoordinate,
    moves: &mut impl Push<MGPieceMove>)
{
    let mut bb: Bitboard<C> = lanescan(&state.bbs, origin);
    bb &= !state.bbs.affilia_bbs[state.active_player()].get();
    for destin in bb.scan() {
        moves.push(MGPieceMove::normal(origin, destin.into()))
    }
}
