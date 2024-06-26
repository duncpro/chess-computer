use crate::grid::StandardCoordinate;
use crate::grid::GridTable;
use crate::piece::Piece;

pub fn get_unicode_symbol(piece: Piece) -> &'static str {
    use crate::piece::Color::*;
    use crate::piece::Species::*;
    
    match piece.color() {
        White => match piece.species() {
            Pawn   => "♙",
            Rook   => "♖",
            Knight => "♘",
            Bishop => "♙",
            Queen  => "♕",
            King   => "♔",
        },
        Black => match piece.species() {
            Pawn   => "♟︎",
            Rook   => "♜",
            Knight => "♞",
            Bishop => "♝",
            Queen  => "♛",
            King   => "♚",
        },
    }
}

pub fn print_board(board: &GridTable<Option<Piece>>) {
    let mut i: u8 = 0;
    for _ in 0..8 {
        for _ in 0..8 {
            let coord = StandardCoordinate::from_index(i);
            match board[coord] {
                Some(piece) => print!(get_unicode_symbol(piece)),
                None => print!(" "),
            }
            print!(" ");
            i += 1;
        }
        print!("\n");
    }
}
