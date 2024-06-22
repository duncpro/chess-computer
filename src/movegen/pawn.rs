use crate::bitboard::RawBitboard;
use crate::bitboard::Bitboard;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::ColorTable;
use crate::misc::TileSpecies;
use crate::misc::PieceColor;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::BitAndAssign;

fn movegen_pawn_forward2(gs: &GameState) {
    let mut bb = PerspectiveBitboard::empty(gs.active_player);

    // Select all pawns affiliated with the active player.
    bb.abs_bb = gs.species_bbs[TileSpecies::Pawn].get();
    bb.abs_bb &= gs.affilia_bbs[gs.active_player].get();

    // Filter out pawns not intersecting their home rank.
    const PAWN_HOME_MASK: PerspectiveBitboardMask = 
        PerspectiveBitboardMask::new(0b11111111 << 8);
    bb &= PAWN_HOME_MASK;

    // Advance the first rank towards the opponent.    
    bb.shift_left(8);

    // Filter out pawns which landed on a preoccupied square.
    bb.abs_bb &= !gs.occupancy();
    
    // Advance the final rank towards the opponent.
    bb.shift_left(8);

    // Filter out pawns which landed on a preoccupied square.
    bb.abs_bb &= !gs.occupancy();

    for destination in bb.abs_bb.scan() {
        todo!("add to move list")
    }
}

const PROMOTE_MASK: PerspectiveBitboardMask
    = PerspectiveBitboardMask::new(0b11111111 << 7 * 8);

fn movegen_pawn_forward1(gs: &GameState) {
    let mut bb = PerspectiveBitboard::empty(gs.active_player);
    
    // Select all pawns affiliated with the active player.
    bb.abs_bb = gs.species_bbs[TileSpecies::Pawn].get();
    bb.abs_bb &= gs.affilia_bbs[gs.active_player].get();

    // Advance one rank towards the opponent.
    bb.shift_left(8);

    // Filter out pawns which landed on a preoccupied square.
    bb.abs_bb &= !gs.occupancy();

    // Push promotions into move queue.
    let promotions = (bb & PROMOTE_MASK);
    for destination in promotions.abs_bb.scan() {
        todo!("add to move list")
    }

    // Push nonpromotions into move queue.
    let nonpromotions = (bb & !PROMOTE_MASK);
    for destination in nonpromotions.abs_bb.scan() {
        todo!("add to move list")
    }
}

// # Utilities

// ## `PerspectiveBitboard`

/// An apparently relatively-oriented rank-major bitboard.
/// The least significant byte corresponds to the active-player's base rank, 
/// while the most significant byte corresponds to the opponent's base rank. 
///
/// Note that `PerspectiveBitboard` is simply a view atop an absolutely-oriented
/// rank-major bitboard. In other words, the bytes in the memory 
/// representation are not truly swapped, they just behave that way
/// when accessed through `PerspectiveBitboard`.
///
/// See also [`PerspectiveBitboardMask`], a mechanism for defining bitmasks
/// that use the rank-major relative coordinate system, and are therefore
/// compatible with `PerspectiveBitboard`.
///
/// This is a specialized bitboard. It is meant to make the pawn movegen
/// code less cumbersome, by allowing the move generation routines
/// to be written from a single perspective but in actuality work properly
/// for both black and white pieces.
#[derive(Clone, Copy)]
struct PerspectiveBitboard {
    abs_bb: Bitboard<RankMajorCS>,
    active_player: PieceColor
}

impl PerspectiveBitboard {
    /// Shift in the opponent's direction. For instance, `shift_left(8)`
    /// would discard the opponents base rank and move the active players
    /// base rank up by 1 whole rank.
    fn shift_left(&mut self, offset: u8) {
        self.abs_bb <<= (self.active_player.oppo().index() * offset);  
        self.abs_bb >>= (self.active_player.index() * offset);
     } 

    /// Shift in the active-players's direction. For instance, `shift_right(8)`
    /// would discard the active-player's base rank and move the opponent's
    /// base rank up by 1 whole rank.
    fn shift_right(&mut self, offset: u8) {
        self.abs_bb >>= (self.active_player.oppo().index() * offset);
        self.abs_bb <<= (self.active_player.index() * offset);
    }

    // Constructors
    
    fn empty(active_player: PieceColor) -> Self {
        Self { 
            abs_bb: Bitboard::empty(), 
            active_player  
        }
    }
}

impl BitAndAssign<PerspectiveBitboardMask> for PerspectiveBitboard {
    fn bitand_assign(&mut self, rhs: PerspectiveBitboardMask) {
        self.abs_bb &= rhs.table[self.active_player];
    }
}

impl BitAnd<PerspectiveBitboardMask> for PerspectiveBitboard {
    type Output = Self;
    fn bitand(self, rhs: PerspectiveBitboardMask) -> Self::Output {
        let mut bb = PerspectiveBitboard::empty(self.active_player);
        bb.abs_bb = self.abs_bb;
        bb.abs_bb &= rhs.table[self.active_player];
        return bb;
    }
}


// ## `PerspectiveBitboardMask`

struct PerspectiveBitboardMask { 
    table: ColorTable<Bitboard<RankMajorCS>>
}

impl PerspectiveBitboardMask {
    const fn new(bb: RawBitboard) -> Self {
        Self { 
            table: ColorTable::new([
                /* white */ Bitboard::from_raw(bb),
                /* black */ Bitboard::from_raw(bb.swap_bytes())
            ]) 
        }
    }
}

impl Not for PerspectiveBitboardMask {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.table[PieceColor::White].invert();
        self.table[PieceColor::Black].invert();
        return self;
    }
}

