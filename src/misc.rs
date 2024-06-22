// This section provides enums for classifying pieces and tiles.
// Specifically, enums for classifying pieces and tiles by species
// and affiliation. For each classifier there are two enums
// a piece enum, and a tile enum. The difference being the former
// does not include a `None` variant while the latter does, since
// tiles can be unoccupied.
// 
//     *Classifier*  | Piece Enum     | Tile Enum
//     -------------------------------------------
//     *Species*     | `PieceSpecies` | `TileSpecies` 
//     --------------------------------------------
//     *Affiliation* | `PieceColor`   | `TileAffiliation`
//

// Piece Enums

#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum PieceSpecies {
    Pawn   = 0,
    Rook   = 1,
    Knight = 2,
    Bishop = 3,
    Queen  = 4,
    King   = 5,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

impl PieceColor {
    pub const COUNT: usize = 2;

    pub const fn index(self) -> u8 { self as u8 }

    pub fn from_index(index: u8) -> Self {
        assert!(index < 2);
        unsafe {
            std::mem::transmute::<u8, PieceColor>(index)
        }
    }

    pub fn oppo(self) -> Self {
        let index = (self.index() + 1) % 2;
        return Self::from_index(index);
    }
}

// Tile Enums

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TileSpecies {
    Pawn   = 0,
    Rook   = 1,
    Knight = 2,
    Bishop = 3,
    Queen  = 4,
    King   = 5,
    None   = 6
}

impl TileSpecies {
    pub const COUNT: usize = 7;
}

impl From<PieceSpecies> for TileSpecies {
    fn from(value: PieceSpecies) -> Self {
        unsafe {
            std::mem::transmute::<PieceSpecies, Self>(value)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum TileAffiliation { 
    White = 0,
    Black = 1,
    None = 2 
}

impl TileAffiliation {
    pub const COUNT: usize = 3;
}

impl From<PieceColor> for TileAffiliation {
    fn from(value: PieceColor) -> Self {
        unsafe {
            std::mem::transmute::<PieceColor, Self>(value)
        }
    }
}

macro_rules! impl_classifier_table_type {
    ($table_type_name:ident, $classifier_type:ty) => {

        pub struct $table_type_name<T> {
            array: [T; <$classifier_type>::COUNT]
        }

        impl<T> Default for $table_type_name<T> where T: Default + Copy {
            fn default() -> Self {
                Self { array: [T::default(); <$classifier_type>::COUNT]  }
            }
        }

        impl<T> std::ops::Index<$classifier_type> for $table_type_name<T> {
            type Output = T;
            fn index(&self, classifier: $classifier_type) -> &Self::Output {
                &self.array[usize::from(classifier as u8)]
            }
        }

        impl<T> std::ops::IndexMut<$classifier_type> for $table_type_name<T> {
            fn index_mut(&mut self, classifier: $classifier_type) -> &mut Self::Output {
                &mut self.array[usize::from(classifier as u8)]
            }
        }

        impl<T> $table_type_name<T> {
            pub const fn new(array: [T; <$classifier_type>::COUNT]) -> Self {
                Self { array }
            }
        }
    };
}

impl_classifier_table_type!(SpeciesTable, TileSpecies);
impl_classifier_table_type!(AffiliationTable, TileAffiliation);
impl_classifier_table_type!(ColorTable, PieceColor);

impl<T> std::ops::Index<PieceColor> for AffiliationTable<T> {
    type Output = T;
    fn index(&self, index: PieceColor) -> &Self::Output {
        &self[TileAffiliation::from(index)]
    }
}

impl<T> std::ops::IndexMut<PieceColor> for AffiliationTable<T> {
    fn index_mut(&mut self, index: PieceColor) -> &mut Self::Output {
        &mut self[TileAffiliation::from(index)]
    }
}

// # Constant Evaluation Utilities

/// A C-style for-loop, usable in `const` contexts.
#[macro_export]
macro_rules! cfor {
    ($init:stmt; $condition:expr; $next:stmt; $do:block) => {
        $init
        while $condition {
            $do;
            $next
        }
    }
}

/// Declare a `const` value and then initialize it at the declaration site as opposed to in
/// a separate `const fn`. 
#[macro_export]
macro_rules! build_const {
    ($id:ident: $t:ty, $init:expr, |$bid:ident| $fill:block) => {
        #[allow(non_snake_case)]
        mod $id {
            use super::*;
            const fn build() -> $t { 
                let mut $bid: $t = $init;
                $fill;
                return $bid;
            } 
            pub const VALUE: $t = build();
        }
        pub use self::$id::VALUE as $id;
    };
}

/// Declare a `const` integer lookup table and then initialize at the declaration
/// site as opposed to in a separate `const fn`.
#[macro_export]
macro_rules! build_itable {
    ($id:ident: [$t:ty; $size:expr], |$bid:ident| $fill:block) => {
        crate::build_const!($id: [$t; $size], [0; $size], |$bid| $fill);
    };
}

pub const fn const_min_u8(left: u8, right: u8) -> u8 {
    if left < right { left } else { right }
}
