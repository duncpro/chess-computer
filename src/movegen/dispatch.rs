use crate::gamestate::GameState;
use super::bishop::movegen_bishops;
use super::castle::movegen_castle;
use super::king::movegen_king;
use super::knight::movegen_knights;
use super::moveset::MoveSet;
use super::queen::movegen_queens;
use super::rook::movegen_rooks;
use super::pawn::movegen_pawns;

pub fn movegen(state: &GameState, moves: &mut MoveSet) {
    movegen_pawns(state, moves);
    movegen_rooks(state, moves);
    movegen_knights(state, moves);
    movegen_bishops(state, moves);
    movegen_queens(state, moves);
    movegen_king(state, moves);
    movegen_castle(state, moves);
}

