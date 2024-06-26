use crate::cli::print_board;
use crate::cli::prompt_ok;
use crate::expect_match;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::makemove::doturn;
use crate::makemove::fill_tile;
use crate::misc::SegVec;
use crate::piece::Piece;
use crate::search::iterdeep_search;
use crate::search::IterDeepSearchContext;
use crate::gamestate::FastPosition;
use crate::gamestate::GameStatus;
use crate::gamestate::status;
use crate::movegen::types::MGAnyMove;
use std::cell::RefCell;
use std::time::Duration;
use std::time::Instant;

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
    
    let best_move = iterdeep_search(IterDeepSearchContext {
        gstate, pmoves: SegVec::new(&mut RefCell::default()),
        deadline: Instant::now() + think_time }).unwrap();

    doturn(gstate, best_move);
 }


pub fn selfplay(think_time: Duration) {
    let mut state: FastPosition = new_game();

    println!("New Self-Play Game");
    print_board(&state.occupant_lut);
    print!("\n");
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        automove(&mut state, think_time);
        print_board(&state.occupant_lut);
        print!("\n");
        prompt_ok();
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
