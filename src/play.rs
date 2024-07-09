use crate::cache::Cache;
use crate::cache::HashChars;
use crate::cli::print_board;
use crate::cli::prompt_move;
use crate::cli::prompt_ok;
use crate::expect_match;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::makemove::make_move;
use crate::makemove::fill_tile;
use crate::mat_eval::calc_matdiff;
use crate::misc::SegVec;
use crate::piece::Color;
use crate::piece::ColorTable;
use crate::piece::Piece;
use crate::search::iterdeep_search;
use crate::search::IterDeepSearchContext;
use crate::gamestate::FastPosition;
use crate::gamestate::GameStatus;
use crate::gamestate::status;
use crate::movegen::types::MGAnyMove;
use crate::stdinit::new_std_chess_position;
use std::cell::RefCell;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use crate::movegen::dispatch::count_legal_moves;

pub fn automove(gstate: &mut FastPosition, think_time: Duration, cache: &mut Cache) {
    if matches!(status(gstate), GameStatus::Complete(_)) {
        return; }
    
    let search_result = iterdeep_search(IterDeepSearchContext {
        gstate, movebuf: SegVec::new(&mut RefCell::default()),
        deadline: Instant::now() + think_time, cache });

    println!("Depth: {} (plys considered)", search_result.depth_achieved);

    if let MGAnyMove::Piece(piece_move) = search_result.bestmove {
        println!("Move: {} to {}", piece_move.origin, piece_move.destin);
    }
    // TODO: Castling!

    make_move(gstate, search_result.bestmove);
}


pub fn selfplay(time_constraints: ColorTable<Duration>) {
    let mut state: FastPosition = new_std_chess_position();
    let mut cache: Cache = Cache::new(1024 * 2);

    println!("New Self-Play Game");
    print_board(&state.p_lut);
    print!("\n");
    // prompt_ok();
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        println!("{}'s turn to move", state.active_player());
        println!("Legal Moves: {}", count_legal_moves(&mut state));
        println!("Move #: {}", state.movelog.len() + 1);
        let think_time = time_constraints[state.active_player()];
        automove(&mut state, think_time, &mut cache);
        print_board(&state.p_lut);
        println!("Hash: {}", state.hash.value());
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

pub fn humanmove(gstate: &mut FastPosition) {
    let mov = prompt_move(gstate);
    make_move(gstate, mov);
}

pub fn humanplay(think_time: Duration) {
    let mut state: FastPosition = new_std_chess_position();
    let mut cache: Cache = Cache::new(1024 * 4);

    println!("New Self-Play Game");
    print_board(&state.p_lut);
    print!("\n");
    // prompt_ok();
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        println!("{}'s turn to move", state.active_player());
        println!("Legal Moves: {}", count_legal_moves(&mut state));
        println!("Ply #: {}", state.movelog.len() + 1);

        match state.active_player() {
            Color::White => humanmove(&mut state),
            Color::Black => automove(&mut state, think_time, &mut cache),
        }
        
        println!("Material Difference: {}", -1 * calc_matdiff(&state.bbs));
        print_board(&state.p_lut);
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
