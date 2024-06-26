use std::cell::RefCell;

use crate::gamestate::FastPosition;
use crate::makemove::test_pmove;
use crate::misc::PushCount;
use crate::misc::PushFilter;
use crate::misc::Push;
use crate::misc::SegVec;
use crate::movegen::bishop::movegen_bishops;
use crate::movegen::castle::movegen_castle_queenside;
use crate::movegen::castle::movegen_castle_kingside;
use crate::movegen::king::movegen_king;
use crate::movegen::knight::movegen_knights;
use crate::movegen::types::PMGMove;
use crate::movegen::queen::movegen_queens;
use crate::movegen::rook::movegen_rooks;
use crate::movegen::pawn::movegen_pawns;
use crate::movegen::types::PMGContext;

fn pseudo_movegen_pmoves(ctx: &mut PMGContext<impl Push<PMGMove>>) 
{
    movegen_pawns(ctx);
    movegen_rooks(ctx);
    movegen_knights(ctx);
    movegen_bishops(ctx);
    movegen_queens(ctx);
    movegen_king(ctx);
}

pub fn movegen_pmoves(state: &mut FastPosition, moves: &mut SegVec<PMGMove>) {
    // Generate pseudo-legal piece moves
    let mut ctx = PMGContext::new(todo!(), moves);
    pseudo_movegen_pmoves(&mut ctx);
    // Filter out illegal moves
    moves.retain(|pmove| test_pmove(state, *pmove));
}

fn movegen_count_pmoves(state: &mut FastPosition) -> usize {
    let state_cell = RefCell::new(state);
    let mut counter = PushFilter::new(PushCount::new(), 
        |pmove| test_pmove(*state_cell.borrow_mut(), *pmove));
    {
        let mut ctx = PMGContext::new(&state_cell, &mut counter);
        pseudo_movegen_pmoves(&mut ctx);
    }
    return counter.wrapped().count();
}

pub fn movegen_count(state: &mut FastPosition) -> usize {
    let mut count: usize = 0;
    count += movegen_castle_kingside(state) as usize;
    count += movegen_castle_queenside(state) as usize;
    count += movegen_count_pmoves(state);
    return count;
}
