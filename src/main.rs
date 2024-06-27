// Unstable features are used **only** in testing modules
// and never in the actual application.
#![feature(variant_count)]

mod bitboard;
mod bits;
mod attack;
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
mod search;
mod sliders;


fn main() {
    use std::time::Duration;
    use crate::play::selfplay;
    selfplay(Duration::from_millis(10 * 1000));
}
