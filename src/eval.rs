use crate::attack::is_attacked;
use crate::gamestate::FastPosition;
use crate::grid::FileDirection;
use crate::makemove::make_pmove;
use crate::makemove::unmake_move;
use crate::makemove::swap_active;
use crate::makemove::make_castle;
use crate::mat_eval::matdiff;
use crate::misc::SegVec;
use crate::misc::max_inplace;
use crate::movegen::castle::movegen_castle_kingside;
use crate::movegen::castle::movegen_castle_queenside;
use crate::movegen::dispatch::count_legal_moves;
use crate::movegen::dispatch::movegen_legal_pmoves;
use crate::movegen::types::MGAnyMove;
use crate::movegen::types::PMGMove;
use crate::movegen_castle;
use std::time::Instant;

pub const MIN_SCORE: i32 = i32::MIN + 1;

// # Time Constrained Evaluation

pub struct DeepEvalWDeadlineContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    /// The number of complete plys to play-out before applying 
    /// the heuristic score function to the position. When zero
    /// the aforementioned function is applied immediately.
    pub lookahead: u8,
    pub movebuf: SegVec<'b, PMGMove>,
    pub deadline: Instant
}

pub struct DeadlineElapsed;

/// Computes the lowest score the active-player is assured of
/// given perfect play. When the deadline elapses, search is
/// cancelled and `Err(DeadlineElapsed)` is returned.
pub fn deep_eval_w_deadline(mut ctx: DeepEvalWDeadlineContext) 
-> Result<i32, DeadlineElapsed> 
{
    if Instant::now() > ctx.deadline { return Err(DeadlineElapsed); }
    movegen_legal_pmoves(ctx.gstate, &mut ctx.movebuf);
    // In the case there are no legal moves, it's a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if ctx.movebuf.is_empty() { return Ok(MIN_SCORE); }
    if ctx.lookahead == 0 { return Ok(matdiff(&ctx.gstate.bbs)); }
     
    let mut best_score: i32 = MIN_SCORE;

    fn eval_unmake(ctx: &mut DeepEvalWDeadlineContext, best_score: &mut i32)
    -> Result<(), DeadlineElapsed>
    {
        swap_active(ctx.gstate);
        let score = deep_eval_w_deadline(DeepEvalWDeadlineContext { 
            gstate: ctx.gstate, lookahead: ctx.lookahead - 1, 
            movebuf: ctx.movebuf.extend(), deadline: ctx.deadline });
        unmake_move(ctx.gstate);
        max_inplace(best_score, -1 * score?);
        return Ok(());
    }
    
    while let Some(pmove) = ctx.movebuf.pop() {        
        make_pmove(ctx.gstate, pmove);
        eval_unmake(&mut ctx, &mut best_score)?;
    }
    if movegen_castle_queenside(ctx.gstate) {
        make_castle(ctx.gstate, FileDirection::Queenside);
        eval_unmake(&mut ctx, &mut best_score)?;
    }
    if movegen_castle_kingside(ctx.gstate) {
        make_castle(ctx.gstate, FileDirection::Kingside);
        eval_unmake(&mut ctx, &mut best_score)?;
    }
    return Ok(best_score);
}

// # Shallow Evaluation

/// Evaluates the given position with no lookahead.
pub fn shallow_eval(gstate: &mut FastPosition) -> i32 {
    // In the case there are no legal moves, it's a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if count_legal_moves(gstate) == 0 { return MIN_SCORE };
    return matdiff(&gstate.bbs);
}
