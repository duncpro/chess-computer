use crate::cache::Cache;
use crate::cache::HashChars;
use crate::crights::CastlingRights;
use crate::gamestate::FastPosition;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::makemove::fill_tile;
use crate::piece::Color;
use crate::piece::Color::*;
use crate::piece::Piece;
use crate::piece::Species::*;

pub fn new_std_chess_position() -> FastPosition {
    let mut state = FastPosition::new(HashChars::new([0; 32]),
        Cache::new(4 * 1024 /* MB */));
    state.crights = CastlingRights::INITIAL;
    state.hash.toggle_crights(state.crights);
    fill_base_rank(&mut state, White);
    fill_base_rank(&mut state, Black);    
    for i in 0..8u8 {
        fill_tile(&mut state, StandardCoordinate::new(
            Rank::from_index(1), File::from_index(i)),
            Piece::new(White, Pawn));
        fill_tile(&mut state, StandardCoordinate::new(
            Rank::from_index(6), File::from_index(i)),
            Piece::new(Black, Pawn));
    }
    return state;
}

fn fill_base_rank(state: &mut FastPosition, color: Color) {
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::A), Piece::new(color, Rook));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::B), Piece::new(color, Knight));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::C), Piece::new(color, Bishop));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::D), Piece::new(color, Queen));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::E), Piece::new(color, King));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::F), Piece::new(color, Bishop));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::G), Piece::new(color, Knight));
    fill_tile(state, StandardCoordinate::new(
        color.base_rank(), File::H), Piece::new(color, Rook));
}

