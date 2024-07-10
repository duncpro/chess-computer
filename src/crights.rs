use std::fmt::Display;

use crate::bitboard::Bitboard;
use crate::coordinates::StandardCS;
use crate::gamestate::locate_king_stdc;
use crate::getbit;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::grid::FileDirection;
use crate::gamestate::FastPosition;
use crate::piece::Color;
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
    /// game, before any moves have been made. This value corresponds
    /// to the state of both players having both their castling
    /// rights.
    pub const INITIAL: Self = Self { data: 0b1111 };

    pub const NONE: Self = Self { data: 0 };
}

// # Updating Castling Rights

pub fn update_crights(state: &mut FastPosition) {
    update_crights_kingside(state);
    update_crights_queenside(state);
}

fn update_crights_kingside(state: &mut FastPosition) {
    let mut value = state.crights.get(FileDirection::Kingside, 
        state.active_player());

    value &= is_king_intact(state);

    let base_rank = state.active_player().base_rank();
    
    let rook_home = StandardCoordinate::new(base_rank, File::from_index(7));
        
    let rooks: Bitboard<StandardCS> = 
        state.bbs.class(state.active_player(), Species::Rook);
    
    value &= rooks.includes(rook_home.into());

    state.crights.set(FileDirection::Kingside, 
        state.active_player(), value);
}

fn update_crights_queenside(state: &mut FastPosition) {
    let mut value = state.crights.get(FileDirection::Queenside, 
        state.active_player());

    value &= is_king_intact(state);

    let base_rank = state.active_player().base_rank();
    
    let rook_home = StandardCoordinate::new(base_rank, File::from_index(0));
    
    let rooks: Bitboard<StandardCS> = 
        state.bbs.class(state.active_player(), Species::Rook);
    
    value &= rooks.includes(rook_home.into());

    state.crights.set(FileDirection::Queenside, 
        state.active_player(), value);
}

fn is_king_intact(state: &mut FastPosition) -> bool {
    let base_rank = state.active_player().base_rank();
    let king_home = StandardCoordinate::new(base_rank, File::from_index(4));
    let king_pos  = locate_king_stdc(&state.bbs);
    return king_home == king_pos;
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let qw = self.get(FileDirection::Queenside, Color::White);
        let kw = self.get(FileDirection::Kingside, Color::White);
        let qb = self.get(FileDirection::Queenside, Color::Black);
        let kb = self.get(FileDirection::Kingside, Color::Black);

        write!(f, "{:?}", (kb, qb, kw, qw))
    }
}
