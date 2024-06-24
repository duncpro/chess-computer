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

#[derive(Clone, Copy)]
pub struct OptionPiece { data: u8 }

impl OptionPiece {
    // Accessors
    
    fn color(self) -> Option<Color> {
        let data = self.data & 0b11;
        unsafe { std::mem::transmute(data) }
    }

    fn species(self) -> Option<Species> {
        let data = self.data >> 2;
        unsafe { std::mem::transmute(data) }
    }

    fn new(color: Option<Color>, species: Option<Species>) -> Self {
        assert!(color.is_some() == species.is_some());
        let mut data: u8 = unsafe { std::mem::transmute(color) };
        data |= (unsafe { std::mem::transmute::<_, u8>(species) } << 2);
        return Self { data }
    }
}

impl_enum_opt_table!(Color);
impl_enum_table!(Color);
impl_enum_opt_table!(Species);
impl_enum_table!(Species);

