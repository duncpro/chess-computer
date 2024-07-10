use crate::gamestate::LoggedMove;
use crate::grid::FileDirection;
use crate::gamestate::FastPosition;

pub fn save_game<W>(stream: &mut W, state: &FastPosition) 
-> std::io::Result<()> 
where W: std::io::Write
{
    for ml_entry in &state.movelog {
        write_move(stream, ml_entry.lmove)?;
        write!(stream, ", ")?;
    }

    Ok(())
}

fn write_move<W>(stream: &mut W, lmove: LoggedMove) 
-> std::io::Result<FastPosition> 
where W: std::io::Write
{
    match lmove {
        LoggedMove::Castle(direction) => {
            write!(stream, "Castle")?;
            match direction {
                FileDirection::Queenside 
                    => write!(stream, "Queenside")?,
                FileDirection::Kingside  
                    => write!(stream, "Kingside")?
            }
        },
        LoggedMove::Piece(pmove) => {
            write!(stream, "{} -> {}", pmove.mgmove.origin,
                pmove.mgmove.destin)?;
        },
    }
    todo!()
}

pub fn load_game<R>(stream: R) -> std::io::Result<FastPosition>
{
    todo!()
}

