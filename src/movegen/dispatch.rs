use crate::gamestate::GameState;
use crate::makemove::test_pmove;
use crate::misc::SegVec;
use super::bishop::movegen_bishops;
use super::king::movegen_king;
use super::knight::movegen_knights;
use super::moveset::MGPieceMove;
use super::queen::movegen_queens;
use super::rook::movegen_rooks;
use super::pawn::movegen_pawns;

fn pseudo_movegen_pmoves(state: &GameState, moves: &mut SegVec<MGPieceMove>) {
    movegen_pawns(state, moves);
    movegen_rooks(state, moves);
    movegen_knights(state, moves);
    movegen_bishops(state, moves);
    movegen_queens(state, moves);
    movegen_king(state, moves);
}

pub fn movegen_pmoves(state: &mut GameState, moves: &mut SegVec<MGPieceMove>) {
    // Generate pseudo-legal piece moves
    pseudo_movegen_pmoves(state, moves);
    // Filter out illegal moves
    moves.retain(|pmove| test_pmove(state, *pmove));
}
