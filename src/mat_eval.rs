use crate::bitboard::Bitboard;
use crate::build_itable;
use crate::coordinates::StandardCS;
use crate::piece::Color;
use crate::piece::Species;
use crate::piece::Species::*;
use crate::gamestate::Bitboards;

build_itable!(SPECIES_VALUE: [u8; 6], |table| {
    macro_rules! put { ($species:expr, $value:expr) => {{
        table[$species.index() as usize] = $value; }}; }

    put!(King, 0);
    put!(Pawn, 1);
    put!(Knight, 3);
    put!(Bishop, 3);
    put!(Rook, 5);
    put!(Queen, 9)
});

fn count_class(board: &Bitboards, color: Color, species: Species) -> i16 {
    let bitboard: Bitboard<StandardCS> = board.class(color, species);
    return i16::from(bitboard.count());
}

fn matval_class(board: &Bitboards, color: Color, species: Species) -> i16 {
    let count = count_class(board, color, species);
    let lut_key = usize::from(species.index());
    return count * i16::from(SPECIES_VALUE[lut_key]);
}

fn matval(board: &Bitboards, color: Color) -> i16 {
    let mut total: i16 = 0;
    total += matval_class(board, color, King);
    total += matval_class(board, color, Pawn);
    total += matval_class(board, color, Knight);
    total += matval_class(board, color, Bishop);
    total += matval_class(board, color, Rook);
    total += matval_class(board, color, Queen);
    return total;
}

pub fn calc_matdiff(board: &Bitboards) -> i16 {
    let mut value: i16 = 0;
    value += matval(board, board.active_player);
    value -= matval(board, board.active_player.oppo());
    return value;
}
