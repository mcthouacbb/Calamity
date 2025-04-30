use crate::games::three_check::{attacks, Color, Piece, PieceType, ThreeCheckBoard, ThreeCheckState};

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

fn evaluate_pieces(state: &ThreeCheckState, color: Color) -> i32 {
    let mut eval = 0;
    eval += evaluate_piece(state, color, PieceType::Knight);
    eval += evaluate_piece(state, color, PieceType::Bishop);
    eval += evaluate_piece(state, color, PieceType::Rook);
    eval += evaluate_piece(state, color, PieceType::Queen);
    eval
}

fn evaluate_piece(state: &ThreeCheckState, color: Color, pt: PieceType) -> i32 {
    let mut eval = 0;
    let mut bb = state.colored_pieces(Piece::new(color, pt));
    let opp_pawn_atks = attacks::pawn_attacks_bb(color.flip(), state.colored_pieces(Piece::new(color.flip(), PieceType::Pawn)));
    let mobility_area = !opp_pawn_atks;
    while bb.any() {
        let sq = bb.poplsb();
        match pt {
            PieceType::Knight => {
                let atk = attacks::knight_attacks(sq);
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 735 - 2896) / 100;
            }
            PieceType::Bishop => {
                let atk = attacks::bishop_attacks(sq, state.occ());
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 487 - 2993) / 100;
            }
            PieceType::Rook => {
                let atk = attacks::rook_attacks(sq, state.occ());
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 486 - 3485) / 100;
            }
            PieceType::Queen => {
                let atk = attacks::queen_attacks(sq, state.occ());
                let mobility = atk & mobility_area;
                eval += (mobility.popcount().min(20) as i32 * 536 - 5390) / 100;
            }
            _ => unreachable!()
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
        eval += evaluate_pieces(state, Color::White) - evaluate_pieces(state, Color::Black);

        eval += CHECK_PENALTY[state.check_count(Color::White) as usize]
            - CHECK_PENALTY[state.check_count(Color::Black) as usize];

        if state.stm() == Color::White {
            eval
        } else {
            -eval
        }
    }
}
