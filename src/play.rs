use crate::cache::Cache;
use crate::cache::HashChars;
use crate::cli::print_board;
use crate::cli::prompt_move;
use crate::cli::prompt_ok;
use crate::expect_match;
use crate::gamestate::GameResult;
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
use crate::gamestate::ChessGame;
use crate::gamestate::GameStatus;
use crate::gamestate::status;
use crate::mov::AnyMove;
use crate::stdinit::new_std_chess_position;
use crate::movegen::dispatch::count_legal_moves;
use std::cell::RefCell;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use crate::persistence::{apply_gstr, write_move};

pub fn automove(gstate: &mut ChessGame, think_time: Duration, cache: &mut Cache,
    gamefile: &mut std::fs::File)
{
    if matches!(status(gstate), GameStatus::Complete(_)) {
        return; }
    
    let search_result = iterdeep_search(IterDeepSearchContext {
        gstate, movebuf: SegVec::new(&mut RefCell::default()),
        deadline: Instant::now() + think_time, cache });

    println!("Depth: {} (plys considered)", search_result.depth_achieved);
    println!("Best Move: {:?}", search_result.bestmove);

    write_move(gamefile, search_result.bestmove);
    gamefile.flush().unwrap();
    make_move(gstate, search_result.bestmove);
}


pub fn selfplay(time_constraints: ColorTable<Duration>) {
    let mut state: ChessGame = new_std_chess_position();
    let mut cache: Cache = Cache::new(1024 * 6);
    let mut gamefile = std::fs::File::create("../debuggame.txt").unwrap();

    println!("New Self-Play Game");
    print_board(&state);
    print!("\n");
    // prompt_ok();
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        println!("{}'s turn to move", state.active_player());
        println!("Legal Moves: {}", count_legal_moves(&mut state));
        println!("Crights: {:?}", state.crights);
        println!("Move #: {}", state.movelog.len() + 1);
        let think_time = time_constraints[state.active_player()];
        automove(&mut state, think_time, &mut cache, &mut gamefile);
        print_board(&state);
        println!("Hash: {}", state.hash.value());
        print!("\n");
        std::io::stdout().flush();
        // prompt_ok();
    }
    
    println!("Game Over");
    expect_match!(status(&mut state), GameStatus::Complete(result));
    match result {
        GameResult::Diff(victor) => println!("{} won", victor),
        GameResult::Tie => println!("draw"),
    }
}

pub fn humanmove(gstate: &mut ChessGame, gamefile: &mut std::fs::File) {
    let mov = prompt_move(gstate);
    write_move(gamefile, mov).unwrap();
    gamefile.flush().unwrap();
    make_move(gstate, mov);
}

pub fn humanplay(think_time: Duration) {
    let mut state: ChessGame = new_std_chess_position();
    let mut cache: Cache = Cache::new(1024 * 6);
    let mut gamefile = std::fs::File::create("lasthumangame.txt").unwrap();

    println!("New Self-Play Game");
    print_board(&state);
    print!("\n");
    // prompt_ok();
    
    while matches!(status(&mut state), GameStatus::Incomplete) {
        println!("{}'s turn to move", state.active_player());
        // println!("Castling Rights: {}", state.crights);
        println!("Legal Moves: {}", count_legal_moves(&mut state));
        println!("Ply #: {}", state.movelog.len() + 1);

        match state.active_player() {
            Color::White => humanmove(&mut state, &mut gamefile),
            Color::Black => automove(&mut state, think_time, &mut cache, &mut gamefile),
        }
        
        println!("Material Difference: {}", -1 * calc_matdiff(&state.bbs));
        print_board(&state);
        print!("\n");
        std::io::stdout().flush();
        // prompt_ok();
    }
    
    println!("Game Over");
    expect_match!(status(&mut state), GameStatus::Complete(result));
    match result {
        GameResult::Diff(victor) => 
            println!("{} won", victor),
        GameResult::Tie => 
            println!("draw"),
    }
}