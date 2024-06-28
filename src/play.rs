use crate::cli::print_board;
use crate::cli::prompt_ok;
use crate::expect_match;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::makemove::doturn;
use crate::makemove::fill_tile;
use crate::mat_eval::matdiff;
use crate::misc::SegVec;
use crate::piece::ColorTable;
use crate::piece::Piece;
use crate::search::iterdeep_search;
use crate::search::IterDeepSearchContext;
use crate::gamestate::FastPosition;
use crate::gamestate::GameStatus;
use crate::gamestate::status;
use crate::movegen::types::MGAnyMove;
use std::cell::RefCell;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use crate::movegen::dispatch::count_legal_moves;

pub fn new_game() -> FastPosition {
    use crate::piece::Color::*;
    use crate::piece::Species::*;
    let mut state = FastPosition::default();
    for i in 0..8u8 {
        fill_tile(&mut state, StandardCoordinate::new(
            Rank::from_index(1), File::from_index(i)),
            Piece::new(White, Pawn));
        fill_tile(&mut state, StandardCoordinate::new(
            Rank::from_index(6), File::from_index(i)),
            Piece::new(Black, Pawn));
    }

    macro_rules! fill_base_rank {
        ($color:expr) => {            
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::A), Piece::new($color, Rook));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::B), Piece::new($color, Knight));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::C), Piece::new($color, Bishop));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::D), Piece::new($color, Queen));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::E), Piece::new($color, King));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::F), Piece::new($color, Bishop));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::G), Piece::new($color, Knight));
            fill_tile(&mut state, StandardCoordinate::new(
                $color.base_rank(), File::H), Piece::new($color, Rook));
        }
    };

    fill_base_rank!(White);
    fill_base_rank!(Black);
    
    return state;
}

pub fn automove(gstate: &mut FastPosition, think_time: Duration) {
    if matches!(status(gstate), GameStatus::Complete(_)) {
        return; }
    
    let search_result = iterdeep_search(IterDeepSearchContext {
        gstate, movebuf: SegVec::new(&mut RefCell::default()),
        deadline: Instant::now() + think_time });

    println!("Depth: {} (plys considered)", search_result.depth_achieved);

    if let MGAnyMove::Piece(piece_move) = search_result.bestmove {
        println!("Move: {} to {}", piece_move.origin, piece_move.destin);
    }

    doturn(gstate, search_result.bestmove);
 }


pub fn selfplay(time_constraints: ColorTable<Duration>) {
    let mut state: FastPosition = new_game();

    println!("New Self-Play Game");
    print_board(&state.occupant_lut);
    print!("\n");
    // prompt_ok();
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        println!("{}'s turn to move", state.active_player());
        println!("Legal Moves: {}", count_legal_moves(&mut state));
        println!("Move #: {}", state.movelog.len() + 1);
        let think_time = time_constraints[state.active_player()];
        automove(&mut state, think_time);
        println!("Material Difference: {}", -1 * matdiff(&state.bbs));
        print_board(&state.occupant_lut);
        print!("\n");
        std::io::stdout().flush();
        // prompt_ok();
    }
    
    println!("Game Over");
    expect_match!(status(&mut state), GameStatus::Complete(result));
    match result {
        crate::gamestate::GameResult::Diff(victor) => 
            println!("{} won", victor),
        crate::gamestate::GameResult::Tie => 
            println!("draw"),
    }
}
