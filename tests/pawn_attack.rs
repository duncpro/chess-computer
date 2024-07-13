use chess_solver_3::bitboard::print_bitboard;
use chess_solver_3::movegen::pawn::{pawn_attack, reverse_pawn_attack};

#[test]
pub fn test_pawn_attack() {
    let bb = pawn_attack(6);
    print_bitboard(bb);
}

#[test]
pub fn test_reverse_pawn_attack() {
    let bb = reverse_pawn_attack(63);
    print_bitboard(bb);
}