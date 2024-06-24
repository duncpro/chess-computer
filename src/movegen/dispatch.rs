use crate::gamestate::GameState;
use crate::makemove::test_pmove;
use super::bishop::movegen_bishops;
use super::castle::movegen_castle;
use super::king::movegen_king;
use super::knight::movegen_knights;
use super::moveset::{MoveSet, MGPieceMove};
use super::queen::movegen_queens;
use super::rook::movegen_rooks;
use super::pawn::movegen_pawns;

fn pseudo_movegen_pmoves(state: &GameState, moves: &mut Vec<MGPieceMove>) {
    movegen_pawns(state, moves);
    movegen_rooks(state, moves);
    movegen_knights(state, moves);
    movegen_bishops(state, moves);
    movegen_queens(state, moves);
    movegen_king(state, moves);
}

pub fn movegen(state: &mut GameState, moves: &mut MoveSet) {
    // Generate pseudo-legal piece moves
    assert!(moves.pmoves.is_empty());
    pseudo_movegen_pmoves(state, &mut moves.pmoves);
    // Filter out illegal moves
    moves.pmoves.retain(|pmove| test_pmove(state, *pmove));
    // Generate castling moves, these do not require a legality check.
    movegen_castle(state, moves);
}
