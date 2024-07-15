use chess_solver_3::enpassant::is_enpassant_vuln;
use chess_solver_3::grid::File;
use chess_solver_3::persistence::apply_gstr;
use chess_solver_3::stdinit::new_std_chess_position;

#[test]
fn test_enpassant_vuln() {
    let mut game = new_std_chess_position();
    apply_gstr(&mut game, "E2:E4; A7:A6; E4:E5; D7:D5;").unwrap();
    assert_eq!(is_enpassant_vuln(&mut game), Some(File::D));
    apply_gstr(&mut game, "E5:D6").unwrap();
    assert_eq!(is_enpassant_vuln(&mut game), None);
}