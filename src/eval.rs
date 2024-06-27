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
use crate::movegen::types::PMGMove;
use std::time::Instant;

pub const MIN_SCORE: i32 = i32::MIN + 1;

// # Time Constrained Evaluation

pub struct DeepEvalContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    /// The number of complete plys to play-out before applying 
    /// the heuristic score function to the position. When zero,
    /// the heuristic score function is applied immediately.
    pub lookahead: u8,
    pub movebuf: SegVec<'b, PMGMove>,
    pub deadline: Instant,
    /// The best score that the parent is assured of.
    /// If a move is encountered which a score less than `cutoff`,
    /// this branch is pruned (not explored), as the opponent
    /// will surely take this branch, and so it is not interesting.
    pub cutoff: i32
}

pub enum DeepEvalException {
    DeadlineElapsed,
    Cut
}

/// Computes the lowest score the active-player is assured of
/// given perfect play. When the deadline elapses, search is
/// cancelled and `Err(DeadlineElapsed)` is returned.
pub fn deep_eval(mut ctx: DeepEvalContext) -> Result<i32, DeepEvalException> {
    use DeepEvalException::*;
    if Instant::now() > ctx.deadline { return Err(DeadlineElapsed); }
    movegen_legal_pmoves(ctx.gstate, &mut ctx.movebuf);
    // In the case there are no legal moves, it's a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if ctx.movebuf.is_empty() { return Ok(MIN_SCORE); }
    if ctx.lookahead == 0 { return Ok(matdiff(&ctx.gstate.bbs)); }
     
    let mut best_score: i32 = MIN_SCORE;

    fn eval_unmake(ctx: &mut DeepEvalContext, best_score: &mut i32)
    -> Result<(), DeepEvalException>
    {
        swap_active(ctx.gstate);
        let result = deep_eval(DeepEvalContext { gstate: ctx.gstate, 
            lookahead: ctx.lookahead - 1, movebuf: ctx.movebuf.extend(), 
            deadline: ctx.deadline, cutoff: *best_score });
        unmake_move(ctx.gstate);
        match result {
            Err(DeadlineElapsed) => Err(DeadlineElapsed),
            Err(DeepEvalException::Cut) => Ok(()),
            Ok(score) => {
                if score * -1 > ctx.cutoff * -1 { return Err(Cut); }
                max_inplace(best_score, -1 * score);
                Ok(())
            },
        }
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
