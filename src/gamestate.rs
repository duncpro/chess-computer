use crate::coordinates::{RankMajorCS, CoordinateSystem};
use crate::misc::{SpeciesTable, AffiliationTable, TileSpecies, TileAffiliation, PieceColor};
use crate::bitboard::{Bitboard, RawBitboard};
use crate::grid::GridTable;
use crate::mdbitboard::MDBitboard;

pub struct GameState {
    pub species_bbs: SpeciesTable<Bitboard<RankMajorCS>>,
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
}
