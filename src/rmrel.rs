//! This module defines the *relative* rank-major tile coordinate
//! system. 
//! 
//! Coordinates in this system are relative to the active-player.
//! For example, the active-player's base rank occupies indices 1..8,
//! and the opponents base rank occupies indices 56..64.
//!
//! This is a specialized coordinate system used primarily during
//! pawn move generation. See the general-purpose coordinate system
//! [`StandardCoordinate`] in [`crate::grid`], and the more specialized
//! absolute major coordinate systems defined in [`crate::coordinates`].

use crate::bitboard::RawBitboard;
use crate::bits::bitscan;
use crate::coordinates::RankMajorCS;
use crate::coordinates::Coordinate;
use crate::grid::StandardCoordinate;
use crate::piece::Color;

/// An involution between relative coordinates and absolute coordinates.
fn convert_rmrel_coord(input: u8, active: Color) -> u8 {
    // - The relative and absolute coordinates are equivalent
    //   when white is the active player.
    // - When black is the active player we must invert the rank index.
    assert!(input < 64);
    let (input_rank, input_file) = (input / 8, input % 8);
    let output_rank = (7 * active.index()) as i8 +
         ((-2 * (active.index() as i8) + 1) * (input_rank as i8));
    assert!(output_rank < 8 && output_rank >= 0);
    return ((output_rank as u8) * 8) + input_file;
}

/// Resolves a *relative* rank-major tile coordinate to a [`StandardCoordinate`].
/// This is the inverse of [`absolutize`].
pub fn absolutize(relc: u8, active_player: Color)
-> StandardCoordinate
{
    let abs_rm_index = convert_rmrel_coord(relc, active_player);
    let abs_rm_coord: Coordinate<RankMajorCS> = Coordinate::from_index(abs_rm_index);
    return StandardCoordinate::from(abs_rm_coord);
}

/// Resolves a [`StandardCoordinate`] to a *relative* rank-major tile coordinate.
/// This is the inverse of [`absolutize`].
pub fn relativize(abs_coord: StandardCoordinate, active: Color) -> u8 {
    let abs_rm_coord = Coordinate::<RankMajorCS>::from(abs_coord);
    return convert_rmrel_coord(abs_rm_coord.index(), active);
}
