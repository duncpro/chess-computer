use crate::gamestate::GameState;
use super::bishop::movegen_bishops;
use super::king::movegen_king;
use super::knight::movegen_knights;
use super::queen::movegen_queens;
use super::rook::movegen_rooks;
use super::pawn::movegen_pawns;

pub fn movegen(state: &GameState) {
    movegen_pawns(state);
    movegen_rooks(state);
    movegen_knights(state);
    movegen_bishops(state);
    movegen_queens(state);
    movegen_king(state);
}

