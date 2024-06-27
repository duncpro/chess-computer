use crate::grid::StandardCoordinate;
use crate::grid::GridTable;
use crate::piece::Color;
use crate::piece::Piece;

pub fn get_unicode_symbol(piece: Piece) -> &'static str {
    use crate::piece::Color::*;
    use crate::piece::Species::*;
    
    match piece.color() {
        White => match piece.species() {
            Pawn   => "♙",
            Rook   => "♖",
            Knight => "♘",
            Bishop => "♗",
            Queen  => "♕",
            King   => "♔",
        },
        Black => match piece.species() {
            Pawn   => "♟",
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
    print!("\n");
    for rank_i in 0..8u8 {
        for file_i in 0..8u8 {
            let coord = StandardCoordinate::from_index(i);
            let is_colored_sq = (((rank_i % 2) + file_i) % 2) == 0;
            if is_colored_sq { print!("\x1b[42m") }
            match board[coord] {
                Some(piece) => print!("{}", get_unicode_symbol(piece)),
                None => print!(" "),
            }
            print!(" ");
            print!("\x1b[0m");
            i += 1;
        }
        print!("\n");
    }
}

pub fn prompt_ok() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
}
