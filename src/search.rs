use crate::eval::DeadlineElapsed;
use crate::eval::DeepEvalWDeadlineContext;
use crate::eval::MIN_SCORE;
use crate::eval::deep_eval_w_deadline;
use crate::eval::eval_shallow;
use crate::misc::Max;
use crate::movegen_castle;
use crate::makemove::make_pmove;
use crate::makemove::make_castle;
use crate::makemove::unmake_move;
use crate::makemove::swap_active;
use crate::misc::SegVec;
use crate::movegen::dispatch::movegen_legal_pmoves;
use crate::movegen::types::PMGMove;
use crate::movegen::types::MGAnyMove;
use crate::gamestate::FastPosition;
use crate::grid::FileDirection;
use crate::movegen::castle::movegen_castle_queenside;
use crate::movegen::castle::movegen_castle_kingside;
use std::time::Instant;

// # Search

pub struct SearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    /// The `lookahead` (as in [`DeepEvilWDeadlineContext`]) used when
    /// evaluating each position resultant from each legal move
    /// the active-player has to choose from.
    pub eval_lookahead: u8,
    pub movebuf: SegVec<'b, PMGMove>,
    pub deadline: Instant
}

/// Conducts a time-limited depth-first search for the optimal/
/// approximately optimal move. 
///
/// This procedure assumes that the game **is not** concluded,
/// and so there **must be** an optimal move. If this procedure
/// is called while the game is completed (there are no legal moves)
/// it will [`panic`]. When the deadline elapses, search is cancelled and
/// `Err(DeadlineElapsed)` is returned.
fn search(mut ctx: SearchContext) -> Result<MGAnyMove, DeadlineElapsed> {
    let mut best: Max<MGAnyMove, i32> = Max::new(MIN_SCORE);

    fn eval_unmake(ctx: &mut SearchContext) -> Result<i32, DeadlineElapsed> {
        swap_active(ctx.gstate);
        let score = deep_eval_w_deadline(DeepEvalWDeadlineContext { 
            gstate: ctx.gstate, lookahead: ctx.eval_lookahead, 
            movebuf: ctx.movebuf.extend(), deadline: ctx.deadline });
        unmake_move(ctx.gstate);
        return Ok(score? * -1);
    }
       
    movegen_legal_pmoves(ctx.gstate, &mut ctx.movebuf); 
    while let Some(pmove) = ctx.movebuf.pop() {        
        make_pmove(ctx.gstate, pmove);
        best.push(MGAnyMove::Piece(pmove), eval_unmake(&mut ctx)?);
    }
    if movegen_castle_queenside(ctx.gstate) {
        make_castle(ctx.gstate, FileDirection::Queenside);
        best.push(MGAnyMove::Castle(FileDirection::Queenside), eval_unmake(&mut ctx)?);
    }
    if movegen_castle_kingside(ctx.gstate) {
        make_castle(ctx.gstate, FileDirection::Kingside);
        best.push(MGAnyMove::Castle(FileDirection::Kingside), eval_unmake(&mut ctx)?);
    }

    return Ok(best.take().unwrap());
}


fn search_shallow(gstate: &mut FastPosition, mut movebuf: SegVec<PMGMove>) -> MGAnyMove {
    let mut best: Max<MGAnyMove, i32> = Max::new(MIN_SCORE);

    fn eval_unmake(gstate: &mut FastPosition) -> i32 {
        swap_active(gstate);
        let score = -1 * eval_shallow(gstate);
        unmake_move(gstate);
        return score;
    }
       
    movegen_legal_pmoves(gstate, &mut movebuf); 
    while let Some(pmove) = movebuf.pop() {        
        make_pmove(gstate, pmove);
        best.push(MGAnyMove::Piece(pmove), eval_unmake(gstate));
    }
    if movegen_castle_queenside(gstate) {
        make_castle(gstate, FileDirection::Queenside);
        best.push(MGAnyMove::Castle(FileDirection::Queenside), eval_unmake(gstate));
    }
    if movegen_castle_kingside(gstate) {
        make_castle(gstate, FileDirection::Kingside);
        best.push(MGAnyMove::Castle(FileDirection::Kingside), eval_unmake(gstate));
    }

    return best.take().unwrap();
}


// # Iterative Deepening Search

pub struct IterDeepSearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    pub movebuf: SegVec<'b, PMGMove>,
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
