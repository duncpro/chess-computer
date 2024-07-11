use crate::gamestate::ChessGame;
use crate::grid::FileDirection;
use crate::grid::ParseStandardCoordinateError;
use crate::grid::StandardCoordinate;
use crate::makemove::make_move;
use crate::movegen::dispatch::movegen_legal;
use crate::movegen::types::MGAnyMove;
use crate::movegen::types::PMGMove;
use crate::piece::Species;
use crate::stdinit::new_std_chess_position;
use std::str::FromStr;

// # Load Game

pub fn load_game(text: &str) -> Result<ChessGame, LoadGameErr> {
    let mut state = new_std_chess_position();
    let tokens = text.split(";").map(|s| s.trim());
    for token in tokens {
        if token.is_empty() { continue; }
        let mgmove = match token {
            "CastleQueenside" => MGAnyMove::Castle(FileDirection::Queenside),
            "CastleKingside" => MGAnyMove::Castle(FileDirection::Kingside),
            other => MGAnyMove::Piece(parse_pmove(other)?)
        };

        let mut legal_moves: Vec<MGAnyMove> = Vec::new();
        movegen_legal(&mut state, &mut legal_moves);
        if !legal_moves.contains(&mgmove) {
            return Err(LoadGameErr::IllegalMove(mgmove))
        }
        make_move(&mut state, mgmove);
    }
    return Ok(state);
}

#[derive(Debug)]
pub enum LoadGameErr {
    ParsePMove(ParsePMoveErr),
    IllegalMove(MGAnyMove)
}

impl From<ParsePMoveErr> for LoadGameErr {
    fn from(value: ParsePMoveErr) -> Self {
        Self::ParsePMove(value)
    }
}

#[derive(Debug)]
pub enum ParsePMoveErr {
    MissingOrigin,
    MalformedOrigin(ParseStandardCoordinateError),
    MissingDestin,
    MalformedDestin(ParseStandardCoordinateError),
    MalformedPromote,
    TooManyParts
}

pub fn parse_pmove(token: &str) -> Result<PMGMove, ParsePMoveErr> {
    let mut subtokens = token.split(":").map(|s| s.trim());
    let origin_part = subtokens.next().ok_or(ParsePMoveErr::MissingOrigin)?;
    let origin = StandardCoordinate::from_str(origin_part)
        .map_err(|e| ParsePMoveErr::MalformedOrigin(e))?;
    let destin_part = subtokens.next().ok_or(ParsePMoveErr::MissingDestin)?;
    let destin = StandardCoordinate::from_str(destin_part)
        .map_err(|e| ParsePMoveErr::MalformedDestin(e))?;
    let mut promote: Option<Species> = None;
    if let Some(promote_part) = subtokens.next() {
        if promote_part.len() != 1 { return Err(ParsePMoveErr::MalformedPromote); }
        let ch = promote_part.chars().next().unwrap();
        match ch {
            'q' | 'Q' => promote = Some(Species::Queen),
            'k' | 'K' => promote = Some(Species::Knight),
            'b' | 'B' => promote = Some(Species::Bishop),
            'r' | 'R' => promote = Some(Species::Rook),
            _ => return Err(ParsePMoveErr::MalformedPromote)
        }
    }
    if subtokens.count() != 0 { return Err(ParsePMoveErr::TooManyParts); }
    return Ok(PMGMove { origin, destin, promote });
}