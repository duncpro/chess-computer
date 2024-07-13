use crate::gamestate::ChessGame;
use crate::grid::FileDirection;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::misc::pick;
use crate::piece::Species;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PieceMove {
    pub origin: StandardCoordinate,
    pub destin: StandardCoordinate,
    pub promote: Option<Species>
}

impl PieceMove {
    pub fn new_basic(origin: StandardCoordinate, destin: StandardCoordinate) -> Self
    {
        Self { origin, destin, promote: None }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnyMove {
    Piece(PieceMove),
    Castle(FileDirection)
}

/// Computes the position of the piece captured by this move (if any).
/// The target is identical to the destination in every case except enpassant.
pub fn get_target_sq(pmgmove: PieceMove, state: &mut ChessGame) -> StandardCoordinate {
    let species = state.p_lut.get(pmgmove.origin).unwrap().species();
    let is_enpassant = (species == Species::Pawn)
        & (pmgmove.origin.file() != pmgmove.destin.file())
        & state.p_lut.get(pmgmove.destin).is_none();
    let ep_targ_rank = Rank::pdj_rank(state.active_player().oppo());
    let target_rank = pick(is_enpassant, ep_targ_rank, pmgmove.destin.rank());
    return StandardCoordinate::new(target_rank, pmgmove.destin.file())
}