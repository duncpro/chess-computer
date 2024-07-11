fn main() {
    use std::time::Duration;
    use chess_solver_3::play::selfplay;
    use chess_solver_3::play::humanplay;
    use chess_solver_3::piece::ColorTable;
    // humanplay(Duration::from_secs(20));
    selfplay(ColorTable::from_array([Duration::from_secs(5), Duration::from_secs(5)]));
}
