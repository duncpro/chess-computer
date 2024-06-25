use crate::eval::DeepEvalContext;
use crate::eval::MIN_EVAL_SCORE;
use crate::eval::deep_eval;
use crate::misc::SegVec;
use crate::movegen::dispatch::movegen_pmoves;
use crate::movegen::moveset::MGPieceMove;
use crate::movegen::moveset::MGAnyMove;
use crate::gamestate::GameState;
use std::time::Instant;

// # Search

pub struct SearchContext<'a, 'b> {
    pub gstate: &'a mut GameState,
    pub maxdepth: u8,
    pub pmoves: SegVec<'b, MGPieceMove>,
    pub deadline: Instant
}

enum SearchResult {
    DeadlineElapsed,
    Completed(Option<MGAnyMove>)
}

/// Conducts a depth-limited time-limited search for the 
/// optimal move. 
/// - If the deadline elapses before the search completes, 
///   `SearchResult::DeadlineElapsed` is returned. 
/// - If the maximum depth is exhausted before reaching
///   a conclusion, the approximate optimal move is returned.
/// - If the player has no legal moves, `SearchResult::Completed(None)` 
///   is returned.
fn search(mut ctx: SearchContext) -> SearchResult {
    let mut best_move: Option<MGAnyMove> = None;
    let mut best_score: i32 = MIN_EVAL_SCORE;
    
    movegen_pmoves(ctx.gstate, &mut ctx.pmoves);
    for pmove in ctx.pmoves.as_slice().iter() {
        let result = deep_eval(DeepEvalContext {
            gstate: ctx.gstate,
            maxdepth: ctx.maxdepth,
            pmoves: ctx.pmoves.extend(),
            deadline: ctx.deadline,
        });
        match result {
            Some(score) => {
                if score > best_score {
                    best_move = Some(MGAnyMove::Piece(*pmove));
                    best_score = score;
                }
            },
            None => return SearchResult::DeadlineElapsed,
        }
    }

    return SearchResult::Completed(best_move);
}

// # Iterative Deepening Search

pub struct IterDeepSearchContext<'a, 'b> {
    gstate: &'a mut GameState,
    pmoves: SegVec<'b, MGPieceMove>,
    deadline: Instant
}

pub fn iterdeep_search(mut ctx: IterDeepSearchContext) -> Option<MGAnyMove> {
    todo!()
}
