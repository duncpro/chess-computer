use std::fmt::Display;
use std::num::NonZeroU8;
use std::ops::{IndexMut, Index};
use crate::impl_enum_table;
use crate::grid::{Rank, StandardCoordinate};

// # `Species`

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Species {
    Pawn   = 1,
    Rook   = 2,
    Knight = 3,
    Bishop = 4,
    Queen  = 5,
    King   = 6,
}

impl Species {
    pub const COUNT: usize = 6;

    pub const fn index(self) -> u8 { self as u8 - 1 }

    pub fn from_index(index: u8) -> Self {
        unsafe {
            std::mem::transmute(index + 1)
        }
    }
}

impl_enum_table!(Species);

// # `Color`

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 1,
    Black = 2
}

impl Color {
    pub const COUNT: usize = 2;

    /// Calculates the index of the color `self`.
    ///
    /// ```text
    /// Color |  Index
    /// ----- | ------
    /// White |    0
    /// Black |    1
    /// ```
    pub const fn index(self) -> u8 { (self as u8) - 1 }

    pub fn from_index(index: u8) -> Self {
        assert!(index < 2);
        unsafe {
            std::mem::transmute::<u8, Color>(index + 1)
        }
    }

    pub fn oppo(self) -> Self {
        let index = (self.index() + 1) % 2;
        return Self::from_index(index);
    }

    /// Calculates the sign of the color `self`.
    ///
    /// ```text
    /// Color |  Sign
    /// ----- | ------
    /// White |    1
    /// Black |   -1
    /// ```
    pub fn sign(self) -> i8 { -1 * (((self.index() as i8) * 2) - 1) }

    pub fn swap(&mut self) { *self = self.oppo(); }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

impl_enum_table!(Color);

// `Piece`

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Piece { data: NonZeroU8 }

impl Piece {
    pub fn new(color: Color, species: Species) -> Self {
        let index = 6 * color.index() + species.index();
        let data  = NonZeroU8::new(index + 1).unwrap();
        return Self { data }
    }

    pub fn color(self) -> Color {
        let class_index = self.data.get() - 1;
        let color_index = class_index / 6;
        return Color::from_index(color_index);
    }

    pub fn species(self) -> Species {
        let class_index = self.data.get() - 1;
        let species_index = class_index % 6;
        return Species::from_index(species_index);
    }
    pub fn set_species(&mut self, species: Species) {
        *self = Piece::new(self.color(), species);
    }
    pub fn index(self) -> u8 { self.data.get() - 1 } 
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PieceGrid { data: [u8; 32] }

impl PieceGrid {
    pub fn set(&mut self, pos: StandardCoordinate, piece: Option<Piece>) {
        let data: u8 = unsafe { std::mem::transmute(piece) };
        let lut_index = usize::from(pos.index() / 2);
        let byte_offset = (pos.index() % 2) * 4;
        self.data[lut_index] &= !(0b1111 << byte_offset);
        self.data[lut_index] |= data << byte_offset;
    }

    pub fn get(&self, pos: StandardCoordinate) -> Option<Piece> {        
        let lut_index = usize::from(pos.index() / 2);
        let byte_offset = (pos.index() % 2) * 4;
        let data = (self.data[lut_index] >> byte_offset) & 0b1111;
        return unsafe { std::mem::transmute(data) };
    }

    pub fn empty() -> Self { Self { data: [0; 32] } }
}
