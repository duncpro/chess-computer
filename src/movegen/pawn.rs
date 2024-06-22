use crate::bitboard::RawBitboard;
use crate::bitboard::Bitboard;
use crate::bits::bitscan;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::misc::ColorTable;
use crate::misc::OptionPieceSpecies;
use crate::misc::PieceColor;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::BitAndAssign;

pub fn movegen_pawns(state: &GameState) {
}

fn movegen_forward2(state: &GameState) {
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  state.affilia_rel_bbs[state.active_player];
    bb &= state.pawn_rel_bb;

    // Filter out all pawns not on their home rank.
    const HOME_RANK: RawBitboard = 0b11111111 << 8;
    bb &= HOME_RANK;

    for destin in bitscan(bb) {
        todo!("add to move queue")
    }
}

fn movegen_forward1(state: &GameState) {
    let mut bb: RawBitboard = 0;
    
}
