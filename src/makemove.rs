use crate::mov::get_target_sq;
use crate::bitboard::RawBitboard;
use crate::bits::swap_bytes_inplace_u64;
use crate::crights::update_crights_all;
use crate::enpassant::is_enpassant_vuln;
use crate::gamestate::LoggedMove;
use crate::gamestate::LoggedPieceMove;
use crate::gamestate::MovelogEntry;
use crate::grid::File;
use crate::grid::Rank;
use crate::grid::Side;
use crate::grid::StandardCoordinate;
use crate::mov::AnyMove;
use crate::piece::Color;
use crate::piece::Piece;
use crate::piece::Species;
use crate::misc::pick;
use crate::mov::PieceMove;
use crate::gamestate::ChessGame;
use crate::rmrel::relativize;
use crate::expect_match;
use crate::setbit;
use crate::cli::print_board;
use crate::snapshot::capture_snapshot;
use crate::unsetbit;

// # Utilities

fn clear_tile(state: &mut ChessGame, pos: StandardCoordinate) {
    let occupant = state.p_lut.get(pos);
    if let Some(piece) = occupant {
        state.bbs.affilia_bbs[piece.color()].unset(pos);
        state.bbs.species_bbs[piece.species()].unset(pos);
        unsetbit!(state.bbs.affilia_rel_bbs[piece.color()],
            relativize(pos, state.active_player()));
        state.hash.toggle_tile(pos, piece);
    }
    state.p_lut.set(pos, None);
    unsetbit!(state.bbs.pawn_rel_bb, 
        relativize(pos, state.active_player()));
}

pub fn fill_tile(state: &mut ChessGame, pos: StandardCoordinate, piece: Piece)
{
    assert!(state.p_lut.get(pos).is_none());
    state.p_lut.set(pos, Some(piece));
    state.bbs.affilia_bbs[piece.color()].set(pos);
    state.bbs.species_bbs[piece.species()].set(pos);
    state.hash.toggle_tile(pos, piece);
    
    let rel_pos = relativize(pos, state.active_player());
    setbit!(state.bbs.affilia_rel_bbs[piece.color()], rel_pos);
    let is_pawn = piece.species() == Species::Pawn;
    state.bbs.pawn_rel_bb |= (1 << rel_pos) * (is_pawn as RawBitboard);
}

pub fn swap_active(state: &mut ChessGame) {
    swap_bytes_inplace_u64(&mut state.bbs.pawn_rel_bb);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[Color::White]);
    swap_bytes_inplace_u64(
        &mut state.bbs.affilia_rel_bbs[Color::Black]);
    state.bbs.active_player.swap();
    state.hash.toggle_active();
}

// # Make

pub fn make_pmove(state: &mut ChessGame, mgmove: PieceMove) {
    let piece = state.p_lut.get(mgmove.origin).unwrap();
    let target = get_target_sq(mgmove, state);
    let capture = state.p_lut.get(target);
    
    clear_tile(state, mgmove.origin);
    clear_tile(state, target);

    let place_piece = Piece::new(state.active_player(),
        mgmove.promote.unwrap_or(piece.species()));        
    fill_tile(state, mgmove.destin, place_piece);

    let prev_crights = state.crights;
    update_crights_all(state);

    let prev_halfmoveclock = state.halfmoveclock;
    state.halfmoveclock += 1;
    let reset_halfmoveclock = capture.is_some() 
        | (piece.species() == Species::Pawn);
    state.halfmoveclock *= (!reset_halfmoveclock) as u16;

    let is_pdj = (piece.species() == Species::Pawn)
        & (mgmove.origin.rank() == Rank::pawn_rank(state.active_player()))
        & (mgmove.destin.rank() == Rank::pdj_rank(state.active_player()));

    let lpm = LoggedPieceMove { mgmove, capture, is_pdj, target };
    let mle = MovelogEntry { prev_crights, prev_halfmoveclock,
        lmove: LoggedMove::Piece(lpm) };
    state.movelog.push(mle);
}

pub fn make_castle(state: &mut ChessGame, side: Side) {
    const ROOK_ORIGIN_LUT: [File; 2] = [
        /* Queenside */ File::A,
        /* Kingside  */ File::H
    ];
    let rook_origin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        ROOK_ORIGIN_LUT[usize::from(side.index())]
    );

    const ROOK_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::D,
        /* Kingside  */ File::F
    ];
    let rook_destin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        ROOK_DESTIN_LUT[usize::from(side.index())]
    );

    const KING_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::C,
        /* Kingside  */ File::G
    ];
    let king_destin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        KING_DESTIN_LUT[usize::from(side.index())]
    );
    
    let king_origin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()), File::E);

    clear_tile(state, king_origin);
    clear_tile(state, rook_origin);
    fill_tile(state, rook_destin, Piece::new(state.active_player(), Species::Rook));
    fill_tile(state, king_destin, Piece::new(state.active_player(), Species::King));

    let prev_crights = state.crights;
    state.crights.revoke(state.active_player());

    let prev_halfmoveclock = state.halfmoveclock;
    state.halfmoveclock += 1;

    state.movelog.push(MovelogEntry { prev_crights, prev_halfmoveclock,
        lmove: LoggedMove::Castle(side) });

    let active_player = state.active_player();
    state.has_castled[active_player] = true;
}

