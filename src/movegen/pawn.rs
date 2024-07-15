use crate::bitboard::RawBitboard;
use crate::bits::bitscan;
use crate::bits::repeat_byte_u64;
use crate::gamestate::LoggedMove;
use crate::misc::Push;
use crate::piece::Species;
use crate::rmrel::{absolutize, RMRelCoord};
use crate::rmrel::relativize;
use crate::setbit;
use crate::movegen::types::{GeneratedMove, MGContext};
use crate::mov::PieceMove;

pub fn movegen_pawns(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    movegen_forward1(ctx);
    movegen_forward2(ctx);
    movegen_capture_queenside(ctx);
    movegen_capture_kingside(ctx);
    movegen_enpassant(ctx);
}

fn movegen_forward1(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
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
        push_basic(ctx, destin_rmrel - 8, destin_rmrel);
    }
}

fn movegen_forward2(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
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
        push_basic(ctx, /* origin = */ destin_rmrel - 16, /* destin = */ destin_rmrel);
    }
}

fn movegen_capture_queenside(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
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
        push_basic(ctx, destin - 7, destin);
    }
}

fn movegen_capture_kingside(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
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
        push_basic(ctx, destin - 9, destin);
    }
}

fn movegen_enpassant(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    if let Some(last_entry) = ctx.inspect(|s| s.movelog.last().copied()) {
        if let LoggedMove::Piece(pmove) = last_entry.lmove {
            if pmove.is_pdj {
                let target_rmrel = relativize(pmove.mgmove.destin,
                    ctx.active_player());
                
                let destin_rmrel = target_rmrel + 8;

                let mut bb = reverse_pawn_attack(destin_rmrel);
                bb &= ctx.inspect(|s| s.bbs.affilia_rel_bbs[s.active_player()]);
                bb &= ctx.inspect(|s| s.bbs.pawn_rel_bb);

                for origin_rmrel in bitscan(bb) {
                    push_basic(ctx, origin_rmrel, destin_rmrel);
                }
            }
        }
    }
}

fn push_promote(ctx: &mut MGContext<impl Push<GeneratedMove>>,
                origin_rmrel: u8, destin_rmrel: u8)
{
    push(ctx, origin_rmrel, destin_rmrel, Some(Species::Knight));
    push(ctx, origin_rmrel, destin_rmrel, Some(Species::Bishop));
    push(ctx, origin_rmrel, destin_rmrel, Some(Species::Queen));
    push(ctx, origin_rmrel, destin_rmrel, Some(Species::Rook));
}

fn push_basic(ctx: &mut MGContext<impl Push<GeneratedMove>>,
              origin_rmrel: u8, destin_rmrel: u8)
{
    push(ctx, origin_rmrel, destin_rmrel, None);
}

fn push(ctx: &mut MGContext<impl Push<GeneratedMove>>,
        origin_rmrel: u8, destin_rmrel: u8, promote: Option<Species>)
{
    let origin = absolutize(origin_rmrel, ctx.active_player());
    let destin = absolutize(destin_rmrel, ctx.active_player());
    ctx.push_p(PieceMove { origin, destin, promote });
}

/// Create a [`RawBitboard`] denoting all squares which are attacked by an
/// opponent-pawn placed at `origin`. Put another way, the pawn attacks *towards*
/// the *least* significant byte.
pub fn reverse_pawn_attack(target: RMRelCoord) -> RawBitboard {
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

/// Create a [`RawBitboard`] denoting all squares which are attacked by a
/// self-pawn placed at `origin`. Put another way, the pawn attacks *towards*
/// the *most* significant byte.
pub fn pawn_attack(origin: RMRelCoord) -> RawBitboard {
    let mut targets: RawBitboard = 0;
    {
        let mut bb: RawBitboard = 0;
        setbit!(bb, origin);
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b00000001);
        bb &= BORDER_MASK;
        bb <<= 7;
        targets |= bb;
    }
    {
        let mut bb: RawBitboard = 0;
        setbit!(bb, origin);
        const BORDER_MASK: RawBitboard = !repeat_byte_u64(0b10000000);
        bb &= BORDER_MASK;
        bb <<= 9;
        targets |= bb;
    }
    return targets;
}
