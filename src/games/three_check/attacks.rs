use super::types::{Bitboard, Color, Square};

const KNIGHT_ATTACKS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];
    let mut sq: usize = 0;
    while sq < 64 {
        let bb = Bitboard::from_square(Square::from_raw(sq as u16));
        result[sq] = result[sq].bit_or(bb.north().west().north());
        result[sq] = result[sq].bit_or(bb.north().west().west());
        result[sq] = result[sq].bit_or(bb.south().west().south());
        result[sq] = result[sq].bit_or(bb.south().west().west());
        result[sq] = result[sq].bit_or(bb.north().east().north());
        result[sq] = result[sq].bit_or(bb.north().east().east());
        result[sq] = result[sq].bit_or(bb.south().east().south());
        result[sq] = result[sq].bit_or(bb.south().east().east());
        sq += 1;
    }
    result
};

const KING_ATTACKS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];
    let mut sq: usize = 0;
    while sq < 64 {
        let bb = Bitboard::from_square(Square::from_raw(sq as u16));
        result[sq] = result[sq].bit_or(bb.north());
        result[sq] = result[sq].bit_or(bb.south());
        result[sq] = result[sq].bit_or(bb.east());
        result[sq] = result[sq].bit_or(bb.west());
        result[sq] = result[sq].bit_or(bb.north().east());
        result[sq] = result[sq].bit_or(bb.north().west());
        result[sq] = result[sq].bit_or(bb.south().east());
        result[sq] = result[sq].bit_or(bb.south().west());
        sq += 1;
    }
    result
};

pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

const RAYS: [[Bitboard; 8]; 64] = {
    let mut result: [[Bitboard; 8]; 64] = [[Bitboard::NONE; 8]; 64];
    let mut sq: usize = 0;
    while sq < 64 {
        let bb = Bitboard::from_square(Square::from_raw(sq as u16));
        let mut tmp = bb;
        while tmp.any() {
            tmp = tmp.north();
            result[sq][Direction::North as usize] =
                result[sq][Direction::North as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.south();
            result[sq][Direction::South as usize] =
                result[sq][Direction::South as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.east();
            result[sq][Direction::East as usize] = result[sq][Direction::East as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.west();
            result[sq][Direction::West as usize] = result[sq][Direction::West as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.north().east();
            result[sq][Direction::NorthEast as usize] =
                result[sq][Direction::NorthEast as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.north().west();
            result[sq][Direction::NorthWest as usize] =
                result[sq][Direction::NorthWest as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.south().east();
            result[sq][Direction::SouthEast as usize] =
                result[sq][Direction::SouthEast as usize].bit_or(tmp);
        }

        tmp = bb;
        while tmp.any() {
            tmp = tmp.south().west();
            result[sq][Direction::SouthWest as usize] =
                result[sq][Direction::SouthWest as usize].bit_or(tmp);
        }

        sq += 1;
    }

    result
};

const LINE_BETWEEN: [[Bitboard; 64]; 64] = {
    let mut result = [[Bitboard::NONE; 64]; 64];

    let mut sq1 = 0usize;
    while sq1 < 64 {
        let mut sq2 = 0usize;
        while sq2 < 64 {
            if ray_bb(Square::from_raw(sq1 as u16), Direction::North)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::North)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::South));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::South)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::South)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::North));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::East)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::East)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::West));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::West)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::West)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::East));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::NorthEast)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::NorthEast)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::SouthWest));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::NorthWest)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::NorthWest)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::SouthEast));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::SouthEast)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::SouthEast)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::NorthWest));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::SouthWest)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::SouthWest)
                    .bit_and(ray_bb(Square::from_raw(sq2 as u16), Direction::NorthEast));
            }
            sq2 += 1;
        }
        sq1 += 1;
    }

    result
};

