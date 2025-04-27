use std::ops;

use super::square::Square;

// A Bitboard stores a set of squares as bits in an integer
// the first width x height bits are used, and the rest are ignored and kept as 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard<const WIDTH: u8, const HEIGHT: u8>(u64);

impl<const WIDTH: u8, const HEIGHT: u8> Bitboard<WIDTH, HEIGHT> {
    pub const FILE_A: Self = {
        let mut result = 0u64;
        let mut i = 0;
        while i < HEIGHT {
            result |= 1 << (i * WIDTH);
            i += 1;
        }
        Self(result)
    };

    pub const RANK_0: Self = {
        let mut result = 0u64;
        let mut i = 0;
        while i < WIDTH {
            result |= 1 << i;
            i += 1;
        }
        Self(result)
    };

    pub const ALL: Self = {
        let mut result = 0u64;
        let mut i = 0;
        while i < WIDTH * HEIGHT {
            result |= 1 << i;
            i += 1;
        }
        Self(result)
    };

    pub const NONE: Self = Self(0);

    pub const LAST_FILE: Self = Self::file(WIDTH - 1);
    pub const LAST_RANK: Self = Self::rank(HEIGHT - 1);

    pub const fn from_raw(value: u64) -> Self {
        assert!((value & !Self::ALL.value()) == 0);
        Self(value)
    }

    pub const fn file(file: u8) -> Self {
        assert!(file < WIDTH);
        Self((Self::FILE_A.value() << file) & Self::ALL.value())
    }

    pub const fn rank(rank: u8) -> Self {
        assert!(rank < HEIGHT);
        Self((Self::RANK_0.value() << (WIDTH * rank)) & Self::ALL.value())
    }

    pub const fn from_square(sq: Square<WIDTH, HEIGHT>) -> Self {
        Self(1 << sq.value())
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn north(self) -> Self {
        Self((self.value() << WIDTH) & Self::ALL.value())
    }

    pub const fn south(self) -> Self {
        // shifting down cannot shift into invalid bits
        Self(self.value() >> WIDTH)
    }

    pub const fn east(self) -> Self {
        Self((self.value() << 1) & Self::ALL.value() & !Self::FILE_A.value())
    }

    pub const fn west(self) -> Self {
        // shifting left cannot shift into invalid bits
        Self((self.value() >> 1) & !Self::LAST_FILE.value())
    }

    pub const fn swap_bytes(self) -> Self {
        Self(self.value().swap_bytes())
    }

    pub const fn lsb(self) -> Square<WIDTH, HEIGHT> {
        Square::from_raw(self.value().trailing_zeros() as u16)
    }

    pub const fn msb(self) -> Square<WIDTH, HEIGHT> {
        Square::from_raw((63 - self.value().leading_zeros()) as u16)
    }

    pub const fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    pub fn poplsb(&mut self) -> Square<WIDTH, HEIGHT> {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    pub const fn any(self) -> bool {
        self.value() > 0
    }

    pub const fn empty(self) -> bool {
        self.value() == 0
    }

    pub const fn multiple(self) -> bool {
        self.value() & (self.value().wrapping_sub(1)) > 0
    }

    pub const fn one(self) -> bool {
        self.any() && !self.multiple()
    }

    pub fn set(&mut self, sq: Square<WIDTH, HEIGHT>) {
        self.0 |= 1 << sq.value();
    }

    pub fn toggle(&mut self, sq: Square<WIDTH, HEIGHT>) {
        self.0 ^= 1 << sq.value();
    }

    pub fn unset(&mut self, sq: Square<WIDTH, HEIGHT>) {
        self.0 &= !(1 << sq.value());
    }

    pub const fn has(self, sq: Square<WIDTH, HEIGHT>) -> bool {
        return ((self.value() >> sq.value()) & 1u64) > 0;
    }

    pub const fn bit_and(self, rhs: Self) -> Self {
        Self(self.value() & rhs.value())
    }

    pub const fn bit_or(self, rhs: Self) -> Self {
        Self(self.value() | rhs.value())
    }

    pub const fn bit_xor(self, rhs: Self) -> Self {
        Self(self.value() ^ rhs.value())
    }

    pub const fn bit_not(self) -> Self {
        Self(!self.value())
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitAnd for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.bit_and(rhs)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitAndAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bit_and(rhs);
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitOr for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit_or(rhs)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitOrAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bit_or(rhs);
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitXor for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bit_xor(rhs)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::BitXorAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bit_xor(rhs)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::Not for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.bit_not()
    }
}
