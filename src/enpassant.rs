use crate::grid::File;
use crate::movegen::pawn::reverse_pawn_attack;
use crate::gamestate::LoggedMove;
use crate::gamestate::ChessGame;
use crate::rmrel::relativize;

/// Determines if the opponent has made an enpassant-vulnerable
/// double pawn jump. Meaning, the opponent double jumped AND
/// the active player has a pawn in position to capture enpassant.
pub fn is_enpassant_vuln(state: &ChessGame) -> Option<File> {
    if let Some(last_entry) = state.movelog.last().copied() {
        if let LoggedMove::Piece(pmove) = last_entry.lmove {
            if pmove.is_pdj {
                let target_rmrel = relativize(pmove.mgmove.destin,
                    state.active_player());
                
                let mut bb = reverse_pawn_attack(target_rmrel + 8);
                bb &= state.bbs.affilia_rel_bbs[state.active_player()];
                bb &= state.bbs.pawn_rel_bb;

                if bb != 0 { 
                    return Some(pmove.mgmove.destin.file()) 
                }
            }
        }
    }
    return None;
}
