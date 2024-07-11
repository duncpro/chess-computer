use crate::bitboard::Bitboard;
use crate::bitboard::RawBitboard;
use crate::coordinates::FileMajorCS;
use crate::coordinates::AntidiagonalMajorCS;
use crate::coordinates::ProdiagonalMajorCS;
use crate::coordinates::StandardCS;
use crate::coordinates::RankMajorCS;
use crate::gamestate::ChessGame;
use crate::gamestate::Bitboards;
use crate::gamestate::locate_king;
use crate::gamestate::locate_king_stdc;
use crate::grid::StandardCoordinate;
use crate::laneutils::lanelimit;
use crate::movegen::king::king_attack;
use crate::piece::Color;
use crate::piece::Species;
use crate::movegen::knight::knight_attack;
use crate::movegen::pawn::{pawn_attack, reverse_pawn_attack};
use crate::rmrel::relativize;

/// Determines if a hypothetical piece placed on `vuln_sq` is currently
/// targeted by the opponent.
pub fn is_attacked(board: &Bitboards, vuln_sq: StandardCoordinate) -> bool {
    let mut attacked: bool = false;
    let args = AttackQuery { board,  vuln_sq };
    attacked |= is_attacked_pawn(args);
    attacked |= is_attacked_rankslide(args);
    attacked |= is_attacked_fileslide(args);
    attacked |= is_attacked_prodiag_slide(args);
    attacked |= is_attacked_antidiag_slide(args);
    attacked |= is_attacked_knight(args);
    attacked |= is_attacked_king(args);
    return attacked;
}

#[derive(Clone, Copy)]
struct AttackQuery<'a> {
    board: &'a Bitboards,
    vuln_sq: StandardCoordinate
}

fn is_attacked_pawn(args: AttackQuery) -> bool {
    let king_rmrel = relativize(locate_king_stdc(args.board),
        args.board.active_player);

    let mut bb = pawn_attack(king_rmrel);
    bb &= args.board.affilia_rel_bbs[args.board.active_player.oppo()];
    bb &= args.board.pawn_rel_bb;

    return bb != 0;
}

fn is_attacked_rankslide(args: AttackQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[Species::Rook].get();
    bb |= args.board.species_bbs[Species::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_attacked_fileslide(args: AttackQuery) -> bool {
    let mut bb: Bitboard<FileMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[Species::Rook].get();
    bb |= args.board.species_bbs[Species::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_attacked_prodiag_slide(args: AttackQuery) -> bool {
    let mut bb: Bitboard<ProdiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[Species::Bishop].get();
    bb |= args.board.species_bbs[Species::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_attacked_antidiag_slide(args: AttackQuery) -> bool {
    let mut bb: Bitboard<AntidiagonalMajorCS> = Bitboard::empty();
    bb |= args.board.species_bbs[Species::Bishop].get();
    bb |= args.board.species_bbs[Species::Queen].get();
    bb &= args.board.affilia_bbs[args.board.active_player.oppo()].get();
    bb &= lanelimit(args.board, args.vuln_sq);
    return bb.is_not_empty();
}

fn is_attacked_knight(args: AttackQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb = args.board.class(args.board.active_player.oppo(), Species::Knight);
    bb &= knight_attack(args.vuln_sq.into());
    return bb.is_not_empty();
}

fn is_attacked_king(args: AttackQuery) -> bool {
    let mut bb: Bitboard<RankMajorCS> = Bitboard::empty();
    bb = args.board.class(args.board.active_player.oppo(), Species::King);
    bb &= king_attack(args.vuln_sq.into());
    return bb.is_not_empty();
}