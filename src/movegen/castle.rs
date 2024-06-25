use crate::check::is_check;
use crate::coordinates::StandardCS;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::grid::File;
use crate::gamestate::GameState;

#[macro_export]
macro_rules! movegen_castle {
    ($side:ident /* either Kingside or Queenside */, $state:expr) => {{
        ::paste::paste! {
            use crate::movegen::castle::movegen_castle_kingside;
            use crate::movegen::castle::movegen_castle_queenside;
            [< movegen_castle _ $side:lower >]($state)
        }}
    };
}

pub fn movegen_castle_kingside(state: &GameState) -> bool{    
    let mut can_castle = false;
        
    let base_rank = state.active_player().base_rank();
    {
        let king_destin_file = File::from_index(6);
        let king_destin = StandardCoordinate::new(base_rank, king_destin_file);
        can_castle &= !is_check(&state.bbs, king_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin_file = File::from_index(5);
        let rook_destin = StandardCoordinate::new(base_rank, rook_destin_file);   
        can_castle &= !is_check(&state.bbs, rook_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }

    can_castle &= state.crights.get(FileDirection::Kingside, state.active_player());
    
    return can_castle; 
}

pub fn movegen_castle_queenside(state: &GameState) -> bool {
    let mut can_castle = false;
        
    let base_rank = state.active_player().base_rank();
    {    
        let king_destin_file = File::from_index(2);
        let king_destin = StandardCoordinate::new(base_rank, king_destin_file);
        can_castle &= !is_check(&state.bbs, king_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(king_destin.into());
    }
    {
        let rook_destin_file = File::from_index(3);
        let rook_destin = StandardCoordinate::new(base_rank, rook_destin_file);
        can_castle &= !is_check(&state.bbs, rook_destin);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(rook_destin.into());
    }
    {
        let knight_file = File::from_index(1);
        let knight_pos = StandardCoordinate::new(base_rank, knight_file);
        can_castle &= !state.bbs.occupancy::<StandardCS>()
            .includes(knight_pos.into());
    }

    can_castle &= state.crights.get(FileDirection::Queenside, state.active_player());

    return can_castle;
}
