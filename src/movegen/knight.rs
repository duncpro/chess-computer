use crate::bitboard::Bitboard;
use crate::bitboard::RawBitboard;
use crate::build_itable;
use crate::cfor;
use crate::coordinates::Coordinate;
use crate::coordinates::RankMajorCS;
use crate::gamestate::FastPosition;
use crate::grid::StandardCoordinate;
use crate::misc::Push;
use crate::piece::Species;
use crate::setbit;
use crate::movegen::types::PMGContext;
use crate::movegen::types::PMGMove;

pub fn movegen_knights(ctx: &mut PMGContext<impl Push<PMGMove>>) {
    let knights = ctx.inspect(|s| s.bbs.class::<RankMajorCS>(
        s.active_player(), Species::Knight));
        
    for origin in knights.scan() {
        let mut destins = knight_attack(origin);
        destins &= !ctx.inspect(|s| s.bbs.affilia_bbs[s.active_player()].get());
        for destin in destins.scan() {
            ctx.push(PMGMove::new(origin.into(), destin.into()));
        }
    }
}

pub fn knight_attack(origin: Coordinate<RankMajorCS>) 
-> Bitboard<RankMajorCS> 
{
    let lut_key = usize::from(origin.index());
    return Bitboard::from_raw(KNIGHT_LUT[lut_key])
}

build_itable!(KNIGHT_LUT: [RawBitboard; 64], |table| {
    let mut i: u8 = 0;
    cfor!(let mut orank: u8 = 0; orank < 8; orank += 1; {
        cfor!(let mut ofile: u8 = 0; ofile < 8; ofile += 1; {
            let mut bb: RawBitboard = 0;

            // Each of the following blocks corresponds to an
            // ordered pair `(long, short)`.

            // 1. (up, queenside)
            if orank < 6 {
                if ofile > 0 {
                    setbit!(bb, i + 16 /* two ranks */ - 1 /* one file */);
                }
            }

            // 2. (up, kingside)
            if orank < 6 {
                if ofile < 7 {
                    setbit!(bb, i + 16 /* two ranks */ + 1 /* one file */);
                }
            }

            // 3. (queenside, up)
            if ofile > 1 {
                if orank < 7 {
                    setbit!(bb, i + 8 /* one rank */ - 2 /* two files */ );
                }
            }

            // 4. (queenside, down)
            if ofile > 1 {
                if orank > 0 {
                    setbit!(bb, i - 8 /* one rank */ - 2 /* two files */);
                }
            }

            // 5. (kingside, up)
            if ofile < 6 {
                if orank < 7 {
                    setbit!(bb, i + 8 /* one rank */ + 2 /* two files */);
                }
            }

            // 6. (kingside, down)
            if ofile < 6 {
                if orank > 0 {
                    setbit!(bb, i - 8 /* one rank */ + 2 /* two files */);
                }
            }

            // 7. (down, queenside)
            if orank > 1 {
                if ofile > 0 {
                    setbit!(bb, i - 16 /* two ranks */ - 1 /* one file */);
                }
            }

            // 8. (down, kingside)
            if orank > 1 {
                if ofile < 7 {
                    setbit!(bb, i - 16 /* two ranks */ + 1 /* one file */);
                }
            }
                
            table[i as usize] = bb;
            i += 1;
        });
    });
});

