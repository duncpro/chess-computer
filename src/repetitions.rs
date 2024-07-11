use crate::crights::CastlingRights;
use crate::expect_match;
use crate::gamestate::ChessGame;
use crate::gamestate::LoggedMove;
use crate::gamestate::MovelogEntry;
use crate::movegen::types::PMGMove;
use crate::gamestate::LoggedPieceMove;
use crate::piece::Color;
use crate::piece::PieceGrid;
use crate::piece::Species;

pub fn count_repetitions(current: &ChessGame) -> usize {
    let mut past_p_lut = current.p_lut;
    let mut past_active_player = current.active_player();
    let mut repeat_count: usize = 0;

    let mut i = current.movelog.len();
    loop {
        if i == 0 { break; }
        i -= 1;
        let mov = current.movelog[i];
        if mov.prev_crights != current.crights { break; }
        expect_match!(mov.lmove, LoggedMove::Piece(pmove));
        if pmove.capture.is_some() { break; }
           
        unmake_move(&mut past_p_lut, pmove);
        past_active_player.swap();

        if past_p_lut.get(pmove.mgmove.origin).unwrap().species() 
            == Species::Pawn { break; }

        let luts_eq = past_p_lut == current.p_lut;
        let acti_eq = past_active_player == current.active_player();
        if luts_eq & acti_eq { repeat_count += 1; }        
    }

    return repeat_count;
}

fn unmake_move(grid: &mut PieceGrid, pmove: LoggedPieceMove) {
    let mut piece = grid.get(pmove.mgmove.destin).unwrap();
    if pmove.mgmove.promote.is_some() {
        piece.set_species(Species::Pawn);
    }
    grid.set(pmove.mgmove.destin, None);
    grid.set(pmove.target, None);
    if let Some(capture_piece) = pmove.capture {
        grid.set(pmove.target, Some(capture_piece));
    }
    grid.set(pmove.mgmove.origin, Some(piece));
}
