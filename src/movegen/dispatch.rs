use crate::gamestate::ChessGame;
use crate::misc::PushCount;
use crate::misc::Push;
use crate::movegen::bishop::movegen_bishops;
use crate::movegen::castle::movegen_castle;
use crate::movegen::king::movegen_king;
use crate::movegen::knight::movegen_knights;
use crate::movegen::queen::movegen_queens;
use crate::movegen::rook::movegen_rooks;
use crate::movegen::pawn::movegen_pawns;
use crate::movegen::types::MGContext;
use std::cell::RefCell;
use crate::movegen::types::GeneratedMove;

fn movegen_dispatch(ctx: &mut MGContext<impl Push<GeneratedMove>>)
{
    movegen_king(ctx);
    movegen_knights(ctx);
    movegen_bishops(ctx);
    movegen_rooks(ctx);
    movegen_queens(ctx);
    movegen_pawns(ctx);
    movegen_castle(ctx);
}

pub fn movegen_legal(state: &mut ChessGame, moves: &mut impl Push<GeneratedMove>) {
    let state_cell = RefCell::new(state);
    let mut ctx = MGContext::new(&state_cell, moves);
    movegen_dispatch(&mut ctx);
}

pub fn count_legal_moves(state: &mut ChessGame) -> usize {
    let mut counter: PushCount<GeneratedMove> = PushCount::new();
    movegen_legal(state, &mut counter);
    return counter.count();
}

