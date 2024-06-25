use crate::check::is_check;
use crate::gamestate::GameState;
use crate::grid::FileDirection;
use crate::makemove::make_pmove;
use crate::makemove::unmake_move;
use crate::makemove::swap_active;
use crate::makemove::make_castle;
use crate::mat_eval::matdiff;
use crate::misc::SegVec;
use crate::misc::max_inplace;
use crate::movegen::dispatch::movegen_pmoves;
use crate::movegen::moveset::MGPieceMove;
use crate::movegen_castle;
use std::time::Instant;

fn shallow_eval(gstate: &mut GameState) -> i32 {
    matdiff(&gstate.bbs)
}

pub const MIN_EVAL_SCORE: i32 = i32::MIN + 1;

pub struct DeepEvalContext<'a, 'b> {
    pub gstate: &'a mut GameState,
    pub maxdepth: u8,
    pub pmoves: SegVec<'b, MGPieceMove>,
    pub deadline: Instant
}

/// Conducts a depth-limited time-limited evaluation of 
/// a chess position. If the deadline elapses before the 
/// evaluation completes, `Option::None` is returned.
/// Otherwise, the score of the position is returned.
pub fn deep_eval(mut ctx: DeepEvalContext) -> Option<i32> {
    movegen_pmoves(ctx.gstate, &mut ctx.pmoves);

    // In the case there are no legal moves, its a stalemate,
    // or we're in checkmate. Either way, this is not a good
    // position to be in, so it gets the minimum score.
    if ctx.pmoves.is_empty() { return Some(MIN_EVAL_SCORE); }

    if ctx.maxdepth == 0 { 
        return Some(shallow_eval(ctx.gstate));
    }

    if Instant::now() > ctx.deadline { return None; }
    
    let mut parent_score: i32 = MIN_EVAL_SCORE;

    macro_rules! eval_child {
        () => {{
            swap_active(ctx.gstate);
            let result = deep_eval(DeepEvalContext { gstate: ctx.gstate,
                maxdepth: ctx.maxdepth - 1, pmoves: ctx.pmoves.extend(), 
                deadline: ctx.deadline });
            match result {
                Some(child_score) => 
                    max_inplace(&mut parent_score, -1 * child_score),
                None => return None
            }
        }};
    }
    
    for pmove in ctx.pmoves.as_slice().iter() {
        make_pmove(ctx.gstate, *pmove);
        eval_child!();
        unmake_move(ctx.gstate);
    }  

    macro_rules! eval_castle { 
        ($side:ident) => {
            if movegen_castle!($side, ctx.gstate) {
                make_castle(ctx.gstate, FileDirection::$side);
                eval_child!();
                unmake_move(ctx.gstate);
            }
        };
    }

    eval_castle!(Kingside);
    eval_castle!(Queenside);
    
    return Some(parent_score);
}

