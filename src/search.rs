use crate::eval::DeepEvalContext;
use crate::eval::MIN_EVAL_SCORE;
use crate::eval::deep_eval;
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
use std::time::Instant;

// # Search

pub struct SearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    pub maxdepth: u8,
    pub pmoves: SegVec<'b, PMGMove>,
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
/// 
/// Note that when the maximum depth is set to zero, this procedure
/// will *never* return `SearchResult::DeadlineElapsed`. Instead,
/// it will perform a complete but shallow evaluation of each move,
/// regardless of the imposed deadline.
fn search(mut ctx: SearchContext) -> SearchResult {
    let mut best_move: Option<MGAnyMove> = None;
    let mut best_score: i32 = MIN_EVAL_SCORE;

    macro_rules! eval_child {
        ($mgmove:expr) => { 
            swap_active(ctx.gstate);
            let result = deep_eval(DeepEvalContext { gstate: ctx.gstate,
                maxdepth: ctx.maxdepth, pmoves: ctx.pmoves.extend(),
                deadline: ctx.deadline, });
            unmake_move(ctx.gstate);
            match result {
                Some(child_score) => {
                    let score = child_score * -1;
                    if score > best_score {
                        best_move = Some($mgmove);
                        best_score = score;
                    }
                },
                None => return SearchResult::DeadlineElapsed,
            }
        };
    }
    
    movegen_legal_pmoves(ctx.gstate, &mut ctx.pmoves);
    while let Some(pmove) = ctx.pmoves.pop() {
        make_pmove(ctx.gstate, pmove);
        eval_child!(MGAnyMove::Piece(pmove));
    }

    macro_rules! eval_castle { 
        ($side:ident) => {
            if movegen_castle!($side, ctx.gstate) {
                make_castle(ctx.gstate, FileDirection::$side);
                eval_child!(MGAnyMove::Castle(FileDirection::$side));
            }
        };
    }

    eval_castle!(Kingside);
    eval_castle!(Queenside);

    return SearchResult::Completed(best_move);
}

// # Iterative Deepening Search

pub struct IterDeepSearchContext<'a, 'b> {
    pub gstate: &'a mut FastPosition,
    pub pmoves: SegVec<'b, PMGMove>,
    pub deadline: Instant
}

/// Conducts a time-limited search for the optimal move.
/// If the game is ended, then there are no legal moves,
/// so this procedure returns `None`.
pub fn iterdeep_search(mut ctx: IterDeepSearchContext) -> Option<MGAnyMove> {
    let mut maxdepth: u8 = 0;
    let mut prev_bestmove: Option<MGAnyMove> = None;
    loop {
        let result = search(SearchContext { gstate: ctx.gstate, maxdepth, 
            pmoves: ctx.pmoves.extend(), deadline: ctx.deadline });
        match result {
            SearchResult::DeadlineElapsed =>
                { assert!(maxdepth > 0); break },
            SearchResult::Completed(bestmove) =>
                prev_bestmove = bestmove,
        }
        maxdepth += 1;
    }
    return prev_bestmove;
}
