use std::marker::PhantomData;
use crate::grid::{StandardCoordinate, Rank, File};

pub trait CoordinateSystem: Sized + Copy {
    fn from_stdc(stdc: StandardCoordinate) -> u8;
    fn into_stdc(coord: u8) -> StandardCoordinate;
    const INDEX: usize;
}

#[derive(Copy, Clone)] pub struct RankMajorCS;
#[derive(Copy, Clone)] pub struct FileMajorCS;
#[derive(Copy, Clone)] pub struct ProdiagonalMajorCS;
#[derive(Copy, Clone)] pub struct AntidiagonalMajorCS;

/// Represents an *absolute* tile coordinate. 
///
/// There are four distinct absolute major coordinate systems.
/// A coordinate in any of these systems is represented via
/// this generic `AMCoordinate` type.
/// 1. [`RankMajorCS`]
/// 2. [`FileMajorCS`]
/// 3. [`ProdiagonalMajorCS`]
/// 4. [`AntidiagonalMajorCS`]
///
/// This is a specialized coordinate type. For the general purpose
/// coordinate type see [`StandardCoordinate`]. All absolute major tile
/// coordinates are convertable to/from [`StandardCoordinate`] but not
/// necessarily to/from one another. 
pub struct Coordinate<C: CoordinateSystem> { 
    index: u8,
    pd: PhantomData<C>
}

impl<C: CoordinateSystem> From<StandardCoordinate> for Coordinate<C> {
    fn from(value: StandardCoordinate) -> Self {
        let index = C::from_stdc(value);
        return Self::from_index(index);
    }
}

impl<C: CoordinateSystem> From<Coordinate<C>> for StandardCoordinate {
    fn from(value: Coordinate<C>) -> Self {
        C::into_stdc(value.index())
    }
}

impl<C: CoordinateSystem> Coordinate<C> {
    pub fn from_index(index: u8) -> Self {
        assert!(index < 64);
        Self { 
            index, 
            pd: PhantomData::default()
        }
    }
    pub fn index(self) -> u8 { self.index }
}

impl CoordinateSystem for RankMajorCS {
    fn from_stdc(stdc: StandardCoordinate) -> u8 { stdc.index() }
    fn into_stdc(coord: u8) -> StandardCoordinate { 
        StandardCoordinate::from_index(coord)
    }
    const INDEX: usize = 0;
}

impl CoordinateSystem for FileMajorCS {
    fn from_stdc(stdc: StandardCoordinate) -> u8 {
        stdc.file().index() * 8
    }
    fn into_stdc(coord: u8) -> StandardCoordinate {
        let rank = Rank::new(coord % 8);
        let file = File::new(coord / 8);
        return StandardCoordinate::new(rank, file);
    }
    const INDEX: usize = 1;
}

impl CoordinateSystem for ProdiagonalMajorCS {
    fn from_stdc(stdc: StandardCoordinate) -> u8 {
        todo!()
    }

    fn into_stdc(coord: u8) -> StandardCoordinate {
        todo!()
    }
    const INDEX: usize = 2;
}

impl CoordinateSystem for AntidiagonalMajorCS {
    fn from_stdc(stdc: StandardCoordinate) -> u8 {
        todo!()
    }

    fn into_stdc(coord: u8) -> StandardCoordinate {
        todo!()
    }
    const INDEX: usize = 3;
}
