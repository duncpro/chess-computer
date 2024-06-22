use crate::bitboard::Bitboard;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::TileSpecies;
use crate::movegen::knight::knight_attack;

impl GameState {
    /// Determines if the active-player is currently checked 
    /// by the opponent.
    pub fn is_check(&self) -> bool {
        let mut check: bool = false;
        check |= is_check_pawn(self);
        check |= is_check_rook(self);
        check |= is_check_knight(self);
        check |= is_check_bishop(self);
        check |= is_check_queen(self);
        return check;
    }
}


fn is_check_pawn(state: &GameState) -> bool {
    todo!()
}

fn is_check_rook(state: &GameState) -> bool {
    todo!()
}

fn is_check_knight(state: &GameState) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb =  state.species_bbs[TileSpecies::Knight].get();
    bb &= state.affilia_bbs[state.active_player.oppo()].get();
    bb &= knight_attack(state.king());
    return bb.is_not_empty();
    
}

fn is_check_bishop(state: &GameState) -> bool {
    todo!()
}

fn is_check_queen(state: &GameState) -> bool {
    todo!()
}

