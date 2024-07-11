use chess_solver_3::cli::print_board;
use chess_solver_3::persistence::load_game;
use chess_solver_3::persistence::LoadGameErr;

#[test]
fn test_castle_kingside() -> Result<(), LoadGameErr> {
    let gstr = "E2:E4; E7:E5; F1:E2; F8:E7; G1:F3; G8:F6; CastleKingside; \
        CastleKingside;";
    let mut game = load_game(gstr)?;
    print_board(&game.p_lut);
    Ok(())
}