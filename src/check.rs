use crate::bitboard::Bitboard;
use crate::bitboard::RawBitboard;
use crate::coordinates::FileMajorCS;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::StandardCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::gamestate::Bitboards;
use crate::gamestate::locate_king;
use crate::gamestate::locate_king_stdc;
use crate::grid::StandardCoordinate;
use crate::lane::lanelimit;
use crate::lane::lanescan;
use crate::misc::PieceColor;
use crate::misc::OptionPieceSpecies;
use crate::movegen::knight::knight_attack;
use crate::misc::PieceSpecies;
use crate::movegen::pawn::reverse_pawn_attack;
use crate::rmrel::relativize;

/// Determines if a hypothetical king placed on `vuln_sq` of
/// is currently checked by the opponent.
pub fn is_check(board: &Bitboards, vuln_sq: StandardCoordinate) -> bool {
    let mut check: bool = false;
    let args = CheckQuery { board,  vuln_sq };
    check |= is_check_pawn_qs(args);
    check |= is_check_rankslide(args);
    check |= is_check_fileslide(args);
    check |= is_check_prodiag_slide(args);
    check |= is_check_antidiag_slide(args);
    check |= is_check_knight(args);
    return check;
}

#[derive(Clone, Copy)]
struct CheckQuery<'a> { 
    board: &'a Bitboards,
    vuln_sq: StandardCoordinate 
}

fn is_check_pawn_qs(args: CheckQuery) -> bool {
    let king_rmrel = relativize(locate_king_stdc(args.board),
        args.board.active_player);

    let mut bb = reverse_pawn_attack(king_rmrel);
    bb &= args.board.affilia_rel_bbs[args.board.active_player.oppo()];
    bb &= args.board.pawn_rel_bb;

    return bb != 0;
}

fn is_check_rankslide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Rook].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_fileslide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<FileMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Rook].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_prodiag_slide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<ProdiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Bishop].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_antidiag_slide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<AntidiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Bishop].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_knight(args: CheckQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb = args.board.class(args.board.active_player.oppo(), PieceSpecies::Knight);
    bb &= knight_attack(args.vuln_sq.into());
    return bb.is_not_empty();
}
