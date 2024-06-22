use crate::bitboard::Bitboard;
use crate::coordinates::{ProdiagonalMajorCS, Coordinate};
use crate::gamestate::GameState;
use crate::grid::StandardCoordinate;
use crate::misc::TileSpecies;
use crate::sliders::get_slidescan;

pub fn movegen_bishops(state: &GameState) {

}

fn movegen_bishop_pd(state: &GameState) {
    
    let mut bb: Bitboard<ProdiagonalMajorCS> = Bitboard::empty();
    bb =  state.species_bbs[TileSpecies::Bishop].get();
    bb &= state.affilia_bbs[state.active_player].get();
    
    for origin in bb.scan() {
        let origin_stdc: StandardCoordinate = origin.into();

        
    }
}

fn slidescan_prodiagonal(state: &GameState, origin: StandardCoordinate) 
-> Bitboard<ProdiagonalMajorCS>
{
    let occupancy: Bitboard<ProdiagonalMajorCS> = state.occupancy();
    let prodiagonal = origin.prodiagonal();
    let base: Coordinate<ProdiagonalMajorCS> = prodiagonal.base().into();
    let occ_bl = occupancy.copy_bitlane(base);
    let destin_bl = get_slidescan(origin.prodiagonal_offset(), occ_bl);
    let mut destins: Bitboard<ProdiagonalMajorCS> = Bitboard::empty();
    destins = Bitboard::from_bitlane(base, destin_bl, prodiagonal.length());
    
    todo!()
}

