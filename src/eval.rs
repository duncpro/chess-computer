use std::cmp::max;

use crate::check::is_check;
use crate::gamestate::{GameState, locate_king_stdc};
use crate::grid::FileDirection;
use crate::makemove::{swap_active, make_pmove, unmake_move, make_castle};
use crate::mat_eval::matdiff;
use crate::misc::{SegVec, pick};
use crate::movegen::dispatch::movegen_pmoves;
use crate::movegen::moveset::MGPieceMove;
use crate::movegen_castle;


struct Context<'a, 'b> {
    gstate: &'a mut GameState,
    depth: u8,
    moves: SegVec<'b, MGPieceMove>
}

fn shallow_eval(gstate: &mut GameState) -> i32 {
    matdiff(&gstate.bbs)
}

const MIN_SCORE: i32 = i32::MIN + 1;

fn eval(mut ctx: Context) -> i32 {
    movegen_pmoves(ctx.gstate, &mut ctx.moves);

    // If we have no moves, then either its a stalemate,
    // or we're in checkmate. Either way, it's not a good
    // position to be in.
    if ctx.moves.is_empty() { return MIN_SCORE; }
    
    if ctx.depth == 0 { 
        return shallow_eval(ctx.gstate);
    }

    let mut parent_score: i32 = MIN_SCORE;

    macro_rules! eval_child {
        () => {{
            swap_active(ctx.gstate);
            let child_score = -1 * eval(Context { gstate: ctx.gstate,
                depth: ctx.depth - 1, moves: ctx.moves.extend() });
            parent_score = max(parent_score, child_score);
            unmake_move(ctx.gstate);
        }};
    }
    
    for pmove in ctx.moves.as_slice().iter() {
        make_pmove(ctx.gstate, *pmove);
        eval_child!();
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
    
    return parent_score;
}

