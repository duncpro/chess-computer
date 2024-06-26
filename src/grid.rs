use std::fmt::{Debug, Display, Formatter, Write};
use crate::misc::const_min_u8;

// # Laterals

#[derive(Clone, Copy)]
pub struct Rank { index: u8 }

#[derive(Clone, Copy)]
pub struct File { index: u8 }

impl Rank {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 8);
        Self { index }
    }

    pub const fn index(self) -> u8 { self.index }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index() + 1)
    }
}

impl File {
    pub const fn from_index(index: u8) -> Self {
        assert!(index < 8);
        Self { index }
    }

    pub const fn index(self) -> u8 { self.index }

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
        write!(f, "{}{}", self.file(), self.rank())
    }
}

// # `GridTable`

pub struct GridTable<T> { array: [T; 64] }

impl<T> std::ops::Index<StandardCoordinate> for GridTable<T> {
    type Output = T;

    fn index(&self, coord: StandardCoordinate) -> &Self::Output {
        &self.array[usize::from(coord.index())]
    }
}

impl<T> std::ops::IndexMut<StandardCoordinate> for GridTable<T> {
    fn index_mut(&mut self, coord: StandardCoordinate) -> &mut Self::Output {
        &mut self.array[usize::from(coord.index())]
    }
}

impl<T: Default + Copy> Default for GridTable<T> {
    fn default() -> Self {
        Self { array: [T::default(); 64]  }    
    }
}

// # `FileDirection`

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FileDirection {
    Queenside = 0,
    Kingside = 1
}

impl FileDirection {
    pub fn index(self) -> u8 { self as u8 } 
}
