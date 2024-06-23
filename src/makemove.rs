use crate::bitboard::RawBitboard;
use crate::crights::update_crights;
use crate::gamestate::LoggedMove;
use crate::gamestate::LoggedPieceMove;
use crate::gamestate::MovelogEntry;
use crate::grid::File;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::misc::OptionPieceColor;
use crate::misc::OptionPieceSpecies;
use crate::misc::PieceColor;
use crate::misc::PieceSpecies;
use crate::misc::select;
use crate::movegen::moveset::MSPieceMove;
use crate::gamestate::GameState;
use crate::gamestate::PieceMoveKind::Promote;
use crate::rmrel::relativize;
use crate::setbit;
use crate::unsetbit;



fn clear_tile(state: &mut GameState, pos: StandardCoordinate) {    
    let species = state.species_lut[pos];
    let affilia = state.affilia_lut[pos];

    state.species_lut[pos] = OptionPieceSpecies::None;
    state.affilia_lut[pos] = OptionPieceColor::None;
    
    state.bbs.affilia_bbs[affilia].unset(pos);
    state.bbs.species_bbs[species].unset(pos);

    unsetbit!(state.bbs.pawn_rel_bb, 
        relativize(pos, state.active_player()));
    unsetbit!(state.bbs.affilia_rel_bbs[affilia],
        relativize(pos, state.active_player()));
}

fn fill_tile(state: &mut GameState, pos: StandardCoordinate,
    color: PieceColor, species: OptionPieceSpecies) 
{
    state.species_lut[pos] = species;
    state.affilia_lut[pos] = color.into();

    state.bbs.affilia_bbs[color].set(pos);
    state.bbs.species_bbs[species].set(pos);
    
    let rel_pos = relativize(pos, state.active_player());
    setbit!(state.bbs.affilia_rel_bbs[color], rel_pos);
    let is_pawn = species == OptionPieceSpecies::Pawn;
    state.bbs.pawn_rel_bb |= ((1 << rel_pos) * (is_pawn as RawBitboard));
}

fn make_pmove(state: &mut GameState, pmove: MSPieceMove) {
    let beg_species = state.species_lut[pmove.origin];
    let affilia = state.affilia_lut[pmove.origin];
    let capture = state.species_lut[pmove.target];

    clear_tile(state, pmove.origin);
    clear_tile(state, pmove.target);

    let end_species = select(pmove.kind == Promote, pmove.promote, beg_species);
    fill_tile(state, pmove.destin, state.active_player(), end_species);


    let prev_crights = state.crights;
    update_crights(state); 

    state.movelog.push(MovelogEntry {
        prev_crights,
        lmove: LoggedMove::Piece(LoggedPieceMove {
            origin: pmove.origin,
            destin: pmove.destin,
            target: pmove.target,
            kind: pmove.kind,
            capture,
        })
    })
}

fn make_castle(state: &mut GameState, side: FileDirection) {
    const ROOK_ORIGIN_LUT: [File; 2] = [
        /* Queenside */ File::A,
        /* Kingside  */ File::H
    ];
    let rook_origin = StandardCoordinate::new(
        state.active_player().base_rank(),
        ROOK_ORIGIN_LUT[usize::from(side.index())]
    );

    const ROOK_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::D,
        /* Kingside  */ File::G
    ];
    let rook_destin = StandardCoordinate::new(
        state.active_player().base_rank(),
        ROOK_DESTIN_LUT[usize::from(side.index())]
    );

    const KING_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::C,
        /* Kingside  */ File::G
    ];
    let king_destin = StandardCoordinate::new(
        state.active_player().base_rank(), 
        KING_DESTIN_LUT[usize::from(side.index())]
    );
    
    let king_origin = StandardCoordinate::new(
        state.active_player().base_rank(), File::E);

    clear_tile(state, king_origin);
    clear_tile(state, rook_origin);
    fill_tile(state, rook_destin, state.active_player(),
        OptionPieceSpecies::Rook);
    fill_tile(state, king_destin, state.active_player(), 
        OptionPieceSpecies::King);

    let prev_crights = state.crights;
    state.crights.revoke(state.active_player());

    state.movelog.push(MovelogEntry { 
        prev_crights,
        lmove: LoggedMove::Castle(side)
    });
}
