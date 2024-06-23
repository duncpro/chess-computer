use crate::bitboard::RawBitboard;
use crate::bitboard::Bitboard;
use crate::bits::bitscan;
use crate::bits::repeat_byte_u64;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::gamestate::LoggedMove;
use crate::gamestate::MovelogEntry;
use crate::gamestate::PieceMoveKind;
use crate::misc::ColorTable;
use crate::misc::OptionPieceSpecies;
use crate::misc::PieceColor;
use crate::rmrel::convert_rmrel_coord;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::BitAndAssign;

pub fn movegen_pawns(state: &GameState) {
    movegen_forward1(state);
    movegen_forward2(state);
    movegen_capture_queenside(state);
    movegen_capture_kingside(state);
    movegen_enpassant(state);
}

fn movegen_forward1(state: &GameState) {
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  state.affilia_rel_bbs[state.active_player];
    bb &= state.pawn_rel_bb;
    
    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !state.rel_occupancy();

    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        todo!("add to move queue")
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        todo!("add to move queue")
    }
}

fn movegen_forward2(state: &GameState) {
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  state.affilia_rel_bbs[state.active_player];
    bb &= state.pawn_rel_bb;

    // Filter out all pawns not on their home rank.
    const HOME_RANK: RawBitboard = 0b11111111 << 8;
    bb &= HOME_RANK;

    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !state.rel_occupancy();

    // Advance the pawns forward on square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !state.rel_occupancy();

    for destin in bitscan(bb) {
        todo!("add to move queue")
    }
}

fn movegen_capture_queenside(state: &GameState) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  state.affilia_rel_bbs[state.active_player];
    bb &= state.pawn_rel_bb;

    // Exclude those pawns on the queenside-most rank,
    // they cannot move any further queenside.
    const BORDER_MASK: RawBitboard = repeat_byte_u64(0b00000001);
    bb &= !BORDER_MASK;

    // Advance the pawns diagonally forward towards the queenside.
    bb <<= 7; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= state.affilia_rel_bbs[state.active_player.oppo()];
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        todo!("add to move queue")
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        todo!("add to move queue")
    }
}

fn movegen_capture_kingside(state: &GameState) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  state.affilia_rel_bbs[state.active_player];
    bb &= state.pawn_rel_bb;

    // Exclude those pawns on the kingside-most rank,
    // they cannot move any further kingside.
    const BORDER_MASK: RawBitboard = repeat_byte_u64(0b10000000);
    bb &= !BORDER_MASK;

    // Advance the pawns diagonally forward towards the kingside.
    bb <<= 9; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= state.affilia_rel_bbs[state.active_player.oppo()];
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        todo!("add to move queue")
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        todo!("add to move queue")
    }
}

fn movegen_enpassant(state: &GameState) {
    if let Some(last_entry) = state.movelog.last() {
        if let LoggedMove::Piece(pmove) = last_entry.lmove {
            if pmove.kind == PieceMoveKind::PawnDoubleJump {
                let target_rmrel = convert_rmrel_coord(
                    pmove.destin.index(), state.active_player);
                let destin_rmrel = target_rmrel + 8;
                todo!()
            }
        }
    }
}
