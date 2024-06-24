#![allow(unused)]

// Unstable features are used **only** in testing modules
// and never in the actual application.
#![feature(variant_count)]

mod bitboard;
mod bits;
mod check;
mod coordinates;
mod crights;
mod eval;
mod gamestate;
mod grid;
mod lane;
mod makemove;
mod misc;
mod movegen;
mod piece;
mod rmrel;
mod search;
mod sliders;

fn main() {
    println!("Hello, world!");
}
