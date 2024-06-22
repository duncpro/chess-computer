use crate::check::is_check;
use crate::coordinates::StandardCS;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::grid::File;
use crate::gamestate::GameState;

pub fn movegen_castle(state: &GameState) {
    movegen_castle_ks(state);
    movegen_castle_qs(state);
}

pub fn movegen_castle_ks(state: &GameState) {    
    let mut can_castle = false;
        
    let base_rank = state.active_player.base_rank();
    {
        let king_destin_file = File::from_index(6);
        let king_destin = StandardCoordinate::new(base_rank, king_destin_file);
        can_castle &= !is_check(&state.mdboard, state.active_player, king_destin);
        can_castle &= !state.mdboard.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin_file = File::from_index(5);
        let rook_destin = StandardCoordinate::new(base_rank, rook_destin_file);   
        can_castle &= !is_check(&state.mdboard, state.active_player, rook_destin);
        can_castle &= !state.mdboard.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }

    can_castle &= state.crights.get(FileDirection::Kingside, state.active_player);
    
    todo!("add to move queue")    
}

pub fn movegen_castle_qs(state: &GameState) {
    let mut can_castle = false;
        
    let base_rank = state.active_player.base_rank();
    {    
        let king_destin_file = File::from_index(2);
        let king_destin = StandardCoordinate::new(base_rank, king_destin_file);
        can_castle &= !is_check(&state.mdboard, state.active_player, king_destin);
        can_castle &= !state.mdboard.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin_file = File::from_index(3);
        let rook_destin = StandardCoordinate::new(base_rank, rook_destin_file);
        can_castle &= !is_check(&state.mdboard, state.active_player, rook_destin);
        can_castle &= !state.mdboard.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }
    {
        let knight_file = File::from_index(1);
        let knight_pos = StandardCoordinate::new(base_rank, knight_file);
        can_castle &= !state.mdboard.occupancy::<StandardCS>()
            .includes(knight_pos.into());
    }

    can_castle &= state.crights.get(FileDirection::Queenside, state.active_player);

    todo!("add to move queue")
}
