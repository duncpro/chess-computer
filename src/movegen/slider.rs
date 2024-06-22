use crate::bitboard::Bitboard;
use crate::coordinates::DefaultCS;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::GameState;
use crate::grid::StandardCoordinate;
use crate::lanescan::lanescan;
use crate::misc::PieceSpecies;

pub fn movegen_sliders<C: CoordinateSystem>(state: &GameState, kind: PieceSpecies)
{
    let mut bb: Bitboard<DefaultCS> = Bitboard::empty();
    bb =  state.species_bbs[kind.into()].get();
    bb &= state.affilia_bbs[state.active_player].get();
    for origin in bb.scan() {
        let origin_stdc: StandardCoordinate = origin.into();
        movegen_slider::<C>(state, origin_stdc);
    }
}

fn movegen_slider<C: CoordinateSystem>(state: &GameState, origin: StandardCoordinate)
{
    let mut bb: Bitboard<C> = lanescan(state, origin);
    bb &= !state.affilia_bbs[state.active_player].get();
    for destin in bb.scan() {
        todo!("add to move queue")
    }
}
