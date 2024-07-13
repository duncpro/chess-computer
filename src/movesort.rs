use crate::cache::Cache;
use crate::gamestate::ChessGame;
use crate::mat_eval::get_species_value;
use crate::misc::{Push, SegVec};
use crate::mov::{AnyMove, get_target_sq, PieceMove};
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::GeneratedMove;
use crate::piece::Species;

pub fn movegen_legal_sorted(state: &mut ChessGame, moves: &mut SegVec<GeneratedMove>,
                            cache: &Cache)
{
    movegen_legal(state, moves);

    let mut bestmov: Option<GeneratedMove> = None;
    if let Some(value) = cache.lookup_any(state) {
        // Note that a hash collision is possible!
        // Therefore this cache entry could be for a
        // different position than we expect, and so
        // `bestmov_id` might exceed the bounds
        // of th emove list.
        let mut bestmov_id = usize::from(value.bestmov_id);
        if bestmov_id < moves.len() {
            let removed = moves.swap_remove(bestmov_id);
            bestmov = Some(removed);
        }
    }

    moves.as_mut_slice().sort_unstable_by_key(|genmov| score_mov(state, genmov.mov));

    if let Some(mov) = bestmov {
        moves.push(mov);
    }
}

// 5. Principal Variation
// 4. Winning Captures
// 3. Castling Moves
// 2. Advance Pawn Moves
// 1. Other Moves

pub fn score_mov(state: &mut ChessGame, mov: AnyMove) -> u8 {
    match mov {
        AnyMove::Piece(pmov) => {
            let mover = state.p_lut.get(pmov.origin).unwrap();
            let target = get_target_sq(pmov, state);
            if let Some(victim) = state.p_lut.get(target) {
                let victim_value = get_species_value(victim.species());
                let mover_value = get_species_value(mover.species());
                if victim_value > mover_value {
                    return 4;
                }
            } else {
                if mover.species() == Species::Pawn {
                    return 2;
                }
            }
            return 1;
        }
        AnyMove::Castle(direction) => {
            return 3;
        }
    }
}
