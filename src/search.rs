use crate::eval::BELOW_MIN_SCORE;
use crate::eval::DeepEvalContext;
use crate::eval::DeepEvalException;
use crate::eval::MIN_SCORE;
use crate::eval::deep_eval;
use crate::eval::shallow_eval;
use crate::makemove::make_move;
use crate::misc::Max;
use crate::makemove::unmake_move;
use crate::misc::SegVec;
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::MGAnyMove;
use crate::gamestate::FastPosition;
use std::time::Instant;

// # Search

struct SearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    /// The `lookahead` (as in [`DeepEvilWDeadlineContext`]) used when
    /// evaluating each position resultant from each legal move
    /// the active-player has to choose from.
    pub eval_lookahead: u8,
    pub movebuf: SegVec<'b, MGAnyMove>,
    pub deadline: Instant
}

pub struct DeadlineElapsed;

/// Conducts a time-limited depth-first search for the optimal/
/// approximately optimal move. 
///
/// This procedure assumes that the game **is not** concluded,
/// and so there **must be** an optimal move. If this procedure
/// is called while the game is completed (there are no legal moves)
/// it will [`panic`]. When the deadline elapses, search is cancelled and
/// `Err(DeadlineElapsed)` is returned.
fn search(mut ctx: SearchContext) -> Result<MGAnyMove, DeadlineElapsed> {
    let mut best: Max<MGAnyMove, i16> = Max::new(BELOW_MIN_SCORE);

    fn eval_unmake(ctx: &mut SearchContext, best: &mut Max<MGAnyMove, i16>, 
        mov: MGAnyMove) -> Result<(), DeadlineElapsed> 
    {
        let result = deep_eval(DeepEvalContext { gstate: ctx.gstate, 
            lookahead: ctx.eval_lookahead, movebuf: ctx.movebuf.extend(), 
            deadline: ctx.deadline, cutoff: best.value() });
        unmake_move(ctx.gstate);
        match result {
            Err(DeepEvalException::DeadlineElapsed) => Err(DeadlineElapsed),
            Err(DeepEvalException::Cut) => Ok(()),
            Ok(score) => { best.push(mov, score * -1); Ok(()) }
        }
    }
       
    movegen_legal(ctx.gstate, &mut ctx.movebuf); 
    while let Some(mov) = ctx.movebuf.pop() {        
        make_move(ctx.gstate, mov);
        eval_unmake(&mut ctx, &mut best, mov)?;
    }
    return Ok(best.take().unwrap());
}


fn search_shallow(gstate: &mut FastPosition, mut movebuf: SegVec<MGAnyMove>) -> MGAnyMove {
    let mut best: Max<MGAnyMove, i16> = Max::new(MIN_SCORE);

    fn eval_unmake(gstate: &mut FastPosition) -> i16 {
        let score = -1 * shallow_eval(gstate);
        unmake_move(gstate);
        return score;
    }
       
    movegen_legal(gstate, &mut movebuf); 
    while let Some(mov) = movebuf.pop() {        
        make_move(gstate, mov);
        best.push(mov, eval_unmake(gstate));
    }
    
    return best.take().unwrap();
}


// # Iterative Deepening Search

pub struct IterDeepSearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    pub movebuf: SegVec<'b, MGAnyMove>,
    pub deadline: Instant
}

pub struct IterDeepSearchResult {
    pub bestmove: MGAnyMove,
    pub depth_achieved: u8
}

/// Conducts a time-limited search for the optimal move.
/// If the game is ended, then there are no legal moves,
/// so this procedure returns `None`.
pub fn iterdeep_search(mut ctx: IterDeepSearchContext) -> IterDeepSearchResult {
    let mut bestmove = search_shallow(ctx.gstate, ctx.movebuf.extend());
    let mut eval_lookahead: u8 = 1;
    loop {
        let result = search(SearchContext { gstate: ctx.gstate, eval_lookahead, 
            movebuf: ctx.movebuf.extend(), deadline: ctx.deadline });
        match result {
            Err(DeadlineElapsed) => break,
            Ok(mov) => bestmove = mov,
        }
        eval_lookahead += 1;
    }
    return IterDeepSearchResult { bestmove, depth_achieved: eval_lookahead };
}
