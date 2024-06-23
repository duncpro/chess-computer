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
    pub bbs: Bitboards,
    pub species_lut: GridTable<OptionPieceSpecies>,
    pub affilia_lut: GridTable<OptionPieceColor>,
    pub movelog: Vec<MovelogEntry>,
    pub crights: CastlingRights
}

impl GameState {
    // ## Accessors
    pub fn active_player(&self) -> PieceColor { self.bbs.active_player }
}

// # `PieceMoveKind`

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceMoveKind {
    Normal,
    Promote,
    PawnDoubleJump
}

// # Movelog

pub struct MovelogEntry {
    pub crights: CastlingRights,
    pub lmove: LoggedMove
}

pub enum LoggedMove {
    Castle(FileDirection),
    Piece(LoggedPieceMove)
}

#[derive(Clone, Copy)]
pub struct LoggedPieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub kind: PieceMoveKind,
    pub capture: OptionPieceSpecies,
}

// # `Bitboards`

pub struct Bitboards {
    // ## MDBitboards
    // 7 species * 4 directions = 28 bitboards
    // 28 bitboards * 8 bytes each = 224 bytes
    pub species_bbs: OptSpeciesTable<MDBitboard>,
    // 3 affiliations * 4 directions = 12 bitboards
    // 12 bitboards * 8 bytes each = 96 bytes
    pub affilia_bbs: OptColorTable<MDBitboard>,

    // ## Relative Bitboards
    // 3 affiliations = 3 bitboards
    // 3 bitboards * 8 bytes each = 24 bytes
    pub affilia_rel_bbs: OptColorTable<RawBitboard>,
    // 1 bitboard * 8 bytes each = 8 bytes
    pub pawn_rel_bb: RawBitboard,
    pub active_player: PieceColor
    // So in total `Bitboards` has memory expenditure of
    // 224 + 96 + 24 + 8 + 1 = 321 bytes.
}

impl Bitboards {
    pub fn rel_occupancy(&self) -> RawBitboard {
        let mut bb: RawBitboard = 0;
        bb |= self.affilia_rel_bbs[PieceColor::White];
        bb |= self.affilia_rel_bbs[PieceColor::Black];
        return bb;
    }

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


/// Calculates the [`Coordinate`] of the active-player's king.
pub fn locate_king<C: CoordinateSystem>(board: &Bitboards) -> Coordinate<C> {
    let mut bb: Bitboard<C> = Bitboard::empty();
    bb  = board.species_bbs[OptionPieceSpecies::King].get();
    bb &= board.affilia_bbs[board.active_player].get();
    return bb.single();
}

/// Calculates the [`StandardCoordinate`] of the active-player's king.
pub fn locate_king_stdc(board: &Bitboards) -> StandardCoordinate {
    locate_king::<StandardCS>(board).into()
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
