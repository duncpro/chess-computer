use chess_solver_3::grid::Side;
use chess_solver_3::persistence::apply_gstr;
use chess_solver_3::piece::Color;
use chess_solver_3::stdinit::new_std_chess_position;

#[test]
fn test_castle_kingside_white() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6;").unwrap();
    // White's castling rights should be intact at this point as neither the
    // king nor the rook have been moved.
    assert!(game.crights.get(Side::Kingside, Color::White));
    assert!(game.crights.get(Side::Queenside, Color::White));
    apply_gstr(&mut game, "CastleKingside").unwrap();
    // White's castling rights should be revoked completely now that they've
    // castled.
    assert!(!game.crights.get(Side::Kingside, Color::White));
    assert!(!game.crights.get(Side::Queenside, Color::White));
    // Black's castling rights should remain intact, however.
    assert!(game.crights.get(Side::Kingside, Color::Black));
    assert!(game.crights.get(Side::Queenside, Color::Black));
}

#[test]
fn test_castle_kingside_black() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6; A2:A3").unwrap();
    // Black's castling rights should be intact at this point as neither the
    // king nor the rook have been moved.
    assert!(game.crights.get(Side::Kingside, Color::Black));
    assert!(game.crights.get(Side::Queenside, Color::Black));
    apply_gstr(&mut game, "CastleKingside").unwrap();
    // Black's castling rights should be revoked completely now that they've
    // castled.
    assert!(!game.crights.get(Side::Kingside, Color::Black));
    assert!(!game.crights.get(Side::Queenside, Color::Black));
    // White's castling rights should remain intact, however.
    assert!(game.crights.get(Side::Kingside, Color::White));
    assert!(game.crights.get(Side::Queenside, Color::White));
}

#[test]
fn test_castle_queenside_white() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; \
        B1:C3; B8:C6;").unwrap();
    // White's castling rights should be intact at this point as neither the
    // king nor the rook have been moved.
    assert!(game.crights.get(Side::Queenside, Color::White));
    assert!(game.crights.get(Side::Kingside, Color::White));
    apply_gstr(&mut game, "CastleQueenside").unwrap();
    // White's castling rights should be revoked completely now that they've
    // castled.
    assert!(!game.crights.get(Side::Kingside, Color::White));
    assert!(!game.crights.get(Side::Queenside, Color::White));
    // Black's castling rights should remain intact, however.
    assert!(game.crights.get(Side::Kingside, Color::Black));
    assert!(game.crights.get(Side::Queenside, Color::Black));
}

#[test]
fn test_castle_queenside_black() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; \
    B1:C3; B8:C6; A2:A3").unwrap();
    // Black's castling rights should be intact at this point as neither the
    // king nor the rook have been moved.
    assert!(game.crights.get(Side::Kingside, Color::Black));
    assert!(game.crights.get(Side::Queenside, Color::Black));
    apply_gstr(&mut game, "CastleQueenside").unwrap();
    // Black's castling rights should be revoked completely now that they've
    // castled.
    assert!(!game.crights.get(Side::Kingside, Color::Black));
    assert!(!game.crights.get(Side::Queenside, Color::Black));
    // White's castling rights should remain intact, however.
    assert!(game.crights.get(Side::Kingside, Color::White));
    assert!(game.crights.get(Side::Queenside, Color::White));
}