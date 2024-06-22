use crate::bitboard::Bitboard;
use crate::bitboard::MDBitboard;
use crate::bitboard::RawBitboard;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::coordinates::StandardCS;
use crate::getbit;
use crate::grid::FileDirection;
use crate::grid::GridTable;
use crate::grid::StandardCoordinate;
use crate::misc::PieceColor;
use crate::misc::PieceSpecies;
use crate::misc::OptionPieceColor;
use crate::misc::OptionPieceSpecies;
use crate::misc::OptColorTable;
use crate::misc::OptSpeciesTable;
use crate::setbit;

pub struct GameState {
    pub affilia_rel_bbs: OptColorTable<RawBitboard>,
    pub pawn_rel_bb: RawBitboard,
    pub mdboard: MDBoard,
    pub species_lut: GridTable<OptionPieceSpecies>,
    pub affilia_lut: GridTable<OptionPieceColor>,
    pub active_player: PieceColor,
    pub movelog: Vec<MovelogEntry>,
    pub crights: CastlingRights
}

impl GameState {
    /// Calculates the [`Coordinate`] of the active-player's king.
    pub fn king<C: CoordinateSystem>(&self) -> Coordinate<C> {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb  = self.mdboard.species_bbs[OptionPieceSpecies::King].get();
        bb &= self.mdboard.affilia_bbs[self.active_player].get();
        return bb.single();
    }

    /// Calculates the [`StandardCoordinate`] of the active-player's king.
    pub fn king_stdc(&self) -> StandardCoordinate {
        self.king::<StandardCS>().into()
    }
}

pub struct MovelogEntry {}

pub struct MDBoard {
    // 7 species * 4 directions = 28 bitboards
    // 28 bitboards * 8 bytes each = 224 bytes
    pub species_bbs: OptSpeciesTable<MDBitboard>,
    // 3 affiliations * 4 directions = 12 bitboards
    // 12 bitboards * 8 bytes each = 96 bytes
    pub affilia_bbs: OptColorTable<MDBitboard>
    // So in total MDBoard has memory expenditure of
    // 224 + 96 = 320 bytes, which is quite small.
}

impl MDBoard {
    pub fn occupancy<C: CoordinateSystem>(&self) -> Bitboard<C> {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb |= self.affilia_bbs[OptionPieceColor::White].get();
        bb |= self.affilia_bbs[OptionPieceColor::Black].get();
        return bb;
    }

    /// Computes a [`Bitboard`] of all pieces of the given class.
    /// That is, all pieces matching the given `color` and `species`.
    pub fn class<C: CoordinateSystem>(&self, color: PieceColor, species: PieceSpecies)
    -> Bitboard<C> 
    {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb =  self.affilia_bbs[color].get();
        bb &= self.species_bbs[species].get();
        return bb;
    }
}

// # Castling Rights

#[derive(Clone, Copy)]
pub struct CastlingRights { data: u8 }

impl CastlingRights {
    pub fn get(self, side: FileDirection, color: PieceColor) -> bool {
        let index = 2 * color.index() + side.index();
        return getbit!(self.data, index);
    }

    pub fn set(&mut self, side: FileDirection, color: PieceColor, value: bool)
    {
        let index = 2 * color.index() + side.index();
        self.data &= !(1 << index);
        self.data |= (1 << index) * (value as u8);
    }
}
