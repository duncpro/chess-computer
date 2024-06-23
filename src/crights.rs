use crate::bitboard::Bitboard;
use crate::coordinates::Coordinate;
use crate::coordinates::StandardCS;
use crate::gamestate::locate_king_stdc;
use crate::getbit;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::grid::FileDirection;
use crate::gamestate::GameState;
use crate::misc::PieceColor;
use crate::misc::PieceSpecies;

// # `CastlingRights`

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

    pub fn revoke(&mut self, color: PieceColor) {
        self.set(FileDirection::Queenside, color, false);
        self.set(FileDirection::Kingside, color, false);
    }
}

// # Updating Castling Rights

pub fn update_crights(state: &mut GameState) {
    update_crights_kingside(state);
    update_crights_queenside(state);
}

fn update_crights_kingside(state: &mut GameState) {
    let mut value = state.crights.get(FileDirection::Kingside, 
        state.active_player());

    value &= is_king_intact(state);

    let base_rank = state.active_player().base_rank();
    
    let rook_home = StandardCoordinate::new(base_rank, File::from_index(7));
        
    let rooks: Bitboard<StandardCS> = 
        state.bbs.class(state.active_player(), PieceSpecies::Rook);
    
    value &= rooks.includes(rook_home.into());

    state.crights.set(FileDirection::Kingside, 
        state.active_player(), value);
}

fn update_crights_queenside(state: &mut GameState) {
    let mut value = state.crights.get(FileDirection::Queenside, 
        state.active_player());

    value &= is_king_intact(state);

    let base_rank = state.active_player().base_rank();
    
    let rook_home = StandardCoordinate::new(base_rank, File::from_index(0));
    
    let rooks: Bitboard<StandardCS> = 
        state.bbs.class(state.active_player(), PieceSpecies::Rook);
    
    value &= rooks.includes(rook_home.into());

    state.crights.set(FileDirection::Queenside, 
        state.active_player(), value);
}

fn is_king_intact(state: &mut GameState) -> bool {
    let base_rank = state.active_player().base_rank();
    let king_home = StandardCoordinate::new(base_rank, File::from_index(4));
    let king_pos  = locate_king_stdc(&state.bbs);
    return king_home == king_pos;
}
