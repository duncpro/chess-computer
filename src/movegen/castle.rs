use crate::attack::is_attacked;
use crate::coordinates::StandardCS;
use crate::grid::{FileDirection, Rank};
use crate::grid::StandardCoordinate;
use crate::grid::File;
use crate::gamestate::ChessGame;

pub fn movegen_castle_kingside(state: &ChessGame) -> bool{
    let mut can_castle = true;
        
    let base_rank = Rank::base_rank(state.active_player());
    {
        let king_destin = StandardCoordinate::new(base_rank, File::G);
        can_castle &= !is_attacked(&state.bbs, king_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin = StandardCoordinate::new(base_rank, File::F);
        can_castle &= !is_attacked(&state.bbs, rook_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }

    can_castle &= state.crights.get(FileDirection::Kingside, state.active_player());
    
    return can_castle; 
}

pub fn movegen_castle_queenside(state: &ChessGame) -> bool {
    let mut can_castle = true;
        
    let base_rank = Rank::base_rank(state.active_player());
    {
        let king_destin = StandardCoordinate::new(base_rank, File::C);
        can_castle &= !is_attacked(&state.bbs, king_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin = StandardCoordinate::new(base_rank, File::D);
        can_castle &= !is_attacked(&state.bbs, rook_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }
    {
        let knight_pos = StandardCoordinate::new(base_rank, File::B);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(knight_pos.into());
    }

    can_castle &= state.crights.get(FileDirection::Queenside, state.active_player());

    return can_castle;
}
