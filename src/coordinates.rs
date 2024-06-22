use crate::build_itable;
use crate::cfor;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::misc::const_min_u8;
use std::marker::PhantomData;

pub trait CoordinateSystem: Sized + Copy {
    fn encode(stdc: StandardCoordinate) -> u8;
    fn decode(spec_coord: u8) -> StandardCoordinate;
    fn get_lane(stdc: StandardCoordinate) -> LaneLoc;
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
/// this generic `Coordinate` type.
/// 1. [`RankMajorCS`]
/// 2. [`FileMajorCS`]
/// 3. [`ProdiagonalMajorCS`]
/// 4. [`AntidiagonalMajorCS`]
///
/// This is a specialized coordinate type. For the general purpose
/// coordinate type see [`StandardCoordinate`]. All absolute major tile
/// coordinates are convertable to/from [`StandardCoordinate`] but not
/// necessarily to/from one another directly. 
#[derive(Clone, Copy)]
pub struct Coordinate<C: CoordinateSystem> { 
    index: u8,
    pd: PhantomData<C>
}

impl<C: CoordinateSystem> From<StandardCoordinate> for Coordinate<C> {
    fn from(value: StandardCoordinate) -> Self {
        let index = C::encode(value);
        return Self::from_index(index);
    }
}

impl<C: CoordinateSystem> From<Coordinate<C>> for StandardCoordinate {
    fn from(value: Coordinate<C>) -> Self {
        C::decode(value.index())
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

// # Lane

pub struct LaneLoc {
    pub local_origin: u8,
    pub base: StandardCoordinate,
    pub length: u8    
}


// # Lateral Coordinate Systems

/// The [`CoordinateSystem`] of [`StandardCoordinate`].
/// Unlike the other coordinate systems, conversion between 
/// `Coordinate<StandardCS>` and [`StandardCoordinate`] is
/// computationally free, since their in-memory representations
/// are indistinguishable.
///
/// Therefore, `StandardCS` should be preferred when a more specific
/// coordinate system is unnecessary. Especially if the
/// `Coordinate<StandardCS>` is to be converted into 
/// a [`StandardCoordinate`] later.
pub type StandardCS = RankMajorCS;

impl CoordinateSystem for RankMajorCS {
    fn encode(stdc: StandardCoordinate) -> u8 { stdc.index() }
    
    fn decode(coord: u8) -> StandardCoordinate { 
        StandardCoordinate::from_index(coord)
    }

    fn get_lane(stdc: StandardCoordinate) -> LaneLoc {
        LaneLoc { 
            local_origin: stdc.file().index(),
            base: StandardCoordinate::new(stdc.rank(), File::from_index(0)), 
            length: 8
        }
    }
    
    const INDEX: usize = 0;
}

impl CoordinateSystem for FileMajorCS {
    fn encode(stdc: StandardCoordinate) -> u8 {
        stdc.file().index() * 8
    }
    
    fn decode(coord: u8) -> StandardCoordinate {
        let rank = Rank::from_index(coord % 8);
        let file = File::from_index(coord / 8);
        return StandardCoordinate::new(rank, file);
    }

    fn get_lane(stdc: StandardCoordinate) -> LaneLoc {
        LaneLoc { 
            local_origin: stdc.rank().index(),
            base: StandardCoordinate::new(Rank::from_index(0), stdc.file()), 
            length: 8
        }
    }
    
    const INDEX: usize = 1;
}

// # Diagonal Coordinate Systems

impl CoordinateSystem for ProdiagonalMajorCS {
    fn encode(stdc: StandardCoordinate) -> u8 {
        calc_diag_coord(
            stdc.prodiagonal().index(), 
            stdc.prodiagonal_offset()
        )
    }

    fn decode(coord: u8) -> StandardCoordinate {
        let index = PDC_INVERSE_LUT[usize::from(coord)];
        return StandardCoordinate::from_index(index);
    }

    fn get_lane(stdc: StandardCoordinate) -> LaneLoc {
        LaneLoc { 
            local_origin: stdc.prodiagonal_offset(),
            base: stdc.prodiagonal().base(), 
            length: stdc.prodiagonal().length()
        }
    }
    
    const INDEX: usize = 2;
}

impl CoordinateSystem for AntidiagonalMajorCS {
    fn encode(stdc: StandardCoordinate) -> u8 {
        calc_diag_coord(
            stdc.antidiagonal().index(), 
            stdc.antidiagonal_offset()
        )
    }

    fn decode(coord: u8) -> StandardCoordinate {
        let index = ADC_INVERSE_LUT[usize::from(coord)];
        return StandardCoordinate::from_index(index);
    }

    fn get_lane(stdc: StandardCoordinate) -> LaneLoc {
        LaneLoc { 
            local_origin: stdc.antidiagonal_offset(),
            base: stdc.antidiagonal().base(), 
            length: stdc.antidiagonal().length()
        }
    }
    
    const INDEX: usize = 3;
}

const fn triangle_num(n: u8) -> u8 { (n.pow(2) + n) / 2 }

const fn calc_diag_coord(diag_index: u8, local_index: u8) -> u8 {
    let mut coord: u8 = 0;
    
    coord += triangle_num(const_min_u8(7, diag_index));
    coord += local_index;  

    const HALF: u8 = triangle_num(8);
    coord += HALF;  
    coord -= triangle_num(8 - diag_index.saturating_sub(7));

    return coord;   
}

build_itable!(PDC_INVERSE_LUT: [u8; 64], |table| {
    cfor!(let mut i: u8 = 0; i < 64; i += 1; {
        let stdc = StandardCoordinate::from_index(i);
        let pdc = calc_diag_coord(stdc.prodiagonal().index(), 
            stdc.prodiagonal_offset()); 
        table[pdc as usize] = i; 
    });
});

build_itable!(ADC_INVERSE_LUT: [u8; 64], |table| {
    cfor!(let mut i: u8 = 0; i < 64; i += 1; {
        let stdc = StandardCoordinate::from_index(i);
        let adc = calc_diag_coord(stdc.antidiagonal().index(), 
            stdc.antidiagonal_offset()); 
        table[adc as usize] = i; 
    });
});
