fn main() {
    use std::time::Duration;
    use chess_solver_3::play::selfplay;
    use chess_solver_3::play::humanplay;
    use chess_solver_3::piece::ColorTable;
    // humanplay(Duration::from_secs(20));
    selfplay(ColorTable::from_array([Duration::from_secs(10), Duration::from_secs(10)]));


    // let mut gstr = String::new();
    // let mut file = std::fs::File::open("debuggame.txt").unwrap();
    // file.read_to_string(&mut gstr).unwrap();
}
