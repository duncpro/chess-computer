#![feature(variant_count)]
#![feature(core_intrinsics)]

use crate::play::humanplay;

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
    humanplay(Duration::from_secs(45));
}
