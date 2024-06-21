use std::ops::{BitAndAssign, BitOrAssign};

use crate::coordinates::{RankMajorCS, FileMajorCS, ProdiagonalMajorCS, AntidiagonalMajorCS, CoordinateSystem};
use crate::bitboard::{RawBitboard, Bitboard};

pub struct MDBitboard { array: [RawBitboard; 4] }

impl MDBitboard {
    pub fn get<C: CoordinateSystem>(&self) -> Bitboard<C> {
        Bitboard::<C>::from_raw(self.array[C::INDEX])
    }
}