pub fn make_move(state: &mut ChessGame, mov: AnyMove) {
    state.hash.toggle_ep_vuln(is_enpassant_vuln(state));
    state.hash.toggle_crights(state.crights); // clear
    match mov {
        AnyMove::Piece(pmove) => make_pmove(state, pmove),
        AnyMove::Castle(side) => make_castle(state, side),
    }
    swap_active(state);
    state.hash.toggle_ep_vuln(is_enpassant_vuln(state));
    state.hash.toggle_crights(state.crights); // restore
}

// # Unmake

fn unmake_pmove(state: &mut ChessGame, pmove: LoggedPieceMove) {
    let species = {
        let is_promote = pmove.mgmove.promote.is_some();
        let current_species = state.p_lut.get(pmove.mgmove.destin)
            .unwrap().species();
        pick(is_promote, Species::Pawn, current_species)
    };
      
    clear_tile(state, pmove.mgmove.destin);
    clear_tile(state, pmove.target);
    
    if let Some(piece) = pmove.capture {
        fill_tile(state, pmove.target, piece);
    }
    
    fill_tile(state, pmove.mgmove.origin,
        Piece::new(state.active_player(), species));
}


fn unmake_castle(state: &mut ChessGame, side: Side) {
    const ROOK_ORIGIN_LUT: [File; 2] = [
        /* Queenside */ File::A,
        /* Kingside  */ File::H
    ];
    let rook_origin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        ROOK_ORIGIN_LUT[usize::from(side.index())]
    );

    const ROOK_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::D,
        /* Kingside  */ File::F
    ];
    let rook_destin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        ROOK_DESTIN_LUT[usize::from(side.index())]
    );

    const KING_DESTIN_LUT: [File; 2] = [
        /* Queenside */ File::C,
        /* Kingside  */ File::G
    ];
    let king_destin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()),
        KING_DESTIN_LUT[usize::from(side.index())]
    );
    
    let king_origin = StandardCoordinate::new(
        Rank::base_rank(state.active_player()), File::E);

    clear_tile(state, rook_destin);
    clear_tile(state, king_destin);
    fill_tile(state, rook_origin, Piece::new(state.active_player(), 
        Species::Rook));
    fill_tile(state, king_origin, Piece::new(state.active_player(),
        Species::King));

    let active_player = state.active_player();
    state.has_castled[active_player] = false;
}

pub fn unmake_move(state: &mut ChessGame) {
    state.hash.toggle_ep_vuln(is_enpassant_vuln(state));
    
    let last_entry = state.movelog.pop().unwrap();
    
    state.hash.toggle_crights(state.crights);           // clear
    state.hash.toggle_crights(last_entry.prev_crights); // restore
    
    state.crights = last_entry.prev_crights;
    state.halfmoveclock = last_entry.prev_halfmoveclock;

    swap_active(state);
    
    match last_entry.lmove {
        LoggedMove::Castle(side) => unmake_castle(state, side),
        LoggedMove::Piece(pmove) => unmake_pmove(state, pmove),
    }

    state.hash.toggle_ep_vuln(is_enpassant_vuln(state));
}

/// Calculates the legality of a pseudo-legal move.
/// This procedure returns `true` if the move is legal and false otherwise.
pub fn test_pmove_legality(state: &mut ChessGame, pmove: PieceMove) -> bool {
    make_pmove(state, pmove);
    let is_legal = !state.bbs.is_check();
    expect_match!(state.movelog.pop(), Some(ml_entry));
    state.halfmoveclock = ml_entry.prev_halfmoveclock;
    state.crights = ml_entry.prev_crights;
    expect_match!(ml_entry.lmove, LoggedMove::Piece(lpm));
    unmake_pmove(state, lpm);
    return is_legal;
}

pub fn inspect_move<R, F>(state: &mut ChessGame, mov: AnyMove, mut inspection: F) -> R
where F: FnMut(&mut ChessGame) -> R
{
    make_move(state, mov);
    let result = inspection(state);
    unmake_move(state);
    return result;
}