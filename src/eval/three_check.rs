use crate::games::three_check::{
    Bitboard, Color, Piece, PieceType, ThreeCheckBoard, ThreeCheckState, attacks,
};

use super::Eval;

struct EvalData {
    mobility_area: [Bitboard; 2],
    attacked: [Bitboard; 2],
    attacked_by: [[Bitboard; 6]; 2],
}

impl EvalData {
    fn new(state: &ThreeCheckState) -> Self {
        let mut result = Self {
            mobility_area: [Bitboard::NONE; 2],
            attacked: [Bitboard::NONE; 2],
            attacked_by: [[Bitboard::NONE; 6]; 2],
        };

        let w_pawn_atks =
            attacks::pawn_attacks_bb(Color::White, state.colored_pieces(Piece::WhitePawn));
        let b_pawn_atks =
            attacks::pawn_attacks_bb(Color::Black, state.colored_pieces(Piece::BlackPawn));

        result.attacked[Color::White as usize] |= w_pawn_atks;
        result.attacked_by[Color::White as usize][PieceType::Pawn as usize] |= w_pawn_atks;
        result.attacked[Color::Black as usize] |= b_pawn_atks;
        result.attacked_by[Color::Black as usize][PieceType::Pawn as usize] |= b_pawn_atks;

        result.mobility_area[Color::White as usize] = !b_pawn_atks;
        result.mobility_area[Color::Black as usize] = !w_pawn_atks;

        result
    }
}

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
const MATERIAL: [i32; 6] = [78, 308, 319, 483, 966, 0];

pub fn piece_value(pt: PieceType) -> i32 {
    MATERIAL[pt as usize]
}

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
            eval += MATERIAL[i];
        }
    }
    eval
}

fn evaluate_pieces(state: &ThreeCheckState, eval_data: &mut EvalData, color: Color) -> i32 {
    let mut eval = 0;
    eval += evaluate_piece(state, eval_data, color, PieceType::Knight);
    eval += evaluate_piece(state, eval_data, color, PieceType::Bishop);
    eval += evaluate_piece(state, eval_data, color, PieceType::Rook);
    eval += evaluate_piece(state, eval_data, color, PieceType::Queen);
    eval
}

fn evaluate_piece(
    state: &ThreeCheckState,
    eval_data: &mut EvalData,
    color: Color,
    pt: PieceType,
) -> i32 {
    let mut eval = 0;
    let mut bb = state.colored_pieces(Piece::new(color, pt));
    let mobility_area = eval_data.mobility_area[color as usize];
    while bb.any() {
        let sq = bb.poplsb();
        match pt {
            PieceType::Knight => {
                let atk = attacks::knight_attacks(sq);
                eval_data.attacked[color as usize] |= atk;
                eval_data.attacked_by[color as usize][pt as usize] |= atk;
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 735 - 2896) / 100;
            }
            PieceType::Bishop => {
                let atk = attacks::bishop_attacks(sq, state.occ());
                eval_data.attacked[color as usize] |= atk;
                eval_data.attacked_by[color as usize][pt as usize] |= atk;
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 487 - 2993) / 100;
            }
            PieceType::Rook => {
                let atk = attacks::rook_attacks(sq, state.occ());
                eval_data.attacked[color as usize] |= atk;
                eval_data.attacked_by[color as usize][pt as usize] |= atk;
                let mobility = atk & mobility_area;
                eval += (mobility.popcount() as i32 * 486 - 3485) / 100;
            }
            PieceType::Queen => {
                let atk = attacks::queen_attacks(sq, state.occ());
                eval_data.attacked[color as usize] |= atk;
                eval_data.attacked_by[color as usize][pt as usize] |= atk;
                let mobility = atk & mobility_area;
                eval += (mobility.popcount().min(20) as i32 * 536 - 5390) / 100;
            }
            _ => unreachable!(),
        }
    }
    eval
}

fn evaluate_king(state: &ThreeCheckState, eval_data: &EvalData, color: Color) -> i32 {
    let their_king = state.king_sq(color.flip());

    let safe = !eval_data.attacked[color.flip() as usize];

    let mut knight_checks = attacks::knight_attacks(their_king);
    let mut bishop_checks = attacks::bishop_attacks(their_king, state.occ());
    let mut rook_checks = attacks::rook_attacks(their_king, state.occ());
    let mut queen_checks = bishop_checks | rook_checks;

    knight_checks &= eval_data.attacked_by[color as usize][PieceType::Knight as usize];
    bishop_checks &= eval_data.attacked_by[color as usize][PieceType::Bishop as usize];
    rook_checks &= eval_data.attacked_by[color as usize][PieceType::Rook as usize];
    queen_checks &= eval_data.attacked_by[color as usize][PieceType::Queen as usize];
    let all_checks = knight_checks | bishop_checks | rook_checks | queen_checks;

    let mut eval = 0;
    eval += 50 * (knight_checks & safe).popcount() as i32;
    eval += 50 * (bishop_checks & safe).popcount() as i32;
    eval += 70 * (rook_checks & safe).popcount() as i32;
    eval += 90 * (queen_checks & safe).popcount() as i32;
    eval += 40 * (all_checks & !safe).popcount() as i32;
    eval
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ThreeCheckEval {}

impl Eval<ThreeCheckBoard> for ThreeCheckEval {
    fn evaluate(board: &ThreeCheckBoard) -> i32 {
        const CHECK_PENALTY: [i32; 3] = [0, -200, -750];

        let state = board.curr_state();
        let mut eval = 0;
        let mut eval_data = EvalData::new(state);

        eval += eval_psqt(state, Color::White) - eval_psqt(state, Color::Black);
        eval += evaluate_pieces(state, &mut eval_data, Color::White)
            - evaluate_pieces(state, &mut eval_data, Color::Black);
        eval += evaluate_king(state, &eval_data, Color::White)
            - evaluate_king(state, &eval_data, Color::Black);

        eval += CHECK_PENALTY[state.check_count(Color::White) as usize]
            - CHECK_PENALTY[state.check_count(Color::Black) as usize];

        if state.stm() == Color::White {
            eval
        } else {
            -eval
        }
    }
}
