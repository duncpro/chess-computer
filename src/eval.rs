use crate::bitboard::Bitboard;
use crate::build_itable;
use crate::coordinates::StandardCS;
use crate::piece::Color;
use crate::piece::Color::*;
use crate::piece::Species;
use crate::piece::Species::*;
use crate::gamestate::Bitboards;
use crate::piece::SpeciesTable;

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

fn count_class(board: &mut Bitboards, color: Color, species: Species) -> i32 {
    let bitboard: Bitboard<StandardCS> = board.class(color, species);
    return i32::from(bitboard.count());
}

fn matval_class(board: &mut Bitboards, color: Color, species: Species) -> i32 {
    let count = count_class(board, color, species);
    let lut_key = usize::from(species.index());
    return count * i32::from(SPECIES_VALUE[lut_key]);
}

fn matval(board: &mut Bitboards, color: Color) -> i32 {
    let mut total: i32 = 0;
    total += matval_class(board, color, King);
    total += matval_class(board, color, Pawn);
    total += matval_class(board, color, Knight);
    total += matval_class(board, color, Bishop);
    total += matval_class(board, color, Rook);
    total += matval_class(board, color, Queen);
    return total;
}

fn matdif(board: &mut Bitboards) -> i32 {
    let mut value: i32 = 0;
    value += matval(board, White);
    value -= matval(board, Black);
    return value;
}
