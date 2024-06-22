use crate::bitboard::Bitboard;
use crate::coordinates::FileMajorCS;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::StandardCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::GameState;
use crate::gamestate::MDBoard;
use crate::grid::StandardCoordinate;
use crate::lane::lanelimit;
use crate::lane::lanescan;
use crate::misc::PieceColor;
use crate::misc::OptionPieceSpecies;
use crate::movegen::knight::knight_attack;
use crate::misc::PieceSpecies;

/// Determines if a hypothetical king placed on `vuln_sq` of
/// affiliation `color` is currently checked by the opponent.
pub fn is_check(board: &MDBoard, color: PieceColor, vuln_sq: StandardCoordinate) -> bool {
    let mut check: bool = false;
    let args = CheckQuery { board, color, vuln_sq };
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
    board: &'a MDBoard,
    color: PieceColor, 
    vuln_sq: StandardCoordinate 
}


fn is_check_pawn_qs(query: CheckQuery) -> bool {
    todo!()
}

fn is_check_rankslide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Rook].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.color.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_fileslide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<FileMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Rook].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.color.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_prodiag_slide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<ProdiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Bishop].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.color.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_antidiag_slide(args: CheckQuery) -> bool {
    let mut bb: Bitboard<AntidiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[OptionPieceSpecies::Bishop].get();
    bb |= args.board.species_bbs[OptionPieceSpecies::Queen].get();
    bb &= args.board.affilia_bbs[args.color.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_check_knight(args: CheckQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb = args.board.class(args.color.oppo(), PieceSpecies::Knight);
    bb &= knight_attack(args.vuln_sq.into());
    return bb.is_not_empty();
}