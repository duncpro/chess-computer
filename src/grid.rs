use crate::misc::const_min_u8;
use crate::piece::Color;
use std::fmt::{Debug, Write};
use std::fmt::Formatter;
use std::str::FromStr;
use std::fmt::Display;
use std::num::NonZeroU8;

// # Laterals

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Rank { value: NonZeroU8 }

impl Rank {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 8);
        let value = unsafe { NonZeroU8::new_unchecked(index + 1) };
        Self { value }
    }

    pub const fn index(self) -> u8 { self.value.get() - 1 }

    pub fn base_rank(color: Color) -> Self {
        Self::from_index(color.index() * 7)
    }

    pub fn relative_to(color: Color, offset: i8) -> Self {
        let base = Self::base_rank(color).index() as i8;
        let sindex = base + color.sign() * offset;
        return Self::from_index(sindex.try_into().unwrap())
    }

    pub fn pawn_rank(color: Color) -> Self {
        return Self::relative_to(color, 1);
    }

    pub fn pdj_rank(color: Color) -> Self {
        return Self::relative_to(color, 3)
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index() + 1)
    }
}

#[derive(Debug)]
pub enum ParseRankErr {
    ParseIntErr(std::num::ParseIntError),
    OutOfBounds(/* bad value */ u8)
}

impl From<std::num::ParseIntError> for ParseRankErr {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntErr(value)
    }
}

impl FromStr for Rank {
    type Err = ParseRankErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.parse::<u8>()?;
        if (index < 1) | (index > 8) {
            return Err(ParseRankErr::OutOfBounds(index))
        }
        return Ok(Rank::from_index(index - 1));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct File { value: NonZeroU8 }

impl File {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 8);
        let value = unsafe { NonZeroU8::new_unchecked(index + 1) };
        Self { value }
    }

    pub const fn index(self) -> u8 { self.value.get() - 1 }

    pub fn letter(self) -> char { char::from(65 + self.index()) }

    pub const A: File = File::from_index(0);
    pub const B: File = File::from_index(1);
    pub const C: File = File::from_index(2);
    pub const D: File = File::from_index(3);
    pub const E: File = File::from_index(4);
    pub const F: File = File::from_index(5);
    pub const G: File = File::from_index(6);
    pub const H: File = File::from_index(7);
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(65 + self.index()))
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(65 + self.index()))
    }
}

#[derive(Debug)]
pub struct ParseFileErr;

impl FromStr for File {
    type Err = ParseFileErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(File::A),
            "B" => Ok(File::B),
            "C" => Ok(File::C),
            "D" => Ok(File::D),
            "E" => Ok(File::E),
            "F" => Ok(File::F),
            "G" => Ok(File::G),
            "H" => Ok(File::H),
            _   => Err(ParseFileErr)
        }
    }
}

// # Diagonals

#[derive(Clone, Copy)]
pub struct Prodiagonal { index: u8 }

#[derive(Clone, Copy)]
pub struct Antidiagonal { index: u8 }

impl Prodiagonal {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 15);
        Self { index }
    }

    pub const fn index(self) -> u8 { self.index }

    pub fn length(self) -> u8 { calc_diag_length(self.index()) }

    pub const fn base_file(self) -> File {
        let index = 7u8.saturating_sub(self.index());
        return File::from_index(index);
    }

    pub fn base_rank(self) -> Rank {
        let index = self.index().saturating_sub(7);
        return Rank::from_index(index);
    }

    pub fn base(self) -> StandardCoordinate {
        StandardCoordinate::new(self.base_rank(), self.base_file())
    }
}

impl Antidiagonal {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 15);
        Self { index }
    }

    pub const fn index(self) -> u8 { self.index }

    pub fn length(self) -> u8 { calc_diag_length(self.index()) }

    pub const fn base_file(self) -> File {
        let index = const_min_u8(self.index(), 7);
        return File::from_index(index);
    }

    pub fn base_rank(self) -> Rank {
        let index = self.index().saturating_sub(7);
        return Rank::from_index(index);
    }

    pub fn base(self) -> StandardCoordinate {
        StandardCoordinate::new(self.base_rank(), self.base_file())
    }
}

fn calc_diag_length(diag_index: u8) -> u8 {
    assert!(diag_index <= 14);
    let signed_diag_indx = diag_index as i8;
    let diag_len = 7 - (7i8 - signed_diag_indx).abs() + 1;
    return u8::try_from(diag_len).unwrap();
    
}

// # `StandardCoordinate`

/// The general purpose tile coordinate type, to be used almost always,
/// except in the rare case when a more specialized coordinate system
/// is convenient for the task at hand.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StandardCoordinate { index: u8 }

impl StandardCoordinate {
    pub const fn rank(self) -> Rank { Rank::from_index(self.index / 8) }
    pub const fn file(self) -> File { File::from_index(self.index % 8) }
    
    pub const fn prodiagonal(self) -> Prodiagonal {
        let index = 7 - self.file().index() + self.rank().index();
        return Prodiagonal::from_index(index);
    }
    pub const fn prodiagonal_offset(self) -> u8 {
        return self.file().index() - self.prodiagonal().base_file().index();
    }
    
    pub const fn antidiagonal(self) -> Antidiagonal {
        let index = self.rank().index() + self.file().index();
        return Antidiagonal::from_index(index);
    }
    pub const fn antidiagonal_offset(self) -> u8 {
        self.antidiagonal().base_file().index() - self.file().index()
    }
    
    pub fn index(self) -> u8 { self.index }
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 64);
        return Self { index }
    }
    
    pub fn new(rank: Rank, file: File) -> Self {
        let index = rank.index() * 8 + file.index();
        return Self::from_index(index);
    }
}

impl Display for StandardCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file().letter(), self.rank())
    }
}

impl Debug for StandardCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file().letter(), self.rank())
    }
}

#[derive(Debug)]
pub enum ParseStandardCoordinateError {
    ParseRankErr(ParseRankErr),
    ParseFileErr(ParseFileErr),
    BadLen
}

impl From<ParseRankErr> for ParseStandardCoordinateError {
    fn from(value: ParseRankErr) -> Self {
        Self::ParseRankErr(value)
    }
}

impl From<ParseFileErr> for ParseStandardCoordinateError {
    fn from(value: ParseFileErr) -> Self {
        Self::ParseFileErr(value)
    }
}

impl FromStr for StandardCoordinate {
    type Err = ParseStandardCoordinateError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 { return Err(Self::Err::BadLen) }
        let file = File::from_str(&s[0..1])?;
        let rank = Rank::from_str(&s[1..2])?;
        return Ok(Self::new(rank, file));
    }
}

// # `FileDirection`

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Side {
    Queenside = 0,
    Kingside = 1
}

impl Side {
    pub fn index(self) -> u8 { self as u8 } 
}
