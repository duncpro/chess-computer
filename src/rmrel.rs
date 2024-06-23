use crate::misc::PieceColor;

/// An involution between relative coordinates and absolute coordinates.
pub fn convert_rmrel_coord(input: u8, active: PieceColor) -> u8 {
    // - The relative and absolute coordinates are equivalent
    //   when white is the active player.
    // - When black is the active player we must invert the rank index.
    assert!(input < 64);
    let (input_rank, input_file) = (input / 8, input % 8);
    let output_rank = (7 * active.index()) as i8 +
         ((-2 * (active.index() as i8) + 1) * (input_rank as i8));
    assert!(output_rank < 8 && output_rank >= 0);
    return ((output_rank as u8) * 8) + input_file;
}


