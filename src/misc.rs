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

