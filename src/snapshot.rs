use crate::crights::CastlingRights;
use crate::enpassant::is_enpassant_vuln;
use crate::gamestate::ChessGame;
use crate::grid::File;
use crate::piece::{Color, PieceGrid};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoardSnapshot {
    p_lut: PieceGrid,
    active_player: Color,
    crights: CastlingRights,
    enpassant_vuln: Option<File>
}

pub fn capture_snapshot(state: &ChessGame) -> BoardSnapshot {
    BoardSnapshot {
        p_lut: state.p_lut,
        active_player: state.active_player(),
        crights: state.crights,
        enpassant_vuln: is_enpassant_vuln(state)
    }
}