#![feature(variant_count)]

mod bitboard;
mod bits;
mod attack;
mod cache;
mod cli;
mod coordinates;
mod crights;
mod enpassant;
mod eval;
mod gamestate;
mod grid;
mod laneutils;
mod makemove;
mod mat_eval;
mod misc;
mod movegen;
mod piece;
mod play;
mod rmrel;
mod repetitions;
mod search;
mod sliders;
mod stdinit;

fn main() {
    use std::time::Duration;
    use crate::play::selfplay;
    use crate::play::humanplay;
    use crate::piece::ColorTable;
    // humanplay(Duration::from_secs(5));
    selfplay(ColorTable::from_array([Duration::from_secs(5); 2]));
}
