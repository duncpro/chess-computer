use crate::bitboard::Bitboard;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::ChessGame;
use crate::makemove::test_pmove;
use crate::misc::Push;
use crate::misc::PushFilter;
use crate::piece::Color;
use crate::piece::Species;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use std::cell::Ref;
use std::cell::RefCell;

// # `PMGMove`

/// There are a variety of chessmove representations in this program.
/// `PMGMove` in particular is produced by the move generation routines
/// and is used during move-application and move-reversal as well.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PMGMove /* Piece Move Generation Move */ {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub promote: Option<Species>
}

impl PMGMove {
    pub fn new_basic(origin: StandardCoordinate, destin: StandardCoordinate) -> Self
    {
        Self { origin, destin, promote: None }
    }

    pub fn new_promote(origin: StandardCoordinate, destin: StandardCoordinate,
        desire: Species) -> Self
    {
        Self { origin, destin, promote: Some(desire) }
    }
}

// # `MGAnyMove`

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MGAnyMove {
    Piece(PMGMove),
    Castle(FileDirection)
}

// # `PMGContext`

pub struct PMGContext<'a, 'b, 'c, P>
where P: Push<PMGMove>
{ 
    gstate: &'a RefCell<&'b mut ChessGame>,
    pmoves: &'c mut P
}

impl<'a, 'b, 'c, P: Push<PMGMove>> PMGContext<'a, 'b, 'c, P> {
    pub fn new(gstate: &'a RefCell<&'b mut ChessGame>,
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

    pub fn inspect<T>(&self, f: impl Fn(&ChessGame) -> T) -> T
    where T: Copy
    {
        return f(*self.gstate.borrow());
    }

    pub fn push(&mut self, pmove: PMGMove) {
        self.pmoves.push(pmove);
    }
}
