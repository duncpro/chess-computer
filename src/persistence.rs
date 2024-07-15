use crate::gamestate::ChessGame;
use crate::grid::Side;
use crate::grid::ParseStandardCoordinateError;
use crate::grid::StandardCoordinate;
use crate::makemove::make_move;
use crate::movegen::dispatch::movegen_legal;
use crate::mov::PieceMove;
use crate::piece::Species;
use std::str::FromStr;
use crate::mov::AnyMove;
use crate::movegen::types::GeneratedMove;

pub fn apply_gstr(state: &mut ChessGame, gstr: &str) -> Result<(), LoadGameErr> {
    let tokens = gstr.split(";").map(|s| s.trim());
    for token in tokens {
        if token.is_empty() { continue; }
        let prop_move = match token {
            "CastleQueenside" => AnyMove::Castle(Side::Queenside),
            "CastleKingside" => AnyMove::Castle(Side::Kingside),
            other => AnyMove::Piece(parse_pmove(other)?)
        };

        let mut legal_moves: Vec<GeneratedMove> = Vec::new();
        movegen_legal(state, &mut legal_moves);

        let is_legal = legal_moves.iter().map(|genmove| genmove.mov)
            .any(|mov| mov == prop_move);
        if !is_legal { return Err(LoadGameErr::IllegalMove(prop_move)) }
        make_move(state, prop_move);
    }
    return Ok(());
}

#[derive(Debug)]
pub enum LoadGameErr {
    ParsePMove(ParsePMoveErr),
    IllegalMove(AnyMove)
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

pub fn parse_pmove(token: &str) -> Result<PieceMove, ParsePMoveErr> {
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
    return Ok(PieceMove { origin, destin, promote });
}

fn write_pmove<W>(stream: &mut W, pmove: PieceMove) -> std::io::Result<()>
where W: std::io::Write
{
    write!(stream, "{}:{}", pmove.origin, pmove.destin)?;
    if let Some(desire) = pmove.promote {
       let ch = match desire {
            Species::Rook => "r",
            Species::Knight => "k",
            Species::Bishop => "b",
            Species::Queen => "q",
            other => panic!("cannot encode promotion to {:?}", other)
        };
        write!(stream, "{}", ch)?;
    }

    return Ok(());
}

pub fn write_move<W>(stream: &mut W, mov: AnyMove) -> std::io::Result<()>
where W: std::io::Write
{
    match mov {
        AnyMove::Piece(pmove) => write_pmove(stream, pmove)?,
        AnyMove::Castle(direction) => match direction {
            Side::Queenside => write!(stream, "CastleQueenside")?,
            Side::Kingside => write!(stream, "CastleKingside")?
        }
    }
    write!(stream, ";")?;
    return Ok(())
}