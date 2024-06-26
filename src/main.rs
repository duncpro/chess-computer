#![allow(unused)]

// Unstable features are used **only** in testing modules
// and never in the actual application.
#![feature(variant_count)]

use std::time::Duration;

use play::selfplay;

mod bitboard;
mod bits;
mod check;
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
    selfplay(Duration::from_secs(5));
}
