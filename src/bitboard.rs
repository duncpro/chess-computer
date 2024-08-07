use std::marker::PhantomData;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::Coordinate;
use crate::coordinates::CoordinateSystem;
use crate::bits::bitscan;
use crate::coordinates::FileMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::RankMajorCS;
use crate::getbit;
use crate::grid::StandardCoordinate;
use crate::setbit;
use crate::unsetbit;

pub type Bitlane = u8;
pub type RawBitboard = u64;

// # `Bitboard`

/// A type-safety wrapper around [`RawBitboard`] which enforces 
/// coordinate system consistency for bitwise union, intersection, etc.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bitboard<C: CoordinateSystem> {
    raw_bb: RawBitboard,
    pd: PhantomData<C>
}

impl<C: CoordinateSystem> std::ops::BitAndAssign for Bitboard<C> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.raw_bb &= rhs.raw_bb
    }
}

impl<C: CoordinateSystem> std::ops::BitOrAssign for Bitboard<C> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.raw_bb |= rhs.raw_bb;
    }
}

impl<C: CoordinateSystem> std::ops::BitAnd for Bitboard<C> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw_bb & rhs.raw_bb)
    }
}

impl<C: CoordinateSystem> std::ops::BitOr for Bitboard<C> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw_bb | rhs.raw_bb)
    }
}

impl<C: CoordinateSystem> std::ops::Not for Bitboard<C> {
    type Output = Self;
    fn not(self) -> Self::Output { Self::from_raw(!self.raw_bb) }
}

impl<C: CoordinateSystem> std::ops::ShlAssign<u8> for Bitboard<C> {
    fn shl_assign(&mut self, rhs: u8) {
        self.raw_bb <<= rhs;
    }
}

impl<C: CoordinateSystem> std::ops::ShrAssign<u8> for Bitboard<C> {
    fn shr_assign(&mut self, rhs: u8) {
        self.raw_bb >>= rhs;
    }
}

impl<C: CoordinateSystem> Bitboard<C> {
    // Accessors
    pub fn scan(self) -> impl Iterator<Item = Coordinate<C>> {
        return bitscan(self.raw_bb)
            .map(|index| Coordinate::<C>::from_index(index));
    }

    pub fn single(self) -> Coordinate<C> {
        assert_eq!(self.raw_bb.count_ones(), 1);
        let index = self.raw_bb.trailing_zeros() as u8;
        return Coordinate::from_index(index);
    }

    pub fn includes(self, coord: Coordinate<C>) -> bool {
        let index = coord.index();
        return getbit!(self.raw_bb, index);
    }

    pub fn is_not_empty(self) -> bool { self.raw_bb.count_ones() > 0 }

    pub fn copy_bitlane(self, begin: Coordinate<C>) -> Bitlane {
        (self.raw_bb >> begin.index()) as u8
    }
        
    pub fn raw(self) -> RawBitboard { self.raw_bb } 

    pub fn count(self) -> u8 { self.raw_bb.count_ones() as u8 }

    // Modifiers

    pub fn invert(&mut self) { self.raw_bb = !self.raw_bb }

    pub fn set(&mut self, pos: Coordinate<C>) { 
        setbit!(self.raw_bb, pos.index());
    }

    pub fn unset(&mut self, pos: Coordinate<C>) {
        unsetbit!(self.raw_bb, pos.index());
    }

    // Constructors

    pub fn from_bitlane(begin: Coordinate<C>, mut bitlane: Bitlane, len: u8) -> Self {
        assert!(len <= 8, "bitlane has maximum width of 8 bits");
        let mask_offset = 8 - len;
        bitlane <<= mask_offset;
        bitlane >>= mask_offset;
        
        let mut raw_bb = (bitlane as u64) << begin.index();
        return Self::from_raw(raw_bb);
    }
    
    pub const fn empty() -> Self { Self::from_raw(0) }

    /// Constructs a type-safe bitboard from a raw bitboard.
    pub const fn from_raw(raw_bb: RawBitboard) -> Self {
        Self { raw_bb, pd: PhantomData }
    }
}

pub fn print_bitboard(bb: RawBitboard) {
    println!("LSB");
    for i in (0..8) {
        let bitlane = (bb >> (8 * i)) as Bitlane;
        for j in (0..8) {
            let mask = 1 << j;
            let selected: bool = mask & bitlane != 0;
            print!("{} ", if selected { '1' } else { '0' });
        }
        print!("\n");
    }
    println!("                MSB");
}


// # `MDBitboard`

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct MDBitboard { array: [RawBitboard; 4] }

impl MDBitboard {
    pub fn get<C: CoordinateSystem>(&self) -> Bitboard<C> {
        Bitboard::<C>::from_raw(self.array[C::INDEX])
    }

    pub fn get_mut<C: CoordinateSystem>(&mut self) -> &mut Bitboard<C> {
        unsafe {
            std::mem::transmute::<&mut RawBitboard, &mut Bitboard<C>>(
                &mut self.array[C::INDEX])
        }
    }

    pub fn set(&mut self, pos: StandardCoordinate) {
        self.get_mut::<RankMajorCS>().set(pos.into());
        self.get_mut::<FileMajorCS>().set(pos.into());
        self.get_mut::<ProdiagonalMajorCS>().set(pos.into());
        self.get_mut::<AntidiagonalMajorCS>().set(pos.into());
    }
    
    pub fn unset(&mut self, pos: StandardCoordinate) {
        self.get_mut::<RankMajorCS>().unset(pos.into());
        self.get_mut::<FileMajorCS>().unset(pos.into());
        self.get_mut::<ProdiagonalMajorCS>().unset(pos.into());
        self.get_mut::<AntidiagonalMajorCS>().unset(pos.into());
    }
}

