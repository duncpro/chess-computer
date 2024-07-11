use crate::cache::HashChars;
use crate::crights::CastlingRights;
use crate::gamestate::ChessGame;
use crate::grid::{File, Rank};
use crate::grid::StandardCoordinate;
use crate::makemove::fill_tile;
use crate::piece::Color;
use crate::piece::Color::*;
use crate::piece::Piece;
use crate::piece::Species::*;

pub fn new_std_chess_position() -> ChessGame {
    let mut state = ChessGame::new(HashChars::new_random());
    state.hash.toggle_crights(state.crights);
    state.crights = CastlingRights::INITIAL;
    state.hash.toggle_crights(state.crights);
    fill_base_rank(&mut state, White);
    fill_base_rank(&mut state, Black);    
    for i in 0..8u8 {
        for j in 0..2u8 {
            let color = Color::from_index(j);
            let coord = StandardCoordinate::new(Rank::pawn_rank(color),
                File::from_index(i));
            let piece = Piece::new(color, Pawn);
            fill_tile(&mut state, coord, piece);
        }
    }
    return state;
}

fn fill_base_rank(state: &mut ChessGame, color: Color) {
    let rank = Rank::base_rank(color);
    fill_tile(state, StandardCoordinate::new(rank, File::A),
              Piece::new(color, Rook));
    fill_tile(state, StandardCoordinate::new(rank, File::B),
              Piece::new(color, Knight));
    fill_tile(state, StandardCoordinate::new(rank, File::C),
              Piece::new(color, Bishop));
    fill_tile(state, StandardCoordinate::new(rank, File::D),
              Piece::new(color, Queen));
    fill_tile(state, StandardCoordinate::new(rank, File::E),
              Piece::new(color, King));
    fill_tile(state, StandardCoordinate::new(rank, File::F),
              Piece::new(color, Bishop));
    fill_tile(state, StandardCoordinate::new(rank, File::G),
              Piece::new(color, Knight));
    fill_tile(state, StandardCoordinate::new(rank, File::H),
              Piece::new(color, Rook));
}

