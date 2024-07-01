use crate::gamestate::FastPosition;
use crate::grid::FileDirection;
use crate::makemove::test_pmove;
use crate::misc::PushCount;
use crate::misc::PushFilter;
use crate::misc::Push;
use crate::misc::PushMap;
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
use crate::movegen::types::MGAnyMove;
use std::cell::RefCell;

fn pmove_dispatch(ctx: &mut PMGContext<impl Push<PMGMove>>) 
{
    movegen_king(ctx);
    movegen_knights(ctx);
    movegen_bishops(ctx);
    movegen_rooks(ctx);
    movegen_queens(ctx);
    movegen_pawns(ctx);
}

fn movegen_legal_pmoves(state: &mut FastPosition, moves: &mut impl Push<PMGMove>) {
    let state_cell = RefCell::new(state);
    let mut pf = PushFilter::new(moves, 
        |pmove| test_pmove(*state_cell.borrow_mut(), *pmove));
    let mut ctx = PMGContext::new(&state_cell, &mut pf);
    pmove_dispatch(&mut ctx);
}

pub fn movegen_legal(state: &mut FastPosition, moves: &mut impl Push<MGAnyMove>) {
    movegen_legal_pmoves(state, 
        &mut PushMap::new(moves, |pmove| MGAnyMove::Piece(*pmove)));
    if movegen_castle_kingside(state) {
        moves.push(MGAnyMove::Castle(FileDirection::Kingside));
    }
    if movegen_castle_queenside(state) {
        moves.push(MGAnyMove::Castle(FileDirection::Queenside));
    }
}

pub fn count_legal_moves(state: &mut FastPosition) -> usize {
    let mut counter: PushCount<MGAnyMove> = PushCount::new();
    movegen_legal(state, &mut counter);
    return counter.count();
}

