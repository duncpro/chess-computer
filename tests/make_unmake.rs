//! This test suite verifies that for every class of move, making *and unmaking* a move in that
//! class leaves the board unchanged. The goal being to verify that `unmake_move` is a
//! proper inverse of `make_move` in every conceivable case. This test suite *does not* verify
//! that the move was applied correctly however, and so this suite alone is not sufficient
//! to verify the total correctness of `make_move` and `unmake_move`.

use chess_solver_3::gamestate::ChessGame;
use chess_solver_3::grid::{File, Side, Rank, StandardCoordinate};
use chess_solver_3::makemove::{make_move, unmake_move};
use chess_solver_3::mov::{AnyMove, PieceMove};
use chess_solver_3::persistence::apply_gstr;
use chess_solver_3::stdinit::new_std_chess_position;

pub fn test_make_unmake(board: &mut ChessGame, mov: AnyMove) {
    let before_snapshot = board.clone();
    make_move(board, mov);
    unmake_move(board);
    let after_snapshot = board.clone();
    assert!(before_snapshot == after_snapshot);
}

#[test]
pub fn test_make_unmake_castle_kingside_white() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6;").unwrap();
    test_make_unmake(&mut board, AnyMove::Castle(Side::Kingside));
}

#[test]
pub fn test_make_unmake_castle_kingside_black() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6; A2:A3").unwrap();
    test_make_unmake(&mut board, AnyMove::Castle(Side::Kingside));
}

#[test]
pub fn test_make_unmake_castle_queenside_white() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; \
        B1:C3; B8:C6;").unwrap();
    test_make_unmake(&mut board, AnyMove::Castle(Side::Queenside));
}

#[test]
pub fn test_make_unmake_castle_queenside_black() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; \
        B1:C3; B8:C6; A2:A3").unwrap();
    test_make_unmake(&mut board, AnyMove::Castle(Side::Queenside));
}

#[test]
pub fn test_make_unmake_enpassant_white() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E4; A7:A6; E4:E5; D7:D5;").unwrap();
    let origin_sq = StandardCoordinate::new(Rank::from_index(4), File::E);
    let destin_sq = StandardCoordinate::new(Rank::from_index(5), File::D);
    test_make_unmake(&mut board, AnyMove::Piece(PieceMove::new_basic(origin_sq, destin_sq)));
}

#[test]
pub fn test_make_unmake_enpassant_black() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E4; A7:A5; D2:D4; A5:A4; B2:B4; ").unwrap();
    let origin_sq = StandardCoordinate::new(Rank::from_index(3), File::A);
    let destin_sq = StandardCoordinate::new(Rank::from_index(2), File::B);
    test_make_unmake(&mut board, AnyMove::Piece(PieceMove::new_basic(origin_sq, destin_sq)));
}

#[test]
pub fn test_make_unmake_pmove_white() {
    let mut board = new_std_chess_position();
    let origin_sq = StandardCoordinate::new(Rank::from_index(1), File::E);
    let destin_sq = StandardCoordinate::new(Rank::from_index(3), File::E);
    test_make_unmake(&mut board, AnyMove::Piece(PieceMove::new_basic(origin_sq, destin_sq)));
}

#[test]
pub fn test_make_unmake_pmove_black() {
    let mut board = new_std_chess_position();
    apply_gstr(&mut board, "E2:E4").unwrap();
    let origin_sq = StandardCoordinate::new(Rank::from_index(6), File::A);
    let destin_sq = StandardCoordinate::new(Rank::from_index(4), File::A);
    test_make_unmake(&mut board, AnyMove::Piece(PieceMove::new_basic(origin_sq, destin_sq)));
}