use crate::attack::is_attacked;
use crate::coordinates::StandardCS;
use crate::grid::FileDirection;
use crate::grid::Rank;
use crate::grid::StandardCoordinate;
use crate::grid::File;
use crate::gamestate::ChessGame;
use crate::misc::Push;
use crate::mov::AnyMove;
use crate::movegen::types::GeneratedMove;
use crate::movegen::types::MGContext;

pub fn movegen_castle(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    movegen_castle_queenside(ctx);
    movegen_castle_kingside(ctx);
}

fn movegen_castle_kingside(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    let mut can_castle = true;
    let base_rank = Rank::base_rank(ctx.active_player());
    {
        let king_destin = StandardCoordinate::new(base_rank, File::G);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, king_destin));
        can_castle &= !ctx.occupancy::<StandardCS>().includes(king_destin.into());
    }
    {
        let rook_destin = StandardCoordinate::new(base_rank, File::F);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, rook_destin));
        can_castle &= ctx.inspect(|s| !s.bbs.occupancy::<StandardCS>()
            .includes(rook_destin.into()));
    }
    {
        let king_origin = StandardCoordinate::new(base_rank, File::E);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, king_origin));
    }
    can_castle &= ctx.inspect(|s| s.crights.get(FileDirection::Kingside, s.active_player()));
    if can_castle { ctx.push_legal(AnyMove::Castle(FileDirection::Kingside)); }
}

fn movegen_castle_queenside(ctx: &mut MGContext<impl Push<GeneratedMove>>) {
    let mut can_castle = true;
    let base_rank = Rank::base_rank(ctx.active_player());
    {
        let king_destin = StandardCoordinate::new(base_rank, File::C);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, king_destin));
        can_castle &= !ctx.occupancy::<StandardCS>().includes(king_destin.into());
    }
    {
        let rook_destin = StandardCoordinate::new(base_rank, File::D);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, rook_destin));
        can_castle &= !ctx.occupancy::<StandardCS>().includes(rook_destin.into());
    }
    {
        let knight_pos = StandardCoordinate::new(base_rank, File::B);
        can_castle &= !ctx.occupancy::<StandardCS>().includes(knight_pos.into());
    }
    {
        let king_origin = StandardCoordinate::new(base_rank, File::E);
        can_castle &= ctx.inspect(|s| !is_attacked(&s.bbs, king_origin));
    }
    can_castle &= ctx.inspect(|s|
        s.crights.get(FileDirection::Queenside, s.active_player()));
    if can_castle { ctx.push_legal(AnyMove::Castle(FileDirection::Queenside)); }
}