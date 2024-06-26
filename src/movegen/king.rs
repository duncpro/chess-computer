use crate::bitboard::Bitboard;
use crate::bitboard::RawBitboard;
use crate::build_itable;
use crate::cfor;
use crate::coordinates::{Coordinate, StandardCS};
use crate::coordinates::RankMajorCS;
use crate::gamestate::FastPosition;
use crate::gamestate::locate_king;
use crate::grid::StandardCoordinate;
use crate::misc::Push;
use crate::movegen::types::PMGMove;
use crate::movegen::types::PMGContext;
use crate::setbit;
use crate::cli::print_board;
use crate::piece::Species;

pub fn movegen_king(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    let origin = ctx.inspect(|s| locate_king::<RankMajorCS>(&s.bbs));
    
    let mut bb = king_attack(origin);
    bb &= !ctx.inspect(|s| s.bbs.affilia_bbs[s.active_player()].get());
    for destin in bb.scan() {
        ctx.push(PMGMove::new(origin.into(), destin.into()));
    }
}

pub fn king_attack(origin: Coordinate<RankMajorCS>) -> Bitboard<RankMajorCS>
{
    let lut_key = usize::from(origin.index());
    return Bitboard::from_raw(KING_LUT[lut_key]);
}

build_itable!(KING_LUT: [RawBitboard; 64], |table| {
    let mut o_coord: u8 = 0;
    cfor!(let mut o_rank: u8 = 0; o_rank < 8; o_rank += 1; {
        cfor!(let mut o_file: u8 = 0; o_file < 8; o_file += 1; {
            let mut bb: RawBitboard = 0;

            // * * *
            // * O *
            // D * *
            if o_file > 0 && o_rank > 0 {
                setbit!(bb, o_coord - 9);
            }

            // * * *
            // * O *
            // * * D
            if o_file < 7 && o_rank > 0 {
                setbit!(bb, o_coord - 7);
            }

            // * * D
            // * O *
            // * * *
            if o_file < 7 && o_rank < 7 {
                setbit!(bb, o_coord + 9);
            }

            // D * *
            // * O *
            // * * *
            if o_file > 0 && o_rank < 7 {
                setbit!(bb, o_coord + 7);
            }

            // * * *
            // D O *
            // * * *
            if o_file > 0 {
                setbit!(bb, o_coord - 1);
            }

            // * * *
            // * O D
            // * * *
            if o_file < 7 {
                setbit!(bb, o_coord + 1);
            }

            // * * *
            // * O *
            // * D *
            if o_rank > 0 {
                setbit!(bb, o_coord - 8);
            }

            // * D *
            // * O *
            // * * *
            if o_rank < 7 {
                setbit!(bb, o_coord + 8);
            }

            table[o_coord as usize] = bb;
            o_coord += 1;
        });
    });  
});

