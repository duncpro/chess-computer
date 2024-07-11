use crate::bitboard::RawBitboard;
use crate::bitboard::Bitboard;
use crate::bits::bitscan;
use crate::bits::repeat_byte_u64;
use crate::coordinates::Coordinate;
use crate::coordinates::RankMajorCS;
use crate::gamestate::ChessGame;
use crate::gamestate::LoggedMove;
use crate::gamestate::MovelogEntry;
use crate::grid::StandardCoordinate;
use crate::misc::Push;
use crate::misc::SegVec;
use crate::piece::ColorTable;
use crate::piece::Color;
use crate::piece::Species;
use crate::movegen::types::PMGMove;
use crate::rmrel::absolutize;
use crate::rmrel::relativize;
use crate::setbit;
use crate::movegen::types::PMGContext;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::BitAndAssign;

pub fn movegen_pawns(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    movegen_forward1(ctx);
    movegen_forward2(ctx);
    movegen_capture_queenside(ctx);
    movegen_capture_kingside(ctx);
    movegen_enpassant(ctx);
}

fn movegen_forward1(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    let mut bb: RawBitboard = 0;

    // Select the active-player's pawns.
    bb =  ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
    bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);
    
    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out pawns intersecting occupied squares.
    bb &= !ctx.inspect(|s| s.bbs.rel_occupancy());

    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin_rmrel in bitscan(bb & PROMOTE_MASK) {
        push_promote(ctx, destin_rmrel - 8, destin_rmrel);
    }
    for destin_rmrel in bitscan(bb & !PROMOTE_MASK) {
        push(ctx, destin_rmrel - 8, destin_rmrel);
    }
}

fn movegen_forward2(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    let mut bb: RawBitboard = 0;

    // Select the active-player's pawns.
    bb =  ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
    bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);

    // Filter out all pawns not on their home rank.
    const HOME_RANK: RawBitboard = 0b11111111 << 8;
    bb &= HOME_RANK;

    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !ctx.inspect(|s| s.bbs.rel_occupancy());

    // Advance the pawns forward on square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !ctx.inspect(|s| s.bbs.rel_occupancy());

    for destin_rmrel in bitscan(bb) {
        let origin = absolutize(destin_rmrel - 16, ctx.active_player());
        let destin = absolutize(destin_rmrel, ctx.active_player());
        ctx.push(PMGMove::new_basic(origin, destin));
    }
}

fn movegen_capture_queenside(ctx: &mut PMGContext<impl Push<PMGMove>>) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
    bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);

    // Exclude those pawns on the queenside-most rank,
    // they cannot move any further queenside.
    const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b00000001);
    bb &= BORDER_MASK;

    // Advance the pawns diagonally forward towards the queenside.
    bb <<= 7; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player().oppo()]);
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        push_promote(ctx, destin - 7, destin);
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        push(ctx, destin - 7, destin);
    }
}

fn movegen_capture_kingside(ctx: &mut PMGContext<impl Push<PMGMove>>) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
    bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);

    // Exclude those pawns on the kingside-most rank,
    // they cannot move any further kingside.
    const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b10000000);
    bb &= BORDER_MASK;

    // Advance the pawns diagonally forward towards the kingside.
    bb <<= 9; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player().oppo()]);
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        push_promote(ctx, destin - 9, destin);
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        push(ctx, destin - 9, destin);
    }
}

fn movegen_enpassant(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    if let Some(last_entry) = ctx.inspect(|s| s.movelog.last().copied()) {
        if let LoggedMove::Piece(pmove) = last_entry.lmove {
            if pmove.is_pdj {
                let target_rmrel = relativize(pmove.mgmove.destin,
                    ctx.active_player());
                
                let destin_rmrel = target_rmrel + 8;

                let mut bb = reverse_pawn_attack(destin_rmrel);
                bb &= ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
                bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);

                let destin = absolutize(destin_rmrel, ctx.active_player());
                for origin_rmrel in bitscan(bb) {
                    let origin = absolutize(origin_rmrel, ctx.active_player());
                    ctx.push(PMGMove { origin, destin, promote: None });
                }
            }
        }
    }
}

pub fn reverse_pawn_attack(target: u8) -> RawBitboard {
    let mut attackers: RawBitboard = 0;
    // Attack from queenside
    {
        let mut bb: RawBitboard = 0;
        setbit!(bb, target);
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b10000000);
        bb &= BORDER_MASK;
        bb >>= 7;
        attackers |= bb;
    }
    {
        let mut bb: RawBitboard = 0;
        setbit!(bb, target);
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b00000001);
        bb &= BORDER_MASK;
        bb >>= 9;
        attackers |= bb;
    }
    return attackers;
}


fn push_promote(ctx: &mut PMGContext<impl Push<PMGMove>>, 
    origin_rmrel: u8, destin_rmrel: u8) 
{
    let origin = absolutize(origin_rmrel, ctx.active_player());
    let destin = absolutize(destin_rmrel, ctx.active_player());

    use Species::*;
    ctx.push(PMGMove::new_promote(origin, destin, Queen));
    ctx.push(PMGMove::new_promote(origin, destin, Rook));
    ctx.push(PMGMove::new_promote(origin, destin, Bishop));
    ctx.push(PMGMove::new_promote(origin, destin, Knight));
}

fn push(ctx: &mut PMGContext<impl Push<PMGMove>>, 
    origin_rmrel: u8, destin_rmrel: u8) 
{
    let origin = absolutize(origin_rmrel, ctx.active_player());
    let destin = absolutize(destin_rmrel, ctx.active_player());
    ctx.push(PMGMove::new_basic(origin, destin));
}
