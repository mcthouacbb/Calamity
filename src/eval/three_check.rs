use crate::games::three_check::{Color, Piece, ThreeCheckBoard};

use super::Eval;

#[derive(Debug, Default, Clone, Copy)]
pub struct ThreeCheckEval {}

impl Eval<ThreeCheckBoard> for ThreeCheckEval {
    fn evaluate(board: &ThreeCheckBoard) -> i32 {
        let state = board.curr_state();
        let mut eval = 100
            * (state.colored_pieces(Piece::WhitePawn).popcount() as i32
                - state.colored_pieces(Piece::BlackPawn).popcount() as i32);
        eval += 300
            * (state.colored_pieces(Piece::WhiteKnight).popcount() as i32
                - state.colored_pieces(Piece::BlackKnight).popcount() as i32);
        eval += 300
            * (state.colored_pieces(Piece::WhiteBishop).popcount() as i32
                - state.colored_pieces(Piece::BlackBishop).popcount() as i32);
        eval += 500
            * (state.colored_pieces(Piece::WhiteRook).popcount() as i32
                - state.colored_pieces(Piece::BlackRook).popcount() as i32);
        eval += 900
            * (state.colored_pieces(Piece::WhiteQueen).popcount() as i32
                - state.colored_pieces(Piece::BlackQueen).popcount() as i32);
		
		if state.stm() == Color::White {
			eval
		} else {
			-eval
		}
    }
}
