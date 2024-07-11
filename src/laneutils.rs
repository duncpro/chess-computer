use crate::bitboard::Bitboard;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::ChessGame;
use crate::gamestate::Bitboards;
use crate::grid::StandardCoordinate;
use crate::sliders::get_slidelimit;
use crate::sliders::get_slidescan;

pub fn lanescan<C: CoordinateSystem>(board: &Bitboards, origin: StandardCoordinate)
-> Bitboard<C>
{
    let lane = C::get_lane(origin);
    let occupancy: Bitboard<C> = board.occupancy();
    let base: Coordinate<C> = lane.base.into();
    let occ_bl = occupancy.copy_bitlane(base);
    let destin_bl = get_slidescan(lane.local_origin, occ_bl);
    return Bitboard::from_bitlane(base, destin_bl, lane.length);
}

pub fn lanelimit<C: CoordinateSystem>(board: &Bitboards, origin: StandardCoordinate)
-> Bitboard<C>
{
    let lane = C::get_lane(origin);
    let occupancy: Bitboard<C> = board.occupancy();
    let base: Coordinate<C> = lane.base.into();
    let occ_bl = occupancy.copy_bitlane(base);
    let destin_bl = get_slidelimit(lane.local_origin, occ_bl);
    return Bitboard::from_bitlane(base, destin_bl, lane.length);
}

