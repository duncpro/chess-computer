use crate::impl_enum_table;
use crate::impl_enum_opt_table;
use crate::grid::Rank;

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
    const COUNT: usize = 6;
}

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

impl_enum_opt_table!(Color);
impl_enum_table!(Color);
impl_enum_opt_table!(Species);
impl_enum_table!(Species);

