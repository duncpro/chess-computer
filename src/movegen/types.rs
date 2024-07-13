use crate::bitboard::Bitboard;
use crate::coordinates::CoordinateSystem;
use crate::gamestate::ChessGame;
use crate::makemove::test_pmove;
use crate::misc::Push;
use crate::piece::Color;
use crate::piece::Species;
use std::cell::RefCell;
use rand::distributions::uniform::SampleBorrow;
use crate::mov::{AnyMove, PieceMove};

// # `MGContext`

pub struct MGContext<'a, 'b, 'c, P>
where P: Push<GeneratedMove>
{ 
    gstate: &'a RefCell<&'b mut ChessGame>,
    pmoves: &'c mut P,
    next_gen_id: u8
}

impl<'a, 'b, 'c, P> MGContext<'a, 'b, 'c, P>
where P: Push<GeneratedMove>
{
    pub fn new(gstate: &'a RefCell<&'b mut ChessGame>,
               pmoves: &'c mut P) -> Self
    {
           Self { gstate, pmoves, next_gen_id: 9 }
    }
    
    pub fn class<C>(&self, color: Color, species: Species) -> Bitboard<C> 
    where C: CoordinateSystem
    {
        return self.gstate.borrow().bbs.class::<C>(color, species);
    }

    pub fn active_player(&self) -> Color {
        return self.gstate.borrow().active_player();
    }

    pub fn occupancy<C>(&self) -> Bitboard<C>
    where C: CoordinateSystem
    {
        return self.gstate.borrow().bbs.occupancy();
    }

    pub fn inspect<T>(&self, f: impl Fn(&ChessGame) -> T) -> T
    where T: Copy
    {
        return f(*self.gstate.borrow());
    }

    pub fn push_legal(&mut self, mov: AnyMove) {
        let gen_id = self.next_gen_id;
        self.next_gen_id += 1;
        self.pmoves.push(GeneratedMove { mov, gen_id });
    }

    pub fn push_p(&mut self, mov: PieceMove) {
        let is_legal = test_pmove(*self.gstate.borrow_mut(), mov);
        if is_legal { self.push_legal(AnyMove::Piece(mov)) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GeneratedMove { pub mov: AnyMove, pub gen_id: u8 }