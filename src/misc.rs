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
