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

// # Utility Functions

pub const fn const_min_u8(left: u8, right: u8) -> u8 {
    if left < right { left } else { right }
}

pub const fn pick<T>(condition: bool, left: T, right: T) -> T 
where T: Copy
{
    let lut: [T; 2] = [left, right];
    return lut[condition as usize];
}

/// This macro provides support for enum tables. An enum table is a wrapper 
/// around an array that implements [`std::ops::Index`] and [`std::ops::IndexMut`]
/// so that the table can be indexed by the enum directly instead of indirectly though
/// the discriminant.
///
/// Warning! All enums which are used as the key in an enum table 
/// **must** conform to the following shape.
/// - The enum must have a `const COUNT: usize` member.
/// - The enum must provide an `index` method which converts
///   the value to a zero-based index.
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
                    &self.array[usize::from(key.index())]
                }
            }

            impl<T> std::ops::IndexMut<$key_type> for [<$key_type Table>]<T> {
                fn index_mut(&mut self, key: $key_type) -> &mut Self::Output {
                    &mut self.array[usize::from(key.index())]
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

