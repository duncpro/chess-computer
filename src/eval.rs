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

#[derive(Clone, Copy)]
#[repr(u8)]
enum Mode { Min = 0, Max = 1 }

impl Mode {
    fn from_index(index: u8) -> Self {
        assert!(index < 2);
        unsafe { std::mem::transmute(index) }
    }
    fn index(self) -> u8 { self as u8 }
    fn sign(self) -> i8 { (self.index() as i8) * 2 - 1 }
    fn inverse(self) -> Self {
        Self::from_index((self.index() + 1) % 2)
    }
}

struct Context<'a, 'b> {
    gstate: &'a mut GameState,
    depth: u8,
    moves: SegVec<'b, MGPieceMove>,
    mode: Mode
}

fn shallow_eval(gstate: &mut GameState) -> i32 {
    matdiff(&gstate.bbs)
}

fn eval(mut ctx: Context) -> i32 {
    movegen_pmoves(ctx.gstate, &mut ctx.moves);

    // If we have no moves, then either its a stalemate,
    // or we're in checkmate. Either way, it's not a good
    // position to be in.
    if ctx.moves.is_empty() { return i32::MIN; }
    
    if ctx.depth == 0 { 
        return shallow_eval(ctx.gstate);
    }

    let mut parent_score: i32 = i32::MIN + 1;

    macro_rules! eval_child {
        () => {{
            swap_active(ctx.gstate);
            let mut child_score = eval(Context { gstate: ctx.gstate,
                depth: ctx.depth - 1, moves: ctx.moves.extend(),
                mode: ctx.mode.inverse() });
            child_score *= i32::from(ctx.mode.sign());
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
    
    parent_score *= i32::from(ctx.mode.sign());  
    return parent_score;
}