const LINE_THROUGH: [[Bitboard; 64]; 64] = {
    let mut result = [[Bitboard::NONE; 64]; 64];

    let mut sq1 = 0usize;
    while sq1 < 64 {
        let mut sq2 = 0usize;
        while sq2 < 64 {
            if ray_bb(Square::from_raw(sq1 as u16), Direction::North)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::North)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::South));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::South)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::South)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::North));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::East)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::East)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::West));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::West)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::West)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::East));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::NorthEast)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::NorthEast)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::SouthWest));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::NorthWest)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::NorthWest)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::SouthEast));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::SouthEast)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::SouthEast)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::NorthWest));
            }
            if ray_bb(Square::from_raw(sq1 as u16), Direction::SouthWest)
                .has(Square::from_raw(sq2 as u16))
            {
                result[sq1][sq2] = ray_bb(Square::from_raw(sq1 as u16), Direction::SouthWest)
                    .bit_or(ray_bb(Square::from_raw(sq2 as u16), Direction::NorthEast));
            }
            sq2 += 1;
        }
        sq1 += 1;
    }

    result
};

// voidstar yoink
const DIAG: u64 = 0x8040_2010_0804_0201;

const fn diag_mask(i: u8) -> Bitboard {
    if i > 7 {
        Bitboard::from_raw(DIAG >> (8 * (i - 7)))
    } else {
        Bitboard::from_raw(DIAG << (8 * (7 - i)))
    }
}

const DIAGS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];
    let mut sq_idx = 0;
    while sq_idx < 64 {
        let square = Square::from_raw(sq_idx);
        result[sq_idx as usize] =
            Bitboard::from_square(square).bit_xor(diag_mask(7 - square.rank() + square.file()));

        sq_idx += 1;
    }

    result
};

const ANTI_DIAGS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];
    let mut sq_idx = 0;
    while sq_idx < 64 {
        let square = Square::from_raw(sq_idx);
        result[sq_idx as usize] = Bitboard::from_square(square)
            .bit_xor(diag_mask(square.rank() + square.file()).swap_bytes());

        sq_idx += 1;
    }

    result
};

const LEFT_ATTACKS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];

    let mut sq_idx = 0;
    while sq_idx < 64 {
        result[sq_idx] = Bitboard::from_raw((1 << (sq_idx as u64)) - 1)
            .bit_and(Bitboard::rank(Square::from_raw(sq_idx as u16).rank()));

        sq_idx += 1;
    }

    result
};

const RIGHT_ATTACKS: [Bitboard; 64] = {
    let mut result = [Bitboard::NONE; 64];

    let mut sq_idx = 0;
    while sq_idx < 64 {
        let raw = LEFT_ATTACKS[sq_idx].value()
            ^ (1 << (sq_idx as u64))
            ^ Bitboard::rank(Square::from_raw(sq_idx as u16).rank()).value();
        result[sq_idx] = Bitboard::from_raw(raw);

        sq_idx += 1;
    }

    result
};

const RANK_ATTACKS: [[Bitboard; 64]; 64] = {
    let mut result = [[Bitboard::NONE; 64]; 64];

    let mut sq_idx = 0;
    while sq_idx < 64 {
        let square = Square::from_raw(sq_idx);
        let mut i = 0;
        while i < 64 {
            let occ = Bitboard::from_raw(i << 1);

            let mut right = RIGHT_ATTACKS[square.file() as usize];
            let mut left = LEFT_ATTACKS[square.file() as usize];

            right = right.bit_xor(
                RIGHT_ATTACKS[right
                    .bit_and(occ)
                    .bit_or(Bitboard::from_square(Square::from_raw(63)))
                    .lsb()
                    .value() as usize],
            );
            left = left.bit_xor(
                LEFT_ATTACKS[left
                    .bit_and(occ)
                    .bit_or(Bitboard::from_square(Square::from_raw(0)))
                    .msb()
                    .value() as usize],
            );

            result[sq_idx as usize][i as usize] =
                Bitboard::from_raw(right.bit_or(left).value() << (square.rank() * 8));

            i += 1;
        }

        sq_idx += 1;
    }
    result
};

