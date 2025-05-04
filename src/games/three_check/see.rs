use super::{Bitboard, Color, Move, MoveKind, Piece, PieceType, ThreeCheckState, attacks};

fn see_piece_value(pt: PieceType) -> i32 {
    const VALUES: [i32; 6] = [100, 450, 450, 650, 1350, 0];
    VALUES[pt as usize]
}

fn pop_least_valuable(
    state: &ThreeCheckState,
    occupancy: &mut Bitboard,
    attackers: Bitboard,
) -> Option<PieceType> {
    for i in 0..6 {
        let pt = PieceType::from_raw(i);
        let pieces = attackers & state.pieces(pt);
        if pieces.any() {
            occupancy.toggle(pieces.lsb());
            return Some(pt);
        }
    }
    None
}

// yoinked from stormphrax
pub fn see(state: &ThreeCheckState, mv: Move, threshold: i32) -> bool {
    if mv.kind() != MoveKind::None {
        return true;
    }

    let mut score = if let Some(captured) = state.piece_at(mv.to_sq()) {
        see_piece_value(captured.piece_type())
    } else {
        0
    };

    score -= threshold;

    if score < 0 {
        return false;
    }

    let next = state.piece_at(mv.from_sq()).unwrap().piece_type();

    score -= see_piece_value(next);

    if score >= 0 {
        return true;
    }

    let square = mv.to_sq();

    let mut occupancy = state.occ();
    occupancy.toggle(square);
    occupancy.toggle(mv.from_sq());

    let mut attackers = state.all_attackers_to(square, occupancy);

    let mut us = state.stm.flip();

    loop {
        let our_attackers = attackers & state.colors(us);
        if our_attackers.empty() {
            break;
        }

        let next = pop_least_valuable(state, &mut occupancy, our_attackers);

        if next == Some(PieceType::Pawn)
            || next == Some(PieceType::Bishop)
            || next == Some(PieceType::Queen)
        {
            attackers |= attacks::bishop_attacks(square, occupancy)
                & (state.pieces(PieceType::Bishop) | state.pieces(PieceType::Queen));
        }

        if next == Some(PieceType::Rook) || next == Some(PieceType::Queen) {
            attackers |= attacks::rook_attacks(square, occupancy)
                & (state.pieces(PieceType::Rook) | state.pieces(PieceType::Queen));
        }

        attackers &= occupancy;
        score = -score
            - 1
            - if let Some(pt) = next {
                see_piece_value(pt)
            } else {
                0
            };

        us = us.flip();

        if score >= 0 {
            if next == Some(PieceType::King) && (attackers & state.colors(us)).any() {
                us = us.flip();
            }
            break;
        }
    }
    return state.stm != us;
}
