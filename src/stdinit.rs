use crate::cache::HashChars;
use crate::crights::CastlingRights;
use crate::gamestate::FastPosition;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::makemove::fill_tile;
use crate::piece::Color;
use crate::piece::Color::*;
use crate::piece::Piece;
use crate::piece::Species::*;

pub fn new_std_chess_position() -> FastPosition {
    let mut state = FastPosition::new(HashChars::new_random());
    state.hash.toggle_crights(state.crights);
    state.crights = CastlingRights::INITIAL;
    state.hash.toggle_crights(state.crights);
    fill_base_rank(&mut state, White);
    fill_base_rank(&mut state, Black);    
    for i in 0..8u8 {
        for j in 0..2u8 {
            let color = Color::from_index(j);
            let coord = StandardCoordinate::new(color.pawn_rank(), 
                File::from_index(i));
            let piece = Piece::new(color, Pawn);
            fill_tile(&mut state, coord, piece);
        }
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

