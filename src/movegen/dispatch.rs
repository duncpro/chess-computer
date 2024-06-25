use crate::gamestate::FastPosition;
use crate::makemove::test_pmove;
use crate::misc::{SegVec, Push, PushFilter, PushCount};
use super::bishop::movegen_bishops;
use super::king::movegen_king;
use super::knight::movegen_knights;
use super::moveset::MGPieceMove;
use super::queen::movegen_queens;
use super::rook::movegen_rooks;
use super::pawn::movegen_pawns;

fn pseudo_movegen_pmoves(state: &FastPosition, moves: &mut impl Push<MGPieceMove>) {
    movegen_pawns(state, moves);
    movegen_rooks(state, moves);
    movegen_knights(state, moves);
    movegen_bishops(state, moves);
    movegen_queens(state, moves);
    movegen_king(state, moves);
}

pub fn movegen_pmoves(state: &mut FastPosition, moves: &mut SegVec<MGPieceMove>) {
    // Generate pseudo-legal piece moves
    pseudo_movegen_pmoves(state, moves);
    // Filter out illegal moves
    moves.retain(|pmove| test_pmove(state, *pmove));
}

pub fn movegen_count_pmoves(state: &mut FastPosition) -> usize {
    let mut counter = PushFilter::new(PushCount::new(), 
        |pmove| test_pmove(state, *pmove));

    pseudo_movegen_pmoves(state, &mut counter);
    return counter.wrapped().count();
}
