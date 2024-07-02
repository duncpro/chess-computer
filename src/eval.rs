use crate::cache::Cache;
use crate::cache::CacheEntry;
use crate::early_ok;
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
use crate::early_return;
use std::time::Instant;

pub const MAX_SCORE: i16 = i16::MAX - 1;
pub const MIN_SCORE: i16 = i16::MIN + 2;
pub const BELOW_MIN_SCORE: i16 = i16::MIN + 1;

// # Time Constrained Evaluation

pub struct DeepEvalContext<'a, 'b, 'c> {
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
    /// opportunity, and so it is not interesting to us.
    pub cutoff: i16,
    pub cache: &'c mut Cache
}

pub enum DeepEvalException { DeadlineElapsed, Cut }

/// Computes the best score the active-player is assured of, assuming perfect play 
/// by the opponent. When the deadline elapses, the search is cancelled and
/// `Err(DeadlineElapsed)` is returned.
pub fn deep_eval(mut ctx: DeepEvalContext) -> Result<i16, DeepEvalException> {
    use DeepEvalException::*;
    // Enforce time and depth constraints.
    if Instant::now() > ctx.deadline { return Err(DeadlineElapsed); }
    if ctx.lookahead == 0 { return Ok(shallow_eval(ctx.gstate)); }
    movegen_legal(ctx.gstate, &mut ctx.movebuf);
    early_ok! { leaf_eval(ctx.gstate, ctx.movebuf.is_empty()) };
    early_ok! { ctx.cache.lookup_score(ctx.gstate, ctx.lookahead) };
    
    let mut best_score: i16 = BELOW_MIN_SCORE;
    while let Some(mov) = ctx.movebuf.pop() {        
        make_move(ctx.gstate, mov);
        let result = deep_eval(DeepEvalContext { gstate: ctx.gstate, 
            lookahead: ctx.lookahead - 1, movebuf: ctx.movebuf.extend(), 
            deadline: ctx.deadline, cutoff: best_score, cache: ctx.cache });
        unmake_move(ctx.gstate);
        match result {
            Err(DeadlineElapsed) => return Err(DeadlineElapsed),
            Err(Cut) => {},
            Ok(score) => {
                // The larger `score` is the better this position is for C.
                // The larger `score * -1` is the better this position is for N.
                // Recall `ctx.cutoff` is the highest score that P is assured of.
                // Then `ctx.cutoff * -1` is the highest score that P will allow N to achieve. 
                // So if this position is better for N than the best score for N that the P will allow,
                // we cut it, as P will never give us an opportunity to make this move (they have
                // a better choice already).
                if score * -1 >= ctx.cutoff * -1 { return Err(Cut); }
                max_inplace(&mut best_score, -1 * score);
            },
        }
    }
    ctx.cache.update_score(ctx.gstate, best_score, ctx.lookahead);
    return Ok(best_score);
}

// # Shallow Evaluation

/// Evaluates the given position with no lookahead and no deadline. 
/// Unlike `deep_eval` which is a long-running procedure that must
/// complete before some deadline, this procedure `shallow_eval` is not
/// time-limitable and is therefore **guaranteed** to return an evaluation.
/// This is the quickest correct evaluation procedure. This program cannot
/// complete an evaluation any faster than the runtime of this procedure. 
/// If there is less time remaining on our clock than the amount of time
/// required to execute this procedure, the game is necessarily lost.
/// The short runtime of `shallow_eval` is at the expense of accuracy.
pub fn shallow_eval(gstate: &mut FastPosition) -> i16 {
    let cant_move = count_legal_moves(gstate) == 0;
    early_return! { leaf_eval(gstate, cant_move) };
    return calc_matdiff(&gstate.bbs);
}

fn leaf_eval(gstate: &mut FastPosition, cant_move: bool) -> Option<i16> {
    if cant_move {
        if gstate.bbs.is_check() {
            return Some(MIN_SCORE)
        } else {
            return Some(0);
        }
    }
    let by_repetition = count_repetitions(gstate) >= 3;
    let by_50moverule = gstate.halfmoveclock >= 100;
    if by_repetition | by_50moverule { return Some(0); }
    return None;
}


