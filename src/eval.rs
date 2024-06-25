use crate::check::is_check;
use crate::gamestate::{GameState, locate_king_stdc};
use crate::makemove::{swap_active, make_pmove, unmake_move};
use crate::mat_eval::matdiff;
use crate::misc::{SegVec, pick};
use crate::movegen::dispatch::movegen_pmoves;
use crate::movegen::moveset::MGPieceMove;

struct Context<'a, 'b> {
    gstate: &'a mut GameState,
    depth: u8,
    moves: SegVec<'b, MGPieceMove>
}

#[derive(Clone, Copy)]
pub enum Eval {
    Lost,
    Won,
    Indeterminant(i32 /* score */),
    Stalemate
}

fn shallow_eval(gstate: &mut GameState) -> Eval {
    Eval::Indeterminant(matdiff(&gstate.bbs))
}

fn deep_eval(mut ctx: Context) -> Eval {
    movegen_pmoves(ctx.gstate, &mut ctx.moves);

    if ctx.moves.is_empty() { 
        return pick(ctx.gstate.bbs.is_check(), Eval::Lost,
            Eval::Stalemate); 
    }
    
    if ctx.depth == 0 { 
        return shallow_eval(ctx.gstate);
    }

    for pmove in ctx.moves.as_slice().iter() {
        make_pmove(ctx.gstate, *pmove);
        let score = deep_eval(Context { gstate: ctx.gstate,
            depth: ctx.depth - 1, moves: ctx.moves.extend() });
        unmake_move(ctx.gstate);
    }    

    todo!()
}

