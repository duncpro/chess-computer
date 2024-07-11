use chess_solver_3::cli::print_board;
use chess_solver_3::crights::CastlingRights;
use chess_solver_3::enpassant::is_enpassant_vuln;
use chess_solver_3::grid::File;
use chess_solver_3::grid::FileDirection;
use chess_solver_3::persistence::apply_gstr;
use chess_solver_3::piece::Color;
use chess_solver_3::stdinit::new_std_chess_position;

#[test]
fn test_castle_kingside() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6;").unwrap();
    apply_gstr(&mut game, "CastleKingside").unwrap();
    assert!(!game.crights.get(FileDirection::Kingside, Color::White));
    apply_gstr(&mut game, "CastleKingside").unwrap();
    assert!(!game.crights.get(FileDirection::Kingside, Color::Black));
    assert_eq!(game.crights, CastlingRights::NONE);
    print_board(&game.p_lut);
}

#[test]
fn test_castle_queenside() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E3; E7:E6; D2:D3; D7:D6; D1:E2; D8:E7; C1:D2; C8:D7; \
        B1:C3; B8:C6;").unwrap();
    apply_gstr(&mut game, "CastleQueenside").unwrap();
    assert!(!game.crights.get(FileDirection::Queenside, Color::White));
    apply_gstr(&mut game, "CastleQueenside").unwrap();
    assert!(!game.crights.get(FileDirection::Queenside, Color::Black));
    assert_eq!(game.crights, CastlingRights::NONE);
    print_board(&game.p_lut);
}

#[test]
fn test_enpassant() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E4; A7:A6; E4:E5; D7:D5;").unwrap();
    println!("{:?}", game.movelog);
    print_board(&game.p_lut);
    assert_eq!(is_enpassant_vuln(&mut game), Some(File::D));
    apply_gstr(&mut game, "E5:D6").unwrap();
    print_board(&game.p_lut);
}