const FILE_ATTACKS: [[Bitboard; 64]; 64] = {
    let mut result = [[Bitboard::NONE; 64]; 64];

    let mut sq_idx = 0;
    while sq_idx < 64 {
        let square = Square::from_raw(sq_idx);
        let mut i = 0;
        while i < 64 {
            let h_file = RANK_ATTACKS[square.rank() as usize ^ 0x7][i]
                .value()
                .wrapping_mul(DIAG)
                & Bitboard::LAST_FILE.value();
            result[sq_idx as usize][i] = Bitboard::from_raw(h_file >> (square.file() ^ 0x7));

            i += 1;
        }

        sq_idx += 1;
    }
    result
};

pub const fn ray_bb(sq: Square, dir: Direction) -> Bitboard {
    RAYS[sq.value() as usize][dir as usize]
}

pub fn line_between(sq1: Square, sq2: Square) -> Bitboard {
    LINE_BETWEEN[sq1.value() as usize][sq2.value() as usize]
}

pub fn line_through(sq1: Square, sq2: Square) -> Bitboard {
    LINE_THROUGH[sq1.value() as usize][sq2.value() as usize]
}

pub fn pawn_pushes_bb(c: Color, bb: Bitboard) -> Bitboard {
    if c == Color::White {
        bb.north()
    } else {
        bb.south()
    }
}

pub fn pawn_east_attacks_bb(c: Color, bb: Bitboard) -> Bitboard {
    if c == Color::White {
        bb.north().east()
    } else {
        bb.south().east()
    }
}

pub fn pawn_west_attacks_bb(c: Color, bb: Bitboard) -> Bitboard {
    if c == Color::White {
        bb.north().west()
    } else {
        bb.south().west()
    }
}

pub fn pawn_attacks_bb(c: Color, bb: Bitboard) -> Bitboard {
    pawn_west_attacks_bb(c, bb) | pawn_east_attacks_bb(c, bb)
}

pub fn pawn_attacks(c: Color, sq: Square) -> Bitboard {
    pawn_attacks_bb(c, Bitboard::from_square(sq))
}

pub fn knight_attacks(sq: Square) -> Bitboard {
    KNIGHT_ATTACKS[sq.value() as usize]
}

pub fn king_attacks(sq: Square) -> Bitboard {
    KING_ATTACKS[sq.value() as usize]
}

pub fn bishop_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    let diag = DIAGS[sq.value() as usize];
    let anti_diag = ANTI_DIAGS[sq.value() as usize];

    let sq_bb = Bitboard::from_square(sq);
    let flipped_sq_bb = sq_bb.swap_bytes();

    let mut diag_attacks = occ & diag;
    let mut diag_flipped = diag_attacks.swap_bytes();

    let mut anti_diag_attacks = occ & anti_diag;
    let mut anti_diag_flipped = anti_diag_attacks.swap_bytes();

    diag_attacks = Bitboard::from_raw(diag_attacks.value().wrapping_sub(sq_bb.value()));
    anti_diag_attacks = Bitboard::from_raw(anti_diag_attacks.value().wrapping_sub(sq_bb.value()));

    diag_flipped = Bitboard::from_raw(diag_flipped.value().wrapping_sub(flipped_sq_bb.value()));
    anti_diag_flipped = Bitboard::from_raw(
        anti_diag_flipped
            .value()
            .wrapping_sub(flipped_sq_bb.value()),
    );

    diag_attacks ^= diag_flipped.swap_bytes();
    anti_diag_attacks ^= anti_diag_flipped.swap_bytes();

    return (diag_attacks & diag) | (anti_diag_attacks & anti_diag);
}

pub fn rook_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    let rank_attacks =
        RANK_ATTACKS[sq.value() as usize][(occ.value() >> (sq.rank() * 8 + 1)) as usize & 0x3f];

    let flip = ((occ.value() >> sq.file()) & Bitboard::FILE_A.value()).wrapping_mul(DIAG);
    let file_attacks = FILE_ATTACKS[sq.value() as usize][(flip >> 57) as usize & 0x3f];

    rank_attacks | file_attacks
}

pub fn queen_attacks(sq: Square, occ: Bitboard) -> Bitboard {
    rook_attacks(sq, occ) | bishop_attacks(sq, occ)
}
