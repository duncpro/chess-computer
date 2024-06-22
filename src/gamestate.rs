use crate::bitboard::Bitboard;
use crate::bitboard::MDBitboard;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::grid::GridTable;
use crate::misc::PieceColor;
use crate::misc::TileAffiliation;
use crate::misc::TileSpecies;
use crate::misc::AffiliationTable;
use crate::misc::SpeciesTable;

pub struct GameState {
    pub species_bbs: SpeciesTable<MDBitboard>,
    pub affilia_bbs: AffiliationTable<MDBitboard>,
    pub species_lut: GridTable<TileSpecies>,
    pub affilia_lut: GridTable<TileAffiliation>,
    pub active_player: PieceColor,
    pub movelog: Vec<MovelogEntry>
}

pub struct MovelogEntry {}

impl GameState {
    pub fn occupancy<C: CoordinateSystem>(&self) -> Bitboard<C> {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb |= self.affilia_bbs[TileAffiliation::White].get();
        bb |= self.affilia_bbs[TileAffiliation::Black].get();
        return bb;
    }

    /// Calculates the coordinate of the active-player's king.
    pub fn king<C: CoordinateSystem>(&self) -> Coordinate<C> {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb  = self.species_bbs[TileSpecies::King].get();
        bb &= self.affilia_bbs[self.active_player].get();
        return bb.single();
    }
}
