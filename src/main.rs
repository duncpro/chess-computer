#![allow(unused)]

// Unstable features are used **only** in testing modules
// and never in the actual application.
#![feature(variant_count)]

use std::time::Duration;

use play::selfplay;
use crate::bitboard::print_bitboard;
use crate::grid::{File, Rank, StandardCoordinate};
use crate::movegen::king::king_attack;
use crate::movegen::knight::knight_attack;

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
    selfplay(Duration::from_millis(5 * 1000));
    // let coord = StandardCoordinate::new(Rank::from_index(7), File::F);
    // print_bitboard(king_attack(coord.into()).raw())
}
