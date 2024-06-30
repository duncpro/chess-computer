use crate::bitboard::Bitboard;
use crate::bitboard::MDBitboard;
use crate::bitboard::RawBitboard;
use crate::attack::is_attacked;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::coordinates::StandardCS;
use crate::crights::CastlingRights;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::movegen::dispatch::count_legal_moves;
use crate::movegen::types::PMGMove;
use crate::piece::Color;
use crate::piece::ColorTable;
use crate::piece::Piece;
use crate::piece::PieceGrid;
use crate::piece::Species;
use crate::piece::SpeciesTable;

// # `FastPosition`

#[derive(Default)]
pub struct FastPosition {
    pub bbs: Bitboards,
    pub p_lut: PieceGrid,
    pub movelog: Vec<MovelogEntry>,
    pub crights: CastlingRights,
    pub halfmoveclock: u16
}

impl FastPosition {
    pub fn active_player(&self) -> Color { self.bbs.active_player } 
}

// # Movelog

#[derive(Clone, Copy)]
pub struct MovelogEntry {
    pub prev_crights: CastlingRights,
    pub prev_halfmoveclock: u16,
    pub lmove: LoggedMove,
}

#[derive(Clone, Copy)]
pub enum LoggedMove {
    Castle(FileDirection),
    Piece(LoggedPieceMove)
}

#[derive(Clone, Copy)]
pub struct LoggedPieceMove {
    pub mgmove: PMGMove,
    pub capture: Option<Piece>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpecialPieceMove { Promote = 1, PawnDoubleJump = 2 }

// # `Bitboards`

#[derive(Default)]
pub struct Bitboards {
    // ## MDBitboards
    pub species_bbs: SpeciesTable<MDBitboard>,
    pub affilia_bbs: ColorTable<MDBitboard>,

    // ## Relative Bitboards
    pub affilia_rel_bbs: ColorTable<RawBitboard>,
    pub pawn_rel_bb: RawBitboard,
    pub active_player: Color
}

impl Bitboards {
    pub fn rel_occupancy(&self) -> RawBitboard {
        let mut bb: RawBitboard = 0;
        bb |= self.affilia_rel_bbs[Color::White];
        bb |= self.affilia_rel_bbs[Color::Black];
        return bb;
    }

    pub fn occupancy<C: CoordinateSystem>(&self) -> Bitboard<C> {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb |= self.affilia_bbs[Color::White].get();
        bb |= self.affilia_bbs[Color::Black].get();
        return bb;
    }

    /// Computes a [`Bitboard`] of all pieces of the given class.
    /// That is, all pieces matching the given `color` and `species`.
    pub fn class<C: CoordinateSystem>(&self, color: Color, species: Species)
    -> Bitboard<C> 
    {
        let mut bb: Bitboard<C> = Bitboard::empty();
        bb =  self.affilia_bbs[color].get();
        bb &= self.species_bbs[species].get();
        return bb;
    }

    /// Determines if the active-player's king is in check.
    pub fn is_check(&self) -> bool { 
        is_attacked(&self, locate_king_stdc(&self))
    }
}


/// Calculates the [`Coordinate`] of the active-player's king.
pub fn locate_king<C: CoordinateSystem>(board: &Bitboards) -> Coordinate<C> {
    let mut bb: Bitboard<C> = Bitboard::empty();
    bb  = board.species_bbs[Species::King].get();
    bb &= board.affilia_bbs[board.active_player].get();
    return bb.single();
}

/// Calculates the [`StandardCoordinate`] of the active-player's king.
pub fn locate_king_stdc(board: &Bitboards) -> StandardCoordinate {
    locate_king::<StandardCS>(board).into()
}


// # Status

pub enum GameStatus {
    Complete(GameResult),
    Incomplete
}

pub enum GameResult {
    Diff(/* victor */ Color),
    Tie
}

pub fn status(state: &mut FastPosition) -> GameStatus {
    let has_move = count_legal_moves(state) > 0;
    if has_move { return GameStatus::Incomplete; }
    if !state.bbs.is_check() {
        return GameStatus::Complete(GameResult::Tie)
    }
    let victor = state.active_player().oppo();
    return GameStatus::Complete(GameResult::Diff(victor));
}
