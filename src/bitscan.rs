pub fn bitscan<B: Bitstring>(bitstring: B) -> BitscanIterator<B> {
    BitscanIterator { bitstring }
}

pub struct BitscanIterator<B: Bitstring> { 
    bitstring: B,
}

impl<B: Bitstring> Iterator for BitscanIterator<B> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.bitstring.trailing_zeros();
        if index == 64 { return None } 
        else { 
            self.bitstring.clear(index);
            return Some(index);
        }
    }
}

pub trait Bitstring {
    fn trailing_zeros(&self) -> u8;
    fn clear(&mut self, bitpos: u8);
}


macro_rules! impl_bitstring {
    ($bs_type:ty) => {
        impl Bitstring for $bs_type {
            fn trailing_zeros(&self) -> u8 { 
                <$bs_type>::trailing_zeros(*self) as u8  
            }
            fn clear(&mut self, bitpos: u8) {
                *self &= !(1 << bitpos);
            }
        }
    };
}

impl_bitstring!(u8);
impl_bitstring!(u64);
