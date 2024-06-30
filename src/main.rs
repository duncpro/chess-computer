#![feature(variant_count)]
#![feature(core_intrinsics)]

mod bitboard;
mod bits;
mod attack;
mod cache;
mod cli;
mod coordinates;
mod crights;
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

fn main() {
    use std::time::Duration;
    use crate::play::selfplay;
    use crate::piece::ColorTable;
    let time_constraints = ColorTable::new([Duration::from_secs(5), 
        Duration::from_secs(5)]);
    selfplay(time_constraints);
}
