use crate::bitboard::RawBitboard;
use crate::bitboard::Bitboard;
use crate::bits::bitscan;
use crate::bits::repeat_byte_u64;
use crate::coordinates::Coordinate;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::gamestate::LoggedMove;
use crate::gamestate::MovelogEntry;
use crate::gamestate::SpecialPieceMove;
use crate::grid::StandardCoordinate;
use crate::misc::SegVec;
use crate::piece::ColorTable;
use crate::piece::Color;
use crate::piece::Species;
use crate::movegen::moveset::MGPieceMove;
use crate::rmrel::absolutize;
use crate::rmrel::relativize;
use crate::setbit;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::BitAndAssign;

pub fn movegen_pawns(gstate: &GameState, moves: &mut SegVec<MGPieceMove>) {
    let mut ctx = PawnMGContext { gstate, moves };
    movegen_forward1(&mut ctx);
    movegen_forward2(&mut ctx);
    movegen_capture_queenside(&mut ctx);
    movegen_capture_kingside(&mut ctx);
    movegen_enpassant(&mut ctx);
}

fn movegen_forward1(ctx: &mut PawnMGContext) {
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player()];
    bb &= ctx.gstate.bbs.pawn_rel_bb;
    
    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !ctx.gstate.bbs.rel_occupancy();

    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin_rmrel in bitscan(bb & PROMOTE_MASK) {
        ctx.push_promote(destin_rmrel - 8, destin_rmrel);
    }
    for destin_rmrel in bitscan(bb & !PROMOTE_MASK) {
        ctx.push(destin_rmrel - 8, destin_rmrel);
    }
}

fn movegen_forward2(ctx: &mut PawnMGContext) {
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player()];
    bb &= ctx.gstate.bbs.pawn_rel_bb;

    // Filter out all pawns not on their home rank.
    const HOME_RANK: RawBitboard = 0b11111111 << 8;
    bb &= HOME_RANK;

    // Advance the pawns forward one square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !ctx.gstate.bbs.rel_occupancy();

    // Advance the pawns forward on square towards
    // the opponent.
    bb <<= 8;

    // Filter out all pawns intersecting an occupied square.
    bb &= !ctx.gstate.bbs.rel_occupancy();

    for destin_rmrel in bitscan(bb) {
        let origin = absolutize(destin_rmrel - 16, ctx.gstate.active_player());
        let destin = absolutize(destin_rmrel, ctx.gstate.active_player());
        ctx.moves.push(MGPieceMove {
            origin, destin, 
            target: destin,
            special: Some(SpecialPieceMove::PawnDoubleJump),
            promote: None,
        });
    }
}

fn movegen_capture_queenside(ctx: &mut PawnMGContext) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player()];
    bb &= ctx.gstate.bbs.pawn_rel_bb;

    // Exclude those pawns on the queenside-most rank,
    // they cannot move any further queenside.
    const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b00000001);
    bb &= BORDER_MASK;

    // Advance the pawns diagonally forward towards the queenside.
    bb <<= 7; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player().oppo()];
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        ctx.push_promote(destin - 7, destin);
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        ctx.push(destin - 7, destin);
    }
}

fn movegen_capture_kingside(ctx: &mut PawnMGContext) {    
    let mut bb: RawBitboard = 0;

    // Select all of the active-player's pawns.
    bb =  ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player()];
    bb &= ctx.gstate.bbs.pawn_rel_bb;

    // Exclude those pawns on the kingside-most rank,
    // they cannot move any further kingside.
    const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b10000000);
    bb &= BORDER_MASK;

    // Advance the pawns diagonally forward towards the kingside.
    bb <<= 9; 

    // Filter out all pawns which are not intersecting
    // an enemy piece.
    bb &= ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player().oppo()];
    
    const PROMOTE_MASK: RawBitboard = 0b11111111 << (8 * 7);

    for destin in bitscan(bb & PROMOTE_MASK) {
        ctx.push_promote(destin - 9, destin);
    }

    for destin in bitscan(bb & !PROMOTE_MASK) {
        ctx.push_promote(destin - 9, destin);
    }
}

fn movegen_enpassant(ctx: &mut PawnMGContext) {
    if let Some(last_entry) = ctx.gstate.movelog.last() {
        if let LoggedMove::Piece(pmove) = last_entry.lmove {
            if pmove.mgmove.special == Some(SpecialPieceMove::PawnDoubleJump) {
                let target_rmrel = relativize(pmove.mgmove.destin,
                    ctx.gstate.active_player());
                
                let destin_rmrel = target_rmrel + 8;

                let mut bb = reverse_pawn_attack(target_rmrel);
                bb &= ctx.gstate.bbs.affilia_rel_bbs[ctx.gstate.active_player()];
                bb &= ctx.gstate.bbs.pawn_rel_bb;

                let destin = absolutize(destin_rmrel, ctx.gstate.active_player());
                let target = StandardCoordinate::from(pmove.mgmove.destin);
                for origin_rmrel in bitscan(bb) {
                    let origin = absolutize(origin_rmrel, ctx.gstate.active_player());
                    ctx.moves.push(MGPieceMove { 
                        origin, destin, target, 
                        special: None,
                        promote: None
                    });
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
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b00000001);
        bb &= BORDER_MASK;
        bb >>= 1;
        attackers |= bb;
    }
    // Attack from kingside
    {
        let mut bb: RawBitboard = 0;
        setbit!(bb, target);
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b10000000);
        bb &= BORDER_MASK;
        bb <<= 1;
        attackers |= bb;
    }
    return attackers;
}

// ## `PawnMGContext`

struct PawnMGContext<'a, 'b> {
    gstate: &'a GameState,
    moves: &'a mut SegVec<'b, MGPieceMove>
}

impl<'a, 'b> PawnMGContext<'a, 'b> {
    fn push_promote(&mut self, origin_rmrel: u8, destin_rmrel: u8) {
        let origin = absolutize(origin_rmrel, self.gstate.active_player());
        let destin = absolutize(destin_rmrel, self.gstate.active_player());

        use Species::*;
        self.moves.push(make_promote_move(origin, destin, Queen));
        self.moves.push(make_promote_move(origin, destin, Rook));
        self.moves.push(make_promote_move(origin, destin, Bishop));
        self.moves.push(make_promote_move(origin, destin, Knight));
    }

    fn push(&mut self, origin_rmrel: u8, destin_rmrel: u8) {
        let origin = absolutize(origin_rmrel, self.gstate.active_player());
        let destin = absolutize(destin_rmrel, self.gstate.active_player());
        self.moves.push(MGPieceMove::normal(origin, destin));
    }
}

// # Move Constructors

fn make_promote_move(origin: StandardCoordinate, destin: StandardCoordinate, 
    desire: Species) -> MGPieceMove
{
    return MGPieceMove { 
        origin, 
        destin, 
        target: destin,
        special: Some(SpecialPieceMove::Promote),
        promote: Some(desire)
    };
}
