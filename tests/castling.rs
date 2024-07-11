use chess_solver_3::cli::print_board;
use chess_solver_3::persistence::load_game;

#[test]
fn test_castle_kingside() {
    let gstr = "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6; CastleKingside; \
        CastleKingside;";
    let mut game = load_game(gstr).unwrap();
    print_board(&game.p_lut);
}

#[test]
fn test_castle_queenside() {
    let gstr = "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; B1:C3; B8:C6; \
        CastleQueenside; CastleQueenside;";
    let mut game = load_game(gstr).unwrap();
    print_board(&game.p_lut);
}