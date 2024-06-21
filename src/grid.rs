#[derive(Clone, Copy)]
pub struct Rank { index: u8 }

#[derive(Clone, Copy)]
pub struct File { index: u8 }

impl Rank {
    pub fn new(index: u8) -> Self {
        assert!(index < 8);
        Self { index }
    }

    pub fn index(self) -> u8 { self.index }
}

impl File {
    pub fn new(index: u8) -> Self {
        assert!(index < 8);
        Self { index }
    }

    pub fn index(self) -> u8 { self.index }
}

/// The general purpose tile coordinate type, to be used almost always,
/// except in the rare case when a more specialized coordinate system
/// is convenient for the task at hand.
#[derive(Clone, Copy)]
pub struct StandardCoordinate { index: u8 }

impl StandardCoordinate {
    pub fn rank(self) -> Rank { Rank::new(self.index / 8) }
    pub fn file(self) -> File { File::new(self.index % 8) }
    
    pub fn index(self) -> u8 { self.index }
    pub fn from_index(index: u8) -> Self {
        assert!(index < 64);
        return Self { index }
    }
    
    pub fn new(rank: Rank, file: File) -> Self {
        let index = rank.index() * 8 + file.index();
        return Self::from_index(index);
    }
}

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
