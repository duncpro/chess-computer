use crate::bitboard::Bitboard;
use crate::bitboard::MDBitboard;
use crate::bitboard::RawBitboard;
use crate::attack::is_attacked;
use crate::cache::Cache;
use crate::cache::HashChars;
use crate::cache::IncrementalHash;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::coordinates::StandardCS;
use crate::crights::CastlingRights;
use crate::grid::Side;
use crate::grid::StandardCoordinate;
use crate::movegen::dispatch::count_legal_moves;
use crate::mov::PieceMove;
use crate::piece::Color;
use crate::piece::ColorTable;
use crate::piece::Piece;
use crate::piece::PieceGrid;
use crate::piece::Species;
use crate::piece::SpeciesTable;

// # `ChessGame`

#[derive(Clone, PartialEq, Eq)]
pub struct ChessGame {
    pub bbs: Bitboards,
    pub p_lut: PieceGrid,
    pub movelog: Vec<MovelogEntry>,
    pub crights: CastlingRights,
    pub halfmoveclock: u16,
    pub hash: IncrementalHash,
    pub has_castled: ColorTable<bool>
}

impl ChessGame {
    pub fn active_player(&self) -> Color { self.bbs.active_player } 

    /// Constructs an *empty* board with white as the active player
    /// and obviously no castling rights for either side (there are no pieces).
    pub fn new(hash_ch: HashChars) -> Self {
        let bbs = Bitboards::new();
        let p_lut = PieceGrid::empty();
        let movelog: Vec<MovelogEntry> = Vec::new();
        let crights = CastlingRights::NONE;
        let halfmoveclock = 0u16;
        let mut hash = IncrementalHash::new(hash_ch);
        hash.toggle_crights(crights);
        return Self { bbs, p_lut, movelog, crights, halfmoveclock,
            hash, has_castled: ColorTable::default() };
    }
}

// # Movelog

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovelogEntry {
    pub prev_crights: CastlingRights,
    pub prev_halfmoveclock: u16,
    pub lmove: LoggedMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoggedMove {
    Castle(Side),
    Piece(LoggedPieceMove)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoggedPieceMove {
    pub mgmove: PieceMove,
    pub capture: Option<Piece>,
    pub is_pdj /* (pawn double jump) */: bool,
    pub target: StandardCoordinate
}

// # `Bitboards`

#[derive(Clone, PartialEq, Eq)]
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
        is_attacked(&self, locate_king_stdc(&self, self.active_player))
    }

    /// Constructs `Bitbaords` representing an completely empty 
    /// board where white is the active player.
    pub fn new() -> Self {
        Self {
            species_bbs: SpeciesTable::default(),
            affilia_bbs: ColorTable::default(),
            affilia_rel_bbs: ColorTable::default(),
            pawn_rel_bb: 0,
            active_player: Color::White
        }
    }   
}


/// Calculates the [`Coordinate`] of the given player's king.
pub fn locate_king<C: CoordinateSystem>(board: &Bitboards, color: Color) -> Coordinate<C> {
    let mut bb: Bitboard<C> = Bitboard::empty();
    bb  = board.species_bbs[Species::King].get();
    bb &= board.affilia_bbs[color].get();
    return bb.single();
}

/// Calculates the [`StandardCoordinate`] of the given player's king.
pub fn locate_king_stdc(board: &Bitboards, color: Color) -> StandardCoordinate {
    locate_king::<StandardCS>(board, color).into()
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

pub fn status(state: &mut ChessGame) -> GameStatus {
    let has_move = count_legal_moves(state) > 0;
    if has_move { return GameStatus::Incomplete; }
    if !state.bbs.is_check() {
        return GameStatus::Complete(GameResult::Tie)
    }
    let victor = state.active_player().oppo();
    return GameStatus::Complete(GameResult::Diff(victor));
}
