use crate::bitboard::Bitboard;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::FastPosition;
use crate::gamestate::SpecialPieceMove;
use crate::makemove::test_pmove;
use crate::misc::Push;
use crate::misc::PushFilter;
use crate::piece::Color;
use crate::piece::Species;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use std::cell::Ref;
use std::cell::RefCell;

// # `MGPieceMove`

#[derive(Clone, Copy)]
pub struct PMGMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub target: StandardCoordinate,
    pub special: Option<SpecialPieceMove>,
    pub promote: Option<Species>
}

impl PMGMove {
    pub fn new(origin: StandardCoordinate, destin: StandardCoordinate) 
    -> Self 
    {
        Self { origin, destin, target: destin, special: None,
            promote: None }
    }
}

// # `MGAnyMove`

#[derive(Clone, Copy)]
pub enum MGAnyMove {
    Piece(PMGMove),
    Castle(FileDirection)
}

// # `PMGContext`

pub struct PMGContext<'a, 'b, 'c, P>
where P: Push<PMGMove>
{ 
    gstate: &'a RefCell<&'b mut FastPosition>,
    pmoves: &'c mut P
}

impl<'a, 'b, 'c, P: Push<PMGMove>> PMGContext<'a, 'b, 'c, P> {
    pub fn new(gstate: &'a RefCell<&'b mut FastPosition>, 
        pmoves: &'c mut P) -> Self 
    {
           Self { gstate, pmoves }
    }
    
    pub fn class<C>(&self, color: Color, species: Species) -> Bitboard<C> 
    where C: CoordinateSystem
    {
        return self.gstate.borrow().bbs.class::<C>(color, species);
    }

    pub fn active_player(&self) -> Color {
        return self.gstate.borrow().active_player();
    }

    pub fn inspect<T>(&self, f: impl Fn(&FastPosition) -> T) -> T
    where T: Copy
    {
        return f(*self.gstate.borrow());
    }

    pub fn push(&mut self, pmove: PMGMove) {
        self.pmoves.push(pmove);
    }
}
