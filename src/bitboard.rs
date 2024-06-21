use std::marker::PhantomData;
use crate::coordinates::{CoordinateSystem, RankMajorCS, Coordinate};
use crate::bitscan::bitscan;

pub type Bitlane = u8;

pub type RawBitboard = u64;

/// A type-safety wrapper around [`RawBitboard`] which enforces 
/// coordinate system consistency for bitwise union, intersection, etc.
#[derive(Clone, Copy)]
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
    pub fn scan(self) -> impl Iterator<Item = Coordinate<C>> {
        return bitscan(self.raw_bb)
            .map(|index| Coordinate::<C>::from_index(index));
    }

    pub fn includes(self, coord: Coordinate<C>) -> bool {
        let mask = (1 << coord.index());
        return self.raw_bb & mask == mask;
    }

    pub fn copy_bitlane(self, begin: Coordinate<C>, count: u8) -> Bitlane {
        assert!(count <= 8, "bitlane has maximum width of 8 bits");
        let mut byte = (self.raw_bb >> begin.index()) as u8;
        let mask_offset = 8 - count;
        byte <<= mask_offset;
        byte >>= mask_offset;
        return byte;
    }

    pub fn invert(&mut self) { self.raw_bb = !self.raw_bb }

    pub fn raw(self) -> RawBitboard { self.raw_bb } 

    // Constructors

    pub fn from_bitlane(begin: Coordinate<C>, bitlane: Bitlane) -> Self {
        let mut raw_bb = (bitlane as u64) << begin.index();
        return Self::from_raw(raw_bb);
    }

    pub const fn empty() -> Self { Self::from_raw(0) }

    /// Constructs a type-safe bitboard from a raw bitboard.
    pub const fn from_raw(raw_bb: RawBitboard) -> Self {
        Self { raw_bb, pd: PhantomData }
    }
}
