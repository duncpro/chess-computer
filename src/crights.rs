use std::fmt::{Debug, Display};

use crate::bitboard::Bitboard;
use crate::coordinates::StandardCS;
use crate::gamestate::locate_king_stdc;
use crate::{getbit, play};
use crate::grid::{File, Rank};
use crate::grid::StandardCoordinate;
use crate::grid::FileDirection;
use crate::gamestate::ChessGame;
use crate::piece::{Color, Piece};
use crate::piece::Species;

// # `CastlingRights`

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights { data: u8 }

impl CastlingRights {
    pub fn get(self, side: FileDirection, color: Color) -> bool {
        let index = 2 * color.index() + side.index();
        return getbit!(self.data, index);
    }

    pub fn set(&mut self, side: FileDirection, color: Color, value: bool)
    {
        let index = 2 * color.index() + side.index();
        self.data &= !(1 << index);
        self.data |= (1 << index) * (value as u8);
    }

    pub fn revoke(&mut self, color: Color) {
        self.set(FileDirection::Queenside, color, false);
        self.set(FileDirection::Kingside, color, false);
    }

    pub fn data(self) -> u8 { self.data }

    /// Value of `CastlingRights` at the beginning of a standard chess
    /// game before any moves have been made. This value corresponds
    /// to the state of both players having both their castling
    /// rights intact.
    pub const INITIAL: Self = Self { data: 0b1111 };

    pub const NONE: Self = Self { data: 0 };
}

// # Updating Castling Rights

pub fn update_crights_all(state: &mut ChessGame) {
    update_crights_spec(state, FileDirection::Queenside, Color::White);
    update_crights_spec(state, FileDirection::Kingside, Color::White);
    update_crights_spec(state, FileDirection::Queenside, Color::Black);
    update_crights_spec(state, FileDirection::Kingside, Color::Black);
}

fn update_crights_spec(state: &mut ChessGame, side: FileDirection, player: Color) {
    let mut value = state.crights.get(side, player);
    value &= is_king_intact(state, player);
    const ROOK_FILE_LUT: [File; 2] = [File::A, File::H];
    {
        let rook_home = StandardCoordinate::new(
            Rank::base_rank(player),
            ROOK_FILE_LUT[usize::from(side.index())]);
        let occupant = state.p_lut.get(rook_home);
        let is_rook_intact = (occupant == Some(Piece::new(player, Species::Rook)));
        value &= is_rook_intact;
    }
    state.crights.set(side, player, value);
}

fn is_king_intact(state: &mut ChessGame, player: Color) -> bool {
    let base_rank = Rank::base_rank(player);
    let king_home = StandardCoordinate::new(base_rank, File::E);
    let king_pos  = locate_king_stdc(&state.bbs, player);
    return king_home == king_pos;
}

impl Debug for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let qw = self.get(FileDirection::Queenside, Color::White);
        let kw = self.get(FileDirection::Kingside, Color::White);
        let qb = self.get(FileDirection::Queenside, Color::Black);
        let kb = self.get(FileDirection::Kingside, Color::Black);
        write!(f, "(kb: {}, qb: {}, kw: {}, qw: {})", kb, qb, kw, qw)
    }
}
