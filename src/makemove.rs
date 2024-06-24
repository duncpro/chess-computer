use crate::bitboard::RawBitboard;
use crate::bits::swap_bytes_inplace_u64;
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
    color: OptionPieceColor, species: OptionPieceSpecies) 
{
    state.species_lut[pos] = species;
    state.affilia_lut[pos] = color;

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
    fill_tile(state, pmove.destin, state.active_player().into(), 
        end_species);


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
    fill_tile(state, rook_destin, state.active_player().into(),
        OptionPieceSpecies::Rook);
    fill_tile(state, king_destin, state.active_player().into(), 
        OptionPieceSpecies::King);

    let prev_crights = state.crights;
    state.crights.revoke(state.active_player());

    state.movelog.push(MovelogEntry { 
        prev_crights,
        lmove: LoggedMove::Castle(side)
    });
}

fn swap_active(state: &mut GameState) {
    swap_bytes_inplace_u64(&mut state.bbs.pawn_rel_bb);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[PieceColor::White]);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[PieceColor::Black]);
    state.bbs.active_player = state.bbs.active_player.oppo();
}

fn unmake_pmove(state: &mut GameState, pmove: LoggedPieceMove) {
    clear_tile(state, pmove.destin);
    clear_tile(state, pmove.target);

    // Undo capture
    {
        let was_capture = (pmove.capture != OptionPieceSpecies::None);
        let capture_affil = select(was_capture, 
            state.active_player().oppo().into(), OptionPieceColor::None);
        // TODO: Get rid of this lookup table easily.
        // Create a `Capture` struct which holds an OptionPieceColor,
        // and an OptionPieceSpecies, compressed of course, and store
        // that in the LoggedMove instead of only the kind.
        fill_tile(state, pmove.target, capture_affil, pmove.capture);
    }
}

fn unmake_castle(state: &mut GameState, side: FileDirection) {
    
}

fn unmake_move(state: &mut GameState) {
    let last_entry = state.movelog.pop().unwrap();
    state.crights = last_entry.prev_crights;

    swap_active(state);

    match last_entry.lmove {
        LoggedMove::Castle(side) => unmake_castle(state, side),
        LoggedMove::Piece(pmove) => unmake_pmove(state, pmove),
    }
}
