use crate::cache::CacheEntry;
use crate::gamestate::FastPosition;
use crate::makemove::make_move;
use crate::makemove::unmake_move;
use crate::mat_eval::calc_matdiff;
use crate::misc::SegVec;
use crate::misc::max_inplace;
use crate::movegen::dispatch::count_legal_moves;
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::MGAnyMove;
use crate::repetitions::count_repetitions;
use std::time::Instant;

pub const MAX_SCORE: i16 = i16::MAX - 1;
pub const MIN_SCORE: i16 = i16::MIN + 2;
pub const BELOW_MIN_SCORE: i16 = i16::MIN + 1;

// # Time Constrained Evaluation

pub struct DeepEvalContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    /// The number of complete plys to play-out before applying 
    /// the heuristic score function to the position. When zero,
    /// the heuristic score function is applied immediately.
    pub lookahead: u8,
    /// The buffer used to hold lookahead moves after they
    /// are generated but before they are evaluated. This
    /// buffer should be empty when `DeepEvalContext` is
    /// constructed by the caller.
    pub movebuf: SegVec<'b, MGAnyMove>,
    pub deadline: Instant,
    /// The best score that the parent is assured of so-far.
    /// If a child/opponent move is encountered with a score 
    /// better than `cutoff`, this branch is pruned (not explored),
    /// as the opponent will surely take this branch given the
    ///  opportunity, and so it is not interesting to us.
    pub cutoff: i16
}

pub enum DeepEvalException {
    DeadlineElapsed,
    Cut
}


/// Computes the highest score the active-player is assured of
/// given perfect play by the opponent. If the deadline elapses, the 
/// search is cancelled and `Err(DeadlineElapsed)` is returned.
pub fn deep_eval(mut ctx: DeepEvalContext) -> Result<i16, DeepEvalException> {
    use DeepEvalException::*;
    
    if Instant::now() > ctx.deadline { return Err(DeadlineElapsed); }
    if ctx.lookahead == 0 { return Ok(shallow_eval(ctx.gstate)); }
    if is_drawable(ctx.gstate) { return Ok(0); }

    let mut best_score: i16 = BELOW_MIN_SCORE;

    fn eval_unmake(ctx: &mut DeepEvalContext, best_score: &mut i16)
    -> Result<(), DeepEvalException>
    {
        let result = deep_eval(DeepEvalContext { gstate: ctx.gstate, 
            lookahead: ctx.lookahead - 1, movebuf: ctx.movebuf.extend(), 
            deadline: ctx.deadline, cutoff: *best_score });
        unmake_move(ctx.gstate);
        match result {
            Err(DeadlineElapsed) => Err(DeadlineElapsed),
            Err(Cut) => Ok(()),
            Ok(score) => {
                if score * -1 >= ctx.cutoff * -1 { return Err(Cut); }
                max_inplace(best_score, -1 * score);
                Ok(())
            },
        }
    }
    movegen_legal(ctx.gstate, &mut ctx.movebuf);
    // In the case there are no legal moves, it's a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if ctx.movebuf.is_empty() { return Ok(MIN_SCORE); }
    while let Some(mov) = ctx.movebuf.pop() {        
        make_move(ctx.gstate, mov);
        eval_unmake(&mut ctx, &mut best_score)?;
    }
    return Ok(best_score);
}

// # Shallow Evaluation

/// Evaluates the given position with no lookahead.
pub fn shallow_eval(gstate: &mut FastPosition) -> i16 {
    // In the case there are no legal moves, it's a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if count_legal_moves(gstate) == 0 { return MIN_SCORE };    
    let matdiff = calc_matdiff(&gstate.bbs);
    if is_drawable(gstate) { return 0; }
    return matdiff;
}

fn is_drawable(gstate: &mut FastPosition) -> bool {
    let by_repetition = count_repetitions(gstate) >= 3;
    let by_50moverule = gstate.halfmoveclock >= 100;
    return by_repetition | by_50moverule;
}
