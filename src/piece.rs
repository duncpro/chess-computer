use std::fmt::Display;
use std::num::NonZeroU8;

use crate::impl_enum_table;
use crate::grid::Rank;

// # `Species`

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn base_rank(self) -> Rank {
        const RANK_LUT: ColorTable<Rank> = ColorTable::new([
            Rank::from_index(0),
            Rank::from_index(7)
        ]);
        return RANK_LUT[self];
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

impl Default for Color {
    fn default() -> Self { Self::White }
}

impl_enum_table!(Color);

// `Piece`

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Piece {
    // LSB 8 7 6 5 4 3 2 1
    //  Species  -----
    //          Color  ---
    data: NonZeroU8 
}

impl Piece {
    pub fn color(self) -> Color {
        let index = (self.data.get() & 0b11) - 1;
        return Color::from_index(index);
    }
    
    pub fn species(self) -> Species {
        let index = (self.data.get() >> 2) - 1;
        return Species::from_index(index);
    }

    pub fn new(color: Color, species: Species) -> Self {
        let mut data = 0;
        data |= color.index() + 1;
        data |= (species.index() + 1) << 2;
        return Self { 
            data: NonZeroU8::new(data).unwrap()  
        }
    }
}

