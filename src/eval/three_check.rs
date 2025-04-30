use crate::{
    games::three_check::{Color, Piece, PieceType, ThreeCheckBoard, ThreeCheckState},
    search::three_check::ThreeCheckSearch,
};

use super::Eval;

const PST_RANK: [i32; 48] = [
    0, -12, -14, -13, -1, 40, 114, 0, // Pawn
    -36, -19, 1, 16, 28, 28, 8, -25, // Knight
    -27, -9, 3, 10, 15, 15, 3, -9, // Bishop
    -11, -19, -19, -9, 6, 15, 21, 16, // Rook
    -21, -13, -9, -3, 6, 16, 6, 16, // Queen
    -20, -12, -5, 6, 18, 24, 13, -15, // King
];
const PST_FILE: [i32; 48] = [
    -2, 2, -5, -2, 0, 5, 10, -8, // Pawn
    -28, -7, 6, 15, 14, 13, 1, -14, // Knight
    -13, 0, 3, 5, 6, 1, 5, -7, // Bishop
    -2, 0, 3, 5, 4, 6, -2, -14, // Rook
    -22, -9, 2, 6, 5, 6, 6, 6, // Queen
    -13, 3, 1, 0, -2, -2, 6, -10, // King
];

fn eval_psqt(state: &ThreeCheckState, color: Color) -> i32 {
    let mut eval = 0;
    for (i, pt) in [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ]
    .iter()
    .enumerate()
    {
        let mut bb = state.colored_pieces(Piece::new(color, *pt));
        while bb.any() {
            let sq = bb.poplsb();
            let mirror = if color == Color::Black { 0b111 } else { 0 };
            eval += PST_RANK[(sq.rank() as usize ^ mirror) + 8 * i];
            eval += PST_FILE[sq.file() as usize + 8 * i];
        }
    }
    eval
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ThreeCheckEval {}

impl Eval<ThreeCheckBoard> for ThreeCheckEval {
    fn evaluate(board: &ThreeCheckBoard) -> i32 {
        const CHECK_PENALTY: [i32; 3] = [0, -200, -750];

        let state = board.curr_state();
        let mut eval = 78
            * (state.colored_pieces(Piece::WhitePawn).popcount() as i32
                - state.colored_pieces(Piece::BlackPawn).popcount() as i32);
        eval += 308
            * (state.colored_pieces(Piece::WhiteKnight).popcount() as i32
                - state.colored_pieces(Piece::BlackKnight).popcount() as i32);
        eval += 319
            * (state.colored_pieces(Piece::WhiteBishop).popcount() as i32
                - state.colored_pieces(Piece::BlackBishop).popcount() as i32);
        eval += 483
            * (state.colored_pieces(Piece::WhiteRook).popcount() as i32
                - state.colored_pieces(Piece::BlackRook).popcount() as i32);
        eval += 966
            * (state.colored_pieces(Piece::WhiteQueen).popcount() as i32
                - state.colored_pieces(Piece::BlackQueen).popcount() as i32);
        
        eval += eval_psqt(state, Color::White) - eval_psqt(state, Color::Black);

        eval += CHECK_PENALTY[state.check_count(Color::White) as usize]
            - CHECK_PENALTY[state.check_count(Color::Black) as usize];

        if state.stm() == Color::White {
            eval
        } else {
            -eval
        }
    }
}
