use crate::build_itable;
use crate::bitboard::Bitlane;
use crate::cfor;
use crate::setbit;
use crate::getbit;

// # Slidescsan

const SLIDE_LUT_LEN: usize = 8 /* origins */ * 2usize.pow(8) /* occupancies */;

fn calc_lut_key(origin: u8, occupancy: Bitlane) -> usize {
    assert!(origin < 8);
    
    return
        usize::from(origin) * 2usize.pow(8) /* occupancies */ 
        + usize::from(occupancy);
}

build_itable!(SLIDESCAN_LUT: [u8; SLIDE_LUT_LEN], |table| {
    let mut lut_indx: usize = 0;
    cfor!(let mut origin: u8 = 0; origin < 8; origin += 1; {
        cfor!(let mut occ: Bitlane = 0; occ <= Bitlane::MAX; occ += 1; {
            table[lut_indx] = slidescan(origin, occ);
            lut_indx += 1;    
            if occ == u8::MAX { break; }        
        });
    });
});

const fn slidescan(origin: u8, occ: Bitlane) -> Bitlane /* destins */ {
    let mut destins: Bitlane = 0;

    // Slide towards least significant bit
    if origin > 0 {
        let mut i = origin - 1;
        loop {
            setbit!(destins, i);
            if i == 0 { break; }
            if getbit!(occ, i) { break; }
            i -= 1;
        }
    } 

    // Slide towards most significant bit
    if origin < 7 {
        cfor!(let mut i = origin + 1; i <= 7; i += 1; {
            setbit!(destins, i);
            if getbit!(occ, i) { break; }
        });
    }

    return destins;
}

pub fn get_slidescan(origin: u8, occ: Bitlane) -> Bitlane {
    let indx = calc_lut_key(origin, occ);
    return SLIDESCAN_LUT[indx];
}

// # Slidelimit

build_itable!(SLIDELIMIT_LUT: [u8; SLIDE_LUT_LEN], |table| {
    let mut lut_indx: usize = 0;
    cfor!(let mut origin: u8 = 0; origin < 8; origin += 1; {
        cfor!(let mut occ: Bitlane = 0; occ <= Bitlane::MAX; occ += 1; {
            table[lut_indx] = slidelimit(origin, occ);
            lut_indx += 1;    
            if occ == u8::MAX { break; }        
        });
    });
});

pub const fn slidelimit(origin: u8, occ: Bitlane) -> Bitlane {
    let mut limits: Bitlane = 0;

    // Slide towards least significant bit
    if origin > 0 {
        let mut i = origin - 1;
        loop {
            if i == 0 { break; }
            if getbit!(occ, i) { break; }
            i -= 1;
        }
        setbit!(limits, i);
    } 

    // Slide towards most significant bit
    if origin < 7 {
        cfor!(let mut i = origin + 1; i <= 7; i += 1; {
            setbit!(limits, i);
            if getbit!(occ, i) { break; }
        });
    }

    if origin < 7 {
        let mut i = origin + 1;
        loop {
            if i == 7 { break; }
            if getbit!(occ, i) { break; }
            i += 1;
        }
        setbit!(limits, i);
    } 

    return limits;
}

pub fn get_slidelimit(origin: u8, occ: Bitlane) -> Bitlane {
    let key = calc_lut_key(origin, occ);
    return SLIDELIMIT_LUT[key];
}
