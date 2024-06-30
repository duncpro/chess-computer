use std::cell::Ref;
use std::marker::PhantomData;
use std::cell::RefMut;
use std::cell::RefCell;

#[macro_export]
macro_rules! expect_match {
    ($value:expr, $p:pat) => {
        let $p = $value else { panic!("bad match") };
    }
}

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

pub const fn pick<T>(condition: bool, if_true: T, if_false: T) -> T
where T: Copy
{
    let lut: [T; 2] = [if_false, if_true];
    return lut[condition as usize];
}

pub fn max_inplace<T>(left: &mut T, right: T) 
where T: Ord + Copy
{
    *left = std::cmp::max(*left, right);
}

// # Max

pub struct Max<T, V: Ord + Copy> { obj: Option<T>, value: V }

impl<T, V: Ord + Copy> Max<T, V> {
    pub fn new(min: V) -> Self {
        Self { obj: None, value: min }
    }
    
    pub fn push(&mut self, obj: T, value: V) {
        if value >= self.value {
            self.obj = Some(obj);
            self.value = value;
        }
    }

    pub fn take(self) -> Option<T> { self.obj }
    pub fn value(&self) -> V { self.value }
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


// # `Push`

pub trait Push<T> {
    fn push(&mut self, value: T);
}

pub struct PushCount<T> { 
    count: usize,
    pd: PhantomData<T>
}

impl<T> Push<T> for PushCount<T> {
    fn push(&mut self, value: T) { 
        self.count += 1; 
    }
}

impl<T> PushCount<T> {
    pub fn count(&self) -> usize { self.count }
    pub fn new() -> Self { 
        Self { count: 0, pd: PhantomData } 
    }
}

pub struct PushFilter<T, F, P> 
where P: Push<T>, F: FnMut(&T) -> bool
{
    filter_fn: F,
    inner: P,
    pd: PhantomData<T>
}

impl<T, F, P> Push<T> for PushFilter<T, F, P>
where P: Push<T>, F: FnMut(&T) -> bool
{
    fn push(&mut self, value: T) {
        let pass = (self.filter_fn)(&value);
        if pass { self.inner.push(value); }
    }
}


impl<T, F, P> PushFilter<T, F, P>
where P: Push<T>, F: FnMut(&T) -> bool
{
    pub fn inner(&self) -> &P { &self.inner }
    pub fn new(inner: P, filter_fn: F) -> Self {
        Self { filter_fn, inner, pd: PhantomData }
    }
}

// # `SegVec`

pub struct SegVec<'a, T> 
{
    vec_cell: &'a RefCell<Vec<T>>,
    begin: usize
}

impl<'a, T> SegVec<'a, T> {
    pub fn extend<'b, 'c>(&'b mut self) -> SegVec<'c, T> 
    where 'a: 'b, 'b: 'c 
    {
        let begin = self.vec_cell.borrow().len();
        SegVec { vec_cell: self.vec_cell, begin }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where F: FnMut(&T) -> bool
    {
        let mut vec = self.vec_cell.borrow_mut();
        for i in (self.begin..vec.len()).rev() {
            let retained = f(&vec[i]);
            if !retained { vec.remove(i); }
        }
    }

    pub fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where F: FnMut(&T) -> K, K: Ord
    {
        self.as_mut_slice().sort_unstable_by_key(f);
    }
    
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where F: FnMut(&T) -> K, K: Ord
    {
        self.as_mut_slice().sort_by_key(f);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut vec = self.vec_cell.borrow_mut();
        if self.begin < vec.len() { return vec.pop(); }
        return None;
    }

    pub fn new(vec: &'a mut RefCell<Vec<T>>) -> Self {
        let begin = vec.borrow().len();
        Self { vec_cell: vec, begin }
    }    

    fn as_slice<'b, 'c>(&'b self) -> Ref<'c, [T]>
    where 'a: 'b, 'b: 'c
    {
        let begin = self.begin;
        Ref::map(self.vec_cell.borrow(), |r| &r.as_slice()[begin..])
    }
    
    fn as_mut_slice<'b, 'c>(&'b self) -> RefMut<'c, [T]>
    where 'a: 'b, 'b: 'c
    {
        let begin = self.begin;
        RefMut::map(self.vec_cell.borrow_mut(), 
            |r| &mut r.as_mut_slice()[begin..])
    }

    pub fn is_empty(&self) -> bool { self.as_slice().is_empty() }
}

impl<'a, T> Drop for SegVec<'a, T> {
    fn drop(&mut self) {
        self.vec_cell.borrow_mut().truncate(self.begin);
    }
}

impl<'a, T> Push<T> for SegVec<'a, T> {
    fn push(&mut self, value: T) {
        self.vec_cell.borrow_mut().push(value);
    }
}
