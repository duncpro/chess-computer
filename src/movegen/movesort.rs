use crate::mat_eval::SPECIES_VALUE;
use crate::misc::SegVec;
use crate::movegen::types::PMGMove;
use crate::gamestate::FastPosition;

pub fn movesort(state: &FastPosition, movebuf: &mut SegVec<PMGMove>) {
    movebuf.sort_by_key(
        |pmove| -1 * calc_sort_key(state, *pmove));
}

fn calc_sort_key(state: &FastPosition, pmove: PMGMove) -> i32 {
    let attacker_value = state.p_lut.get(pmove.origin)
        .map(|p| SPECIES_VALUE[usize::from(p.species().index())])
        .unwrap();

    let victim_value = state.p_lut.get(pmove.target)
        .map(|p| SPECIES_VALUE[usize::from(p.species().index())])
        .unwrap_or(attacker_value);    

    return i32::from(victim_value) - i32::from(attacker_value);
}

