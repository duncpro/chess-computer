use crate::cache::{Cache, CacheValue};
use crate::eval::BELOW_MIN_SCORE;
use crate::eval::DeepEvalContext;
use crate::eval::DeepEvalException;
use crate::eval::deep_eval;
use crate::eval::shallow_eval;
use crate::makemove::make_move;
use crate::misc::Max;
use crate::makemove::unmake_move;
use crate::misc::SegVec;
use crate::movegen::dispatch::movegen_legal;
use crate::gamestate::ChessGame;
use std::time::Instant;
use crate::mov::AnyMove;
use crate::movesort::movegen_legal_sorted;
use crate::movegen::types::GeneratedMove;
// # Search

struct SearchContext<'a, 'b, 'c> {
    pub gstate: &'a mut ChessGame,
    pub lookahead: u8,
    pub movebuf: SegVec<'b, GeneratedMove>,
    pub deadline: Instant,
    pub cache: &'c mut Cache
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
fn search(mut ctx: SearchContext) -> Result<AnyMove, DeadlineElapsed> {
    let mut best: Max<GeneratedMove, i16> = Max::new(BELOW_MIN_SCORE);
    movegen_legal_sorted(ctx.gstate, &mut ctx.movebuf, ctx.cache);
    assert!(ctx.movebuf.len() > 0);
    while let Some(genmov) = ctx.movebuf.pop() {
        make_move(ctx.gstate, genmov.mov);
        let result = deep_eval(DeepEvalContext { gstate: ctx.gstate, 
            lookahead: ctx.lookahead - 1, movebuf: ctx.movebuf.extend(),
            deadline: ctx.deadline, cutoff: best.value(), cache: ctx.cache });
        unmake_move(ctx.gstate);
        match result {
            Err(DeepEvalException::DeadlineElapsed) => return Err(DeadlineElapsed),
            Err(DeepEvalException::Cut) => {},
            Ok(score) => { best.push(genmov, score * -1) }
        }
    }
    let bestmov_id = best.item().unwrap().gen_id;
    ctx.cache.update(ctx.gstate, ctx.lookahead, CacheValue { score: best.value(),
        bestmov_id });
    return Ok(best.item().unwrap().mov);
}


fn search_shallow(gstate: &mut ChessGame, mut movebuf: SegVec<GeneratedMove>) -> AnyMove {
    let mut best: Max<AnyMove, i16> = Max::new(BELOW_MIN_SCORE);
    movegen_legal(gstate, &mut movebuf); 
    while let Some(genmov) = movebuf.pop() {
        make_move(gstate, genmov.mov);
        let score = -1 * shallow_eval(gstate);
        unmake_move(gstate);
        best.push(genmov.mov, score);
    }
    return best.item().unwrap();
}


// # Iterative Deepening Search

pub struct IterDeepSearchContext<'a, 'b, 'c> {
    pub gstate: &'a mut ChessGame,
    pub movebuf: SegVec<'b, GeneratedMove>,
    pub deadline: Instant,
    pub cache: &'c mut Cache
}

pub struct IterDeepSearchResult {
    pub bestmove: AnyMove,
    pub depth_achieved: u8
}

/// Conducts a time-limited search for the optimal move. 
/// This procedure will complete at least a shallow search, regardless of 
/// the deadline, but deeper searches are time-constrained.
pub fn iterdeep_search(mut ctx: IterDeepSearchContext) -> IterDeepSearchResult {
    let mut bestmove = search_shallow(ctx.gstate, ctx.movebuf.extend());
    let mut eval_lookahead: u8 = 1;
    loop {
        let result = search(SearchContext { gstate: ctx.gstate,
            lookahead: eval_lookahead,
            movebuf: ctx.movebuf.extend(), deadline: ctx.deadline,
            cache: ctx.cache });
        match result {
            Err(DeadlineElapsed) => break,
            Ok(mov) => bestmove = mov,
        }
        eval_lookahead += 1;
    }
    return IterDeepSearchResult { bestmove, depth_achieved: eval_lookahead };
}
