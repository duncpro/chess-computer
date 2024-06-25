use crate::eval::DeepEvalContext;
use crate::eval::MIN_EVAL_SCORE;
use crate::eval::deep_eval;
use crate::movegen_castle;
use crate::makemove::make_pmove;
use crate::makemove::make_castle;
use crate::makemove::unmake_move;
use crate::makemove::swap_active;
use crate::misc::SegVec;
use crate::movegen::dispatch::movegen_pmoves;
use crate::movegen::moveset::MGPieceMove;
use crate::movegen::moveset::MGAnyMove;
use crate::gamestate::GameState;
use crate::grid::FileDirection;
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

    macro_rules! eval_child {
        ($mgmove:expr) => { 
            swap_active(ctx.gstate);
            let result = deep_eval(DeepEvalContext {
                gstate: ctx.gstate,
                maxdepth: ctx.maxdepth,
                pmoves: ctx.pmoves.extend(),
                deadline: ctx.deadline,
            });
            unmake_move(ctx.gstate);
            match result {
                Some(score) => {
                    if score > best_score {
                        best_move = Some($mgmove);
                        best_score = score;
                    }
                },
                None => return SearchResult::DeadlineElapsed,
            }
        };
    }
    
    movegen_pmoves(ctx.gstate, &mut ctx.pmoves);
    for pmove in ctx.pmoves.as_slice().iter() {
        make_pmove(ctx.gstate, *pmove);
        eval_child!(MGAnyMove::Piece(*pmove));   
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
    gstate: &'a mut GameState,
    pmoves: SegVec<'b, MGPieceMove>,
    deadline: Instant
}

pub fn iterdeep_search(mut ctx: IterDeepSearchContext) -> Option<MGAnyMove> {
    todo!()
}
