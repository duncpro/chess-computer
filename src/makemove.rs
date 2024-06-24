use crate::bitboard::RawBitboard;
use crate::bits::swap_bytes_inplace_u64;
use crate::check::is_check;
use crate::crights::update_crights;
use crate::gamestate::LoggedMove;
use crate::gamestate::LoggedPieceMove;
use crate::gamestate::MovelogEntry;
use crate::gamestate::SpecialPieceMove;
use crate::gamestate::locate_king_stdc;
use crate::grid::File;
use crate::grid::FileDirection;
use crate::grid::StandardCoordinate;
use crate::piece::Color;
use crate::piece::Piece;
use crate::piece::Species;
use crate::misc::pick;
use crate::movegen::moveset::MGPieceMove;
use crate::gamestate::GameState;
use crate::rmrel::relativize;
use crate::setbit;
use crate::unsetbit;

// # Utilities

fn clear_tile(state: &mut GameState, pos: StandardCoordinate) {    
    let occupant = state.occupant_lut[pos];
    state.occupant_lut[pos] = None;
    if let Some(piece) = occupant {
        state.bbs.affilia_bbs[piece.color()].unset(pos);
        state.bbs.species_bbs[piece.species()].unset(pos);
        unsetbit!(state.bbs.affilia_rel_bbs[piece.color()],
            relativize(pos, state.active_player()));
    }
    unsetbit!(state.bbs.pawn_rel_bb, 
        relativize(pos, state.active_player()));
}

fn fill_tile(state: &mut GameState, pos: StandardCoordinate, piece: Piece) 
{
    state.occupant_lut[pos] = Some(piece);
    state.bbs.affilia_bbs[piece.color()].set(pos);
    state.bbs.species_bbs[piece.species()].set(pos);
    
    let rel_pos = relativize(pos, state.active_player());
    setbit!(state.bbs.affilia_rel_bbs[piece.color()], rel_pos);
    let is_pawn = piece.species() == Species::Pawn;
    state.bbs.pawn_rel_bb |= ((1 << rel_pos) * (is_pawn as RawBitboard));
}

fn swap_active(state: &mut GameState) {
    swap_bytes_inplace_u64(&mut state.bbs.pawn_rel_bb);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[Color::White]);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[Color::Black]);
    state.bbs.active_player = state.bbs.active_player.oppo();
}


// # Make

fn make_pmove(state: &mut GameState, mgmove: MGPieceMove) {
    let piece = state.occupant_lut[mgmove.origin].unwrap();
    let capture = state.occupant_lut[mgmove.target];
    
    clear_tile(state, mgmove.origin);
    clear_tile(state, mgmove.target);

    let end_species = pick(mgmove.promote.is_some(), 
        mgmove.promote.unwrap(), piece.species());
    fill_tile(state, mgmove.destin, Piece::new(piece.color(), end_species));

    let prev_crights = state.crights;
    update_crights(state); 

    state.movelog.push(MovelogEntry { prev_crights,
        lmove: LoggedMove::Piece(LoggedPieceMove { mgmove, capture }) });
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
    fill_tile(state, rook_destin, Piece::new(state.active_player(), Species::Rook));
    fill_tile(state, king_destin, Piece::new(state.active_player(), Species::King));

    let prev_crights = state.crights;
    state.crights.revoke(state.active_player());

    state.movelog.push(MovelogEntry { 
        prev_crights,
        lmove: LoggedMove::Castle(side)
    });
}

// # Unmake

fn unmake_pmove(state: &mut GameState, pmove: LoggedPieceMove) {
    let is_promote = (pmove.mgmove.special == Some(SpecialPieceMove::Promote));
    let species = pick(is_promote, Species::Pawn, 
        state.occupant_lut[pmove.mgmove.destin].unwrap().species());
    
    clear_tile(state, pmove.mgmove.destin);
    clear_tile(state, pmove.mgmove.target);
    
    if let Some(piece) = pmove.capture {
        fill_tile(state, pmove.mgmove.target, piece);
    }

    fill_tile(state, pmove.mgmove.origin,
        Piece::new(state.active_player(), species));
}

fn unmake_castle(state: &mut GameState, side: FileDirection) {
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

    clear_tile(state, rook_destin);
    clear_tile(state, king_destin);
    fill_tile(state, rook_origin, Piece::new(state.active_player(), 
        Species::Rook));
    fill_tile(state, king_origin, Piece::new(state.active_player(),
        Species::King));
}

pub fn unmake_move(state: &mut GameState) {
    let last_entry = state.movelog.pop().unwrap();
    state.crights = last_entry.prev_crights;
        
    swap_active(state);
    
    match last_entry.lmove {
        LoggedMove::Castle(side) => unmake_castle(state, side),
        LoggedMove::Piece(pmove) => unmake_pmove(state, pmove),
    }
}

// # Test

/// Calculates the legality of a psuedo-legal move.
/// This procedure returns `true` if the move is legal, 
/// and false otherwise.
pub fn test_pmove(state: &mut GameState, pmove: MGPieceMove) -> bool {
    make_pmove(state, pmove);
    let is_legal = is_check(&state.bbs, locate_king_stdc(&state.bbs));
    swap_active(state);
    unmake_move(state);
    return is_legal;
}
