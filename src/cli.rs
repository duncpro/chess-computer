use crate::gamestate::ChessGame;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::MGAnyMove;
use crate::piece::Color;
use crate::piece::Piece;
use crate::piece::PieceGrid;

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

pub fn print_board(board: &PieceGrid) {
    let mut i: u8 = 0;
    print!("\n");
    println!("  A B C D E F G H");
    for rank_i in 0..8u8 {
        print!("{} ", rank_i + 1);
        for file_i in 0..8u8 {
            let coord = StandardCoordinate::from_index(i);
            let is_colored_sq = (((rank_i % 2) + file_i) % 2) == 0;
            if is_colored_sq { print!("\x1b[42m") }
            match board.get(coord) {
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
    println!("Press any key to continue");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
}

pub fn prompt_move(state: &mut ChessGame) -> MGAnyMove {
    let mut moves: Vec<MGAnyMove> = Vec::new();
    movegen_legal(state, &mut moves);
    for (i, mov) in moves.iter().enumerate() {
        print!("{}. ", i);
        match mov {
            MGAnyMove::Piece(pmov) => print!("{} -> {}", pmov.origin, pmov.destin),
            MGAnyMove::Castle(direction) => {
                print!("Castle ");
                match direction {
                    FileDirection::Queenside => print!("Queenside"),
                    FileDirection::Kingside => print!("Kingside"),
                }
            }
        }
        print!("\n");
    }

    print!("Move #: ");
    let selected_index = prompt_usize();
    return moves[selected_index];
}

pub fn prompt_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.pop();
    return input.parse::<usize>().unwrap();
}
