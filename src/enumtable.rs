//! This modules provides support for enum tables. n enum table is a wrapper 
//! around an array that implements [`std::ops::Index`] and [`std::ops::IndexMut`]
//! so that the table can be indexed by the enum directly instead of indirectly though
//! the discriminant.
//!
//! Warning! All enums which are used as the key in an enum table 
//! **must** conform to the following shape.
//! - The enum must be `repr(u8)`.
//! - The enum must have have its discriminants specified as consecutive
//!   numerals beginning at 1 (NOT zero!).

#[macro_export]
macro_rules! impl_enum_opt_table { 
    ($key_type:ty) => {
        ::paste::paste! {
            pub struct [<Opt $key_type Table>]<T> {
                array: [T; <$key_type>::COUNT + 1]
            }

            impl<T> Default for [<Opt $key_type Table>]<T> where T: Default + Copy {
                fn default() -> Self {
                    Self { array: [T::default(); <$key_type>::COUNT + 1]  }
                }
            }

            impl<T> std::ops::Index<$key_type> for [<Opt $key_type Table>]<T> {
                type Output = T;
                fn index(&self, key: $key_type) -> &Self::Output {
                    &self.array[usize::from(key as u8)]
                }
            }

            impl<T> std::ops::IndexMut<$key_type> for [<Opt $key_type Table>]<T> {
                fn index_mut(&mut self, key: $key_type) -> &mut Self::Output {
                    &mut self.array[usize::from(key as u8)]
                }
            }

            impl<T> std::ops::Index<Option<$key_type>> for [<Opt $key_type Table>]<T> {
                type Output = T;
                fn index(&self, key: Option<$key_type>) -> &Self::Output {
                    let index: u8 = unsafe { std::mem::transmute(key) };
                    &self.array[usize::from(index)]
                }
            }

            impl<T> std::ops::IndexMut<Option<$key_type>> for[<Opt $key_type Table>]<T> {
                fn index_mut(&mut self, key: Option<$key_type>) -> &mut Self::Output {
                    let index: u8 = unsafe { std::mem::transmute(key) };
                    &mut self.array[usize::from(index)]
                }
            }

            impl<T> [<Opt $key_type Table>]<T> {
                pub const fn new(array: [T; <$key_type>::COUNT + 1]) -> Self {
                    Self { array }
                }
            }

            #[cfg(test)]
            mod [<opt _ $key_type:lower _ table _ test >] {
                use super::*;
                
                #[test]
                fn none_is_zero() {
                    unsafe {
                        let num = std::mem::transmute::<Option<$key_type>, u8>(None);
                        assert_eq!(num, 0, "Expected Option::None to be represented \
                            by 0, but encountered another numeral instead. \
                            This likely indicates the the discrimants were not \
                            explicitly specified, or were specified incorrectly.");
                    } 
                }
                
                #[test]
                fn const_count_correct() {
                    let vc = std::mem::variant_count::<$key_type>();
                    assert_eq!(vc, $key_type::COUNT, "Expected T::COUNT \
                        to match std::mem::variant_count but it did not. \
                        The array might be too large or too small then.");
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_table { 
    ($key_type:ty) => {
        ::paste::paste! {
            pub struct [<$key_type Table>]<T> {
                array: [T; <$key_type>::COUNT]
            }

            impl<T> Default for [<$key_type Table>]<T> where T: Default + Copy {
                fn default() -> Self {
                    Self { array: [T::default(); <$key_type>::COUNT]  }
                }
            }

            impl<T> std::ops::Index<$key_type> for [<$key_type Table>]<T> {
                type Output = T;
                fn index(&self, key: $key_type) -> &Self::Output {
                    &self.array[usize::from(key as u8)]
                }
            }

            impl<T> std::ops::IndexMut<$key_type> for [<$key_type Table>]<T> {
                fn index_mut(&mut self, key: $key_type) -> &mut Self::Output {
                    &mut self.array[usize::from(key as u8)]
                }
            }

            impl<T> [<$key_type Table>]<T> {
                pub const fn new(array: [T; <$key_type>::COUNT]) -> Self {
                    Self { array }
                }
            }

            #[cfg(test)]
            mod [<$key_type:lower _ table _ test >] {
                use super::*;
                                
                #[test]
                fn const_count_correct() {
                    let vc = std::mem::variant_count::<$key_type>();
                    assert_eq!(vc, $key_type::COUNT, "Expected T::COUNT \
                        to match std::mem::variant_count but it did not. \
                        The array might be too large or too small then.");
                }
            }
        }
    };
}

