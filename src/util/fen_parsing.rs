use std::collections::HashMap;

// todo: make the result have more error information
pub fn parse_fen_pieces<Piece, F>(
    mut add_func: F,
    fen_pieces: &str,
    width: u8,
    height: u8,
    piece_mappings: HashMap<char, Piece>,
) -> Result<(), ()>
where
    F: FnMut(i32, Piece),
    Piece: Copy + Clone,
{
    let mut curr_file = 0;
    let mut curr_rank = height as i32 - 1;
    for c in fen_pieces.chars() {
        match c {
            '1'..='9' => {
                if curr_file + c as i32 - '0' as i32 > width as i32 {
                    // cannot move off the end of a row
                    return Err(());
                }
                curr_file += c as i32 - '0' as i32;
            }
            '/' => {
                if curr_file != width as i32 {
                    // slash must come after filling in all the pieces
                    return Err(());
                }
                if curr_rank == 0 {
                    // cannot end with slash
                    return Err(());
                }
                curr_file = 0;
                curr_rank -= 1;
            }
            _ => {
                if let Some(p) = piece_mappings.get(&c) {
                    if curr_file == width as i32 {
                        // cannot fill in extra pieces on a row
                        return Err(());
                    }
                    add_func(curr_file + width as i32 * curr_rank, *p);
                    curr_file += 1;
                } else {
                    // unexpected/invalid character
                    return Err(());
                }
            }
        }
    }

    Ok(())
}
