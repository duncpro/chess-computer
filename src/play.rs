use crate::makemove::doturn;
use crate::misc::SegVec;
use crate::search::iterdeep_search;
use crate::search::IterDeepSearchContext;
use crate::gamestate::FastPosition;
use crate::gamestate::GameStatus;
use crate::gamestate::get_status;
use crate::movegen::types::MGAnyMove;
use std::cell::RefCell;
use std::time::Duration;
use std::time::Instant;

pub fn automove(gstate: &mut FastPosition, think_time: Duration) {
    if !(matches!(get_status(gstate), GameStatus::Incomplete)) {
        return; }
    
    let best_move = iterdeep_search(IterDeepSearchContext {
        gstate, pmoves: SegVec::new(&mut RefCell::default()),
        deadline: Instant::now() + think_time }).unwrap();

    doturn(gstate, best_move);
 }


