use crate::gamestate::{ChessGame, LoggedMove};
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::mov::AnyMove;
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::GeneratedMove;
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

pub fn print_board(board: &ChessGame) {
    let mut i: u8 = 0;
    print!("\n");
    println!("  A B C D E F G H");
    for rank_i in 0..8u8 {
        print!("{} ", rank_i + 1);
        for file_i in 0..8u8 {
            let coord = StandardCoordinate::from_index(i);
            let is_colored_sq = (((rank_i % 2) + file_i) % 2) == 0;
            if is_colored_sq { print!("\x1b[42m") }

            if let Some(last_entry) = board.movelog.last() {
                if let LoggedMove::Piece(pmove) = last_entry.lmove {
                    if coord == pmove.mgmove.origin {
                        print!("\x1b[45m")
                    }
                    if coord == pmove.mgmove.destin {
                        print!("\x1b[35m")
                    }
                }
            }

            match board.p_lut.get(coord) {
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

pub fn prompt_move(state: &mut ChessGame) -> AnyMove {
    let mut moves: Vec<GeneratedMove> = Vec::new();
    movegen_legal(state, &mut moves);
    for (i, genmove) in moves.iter().enumerate() {
        print!("{}. ", i);
        match genmove.mov {
            AnyMove::Piece(pmov) => print!("{} -> {}", pmov.origin, pmov.destin),
            AnyMove::Castle(direction) => {
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
    return moves[selected_index].mov;
}

pub fn prompt_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.pop();
    return input.parse::<usize>().unwrap();
}
