use std::ops;

use crate::util::Bitboard;

use super::square::Connect4Square;

// connect 4 bitboards are in a different layout for efficiency purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Connect4Bitboard(Bitboard<7, 7>);

impl Connect4Bitboard {
    pub const NONE: Self = Self(Bitboard::NONE);
    pub const VALID: Self = Self::row(0)
        .bit_or(Self::row(1))
        .bit_or(Self::row(2))
        .bit_or(Self::row(3))
        .bit_or(Self::row(4))
        .bit_or(Self::row(5));

    pub const fn from_raw(value: u64) -> Self {
        Self(Bitboard::from_raw(value))
    }

    pub const fn column(file: u8) -> Self {
        Self(Bitboard::rank(file))
    }

    pub const fn row(rank: u8) -> Self {
        Self(Bitboard::file(rank))
    }

    pub const fn from_square(sq: Connect4Square) -> Self {
        Self(Bitboard::from_square(sq.to_util_sq()))
    }

    pub const fn value(self) -> u64 {
        self.0.value()
    }

    // does not prevent bits from entering the invalid area above the board
    pub const fn north(self) -> Self {
        Self(self.0.west())
    }

    // does not prevent bits from entering the invalid area above the board
    pub const fn south(self) -> Self {
        Self(self.0.east())
    }

    pub const fn east(self) -> Self {
        Self(self.0.north())
    }

    pub const fn west(self) -> Self {
        // shifting left cannot shift into invalid bits
        Self(self.0.south())
    }

    pub const fn lsb(self) -> Connect4Square {
        Connect4Square::from_util_sq(self.0.lsb())
    }

    pub const fn msb(self) -> Connect4Square {
        Connect4Square::from_util_sq(self.0.msb())
    }

    pub const fn popcount(self) -> u32 {
        self.0.popcount()
    }

    pub fn poplsb(&mut self) -> Connect4Square {
        Connect4Square::from_util_sq(self.0.poplsb())
    }

    pub const fn any(self) -> bool {
        self.0.any()
    }

    pub const fn empty(self) -> bool {
        self.0.empty()
    }

    pub const fn multiple(self) -> bool {
        self.0.multiple()
    }

    pub const fn one(self) -> bool {
        self.0.one()
    }

    pub fn set(&mut self, sq: Connect4Square) {
        self.0.set(sq.to_util_sq());
    }

    pub fn toggle(&mut self, sq: Connect4Square) {
        self.0.toggle(sq.to_util_sq());
    }

    pub fn unset(&mut self, sq: Connect4Square) {
        self.0.unset(sq.to_util_sq());
    }

    pub const fn has(self, sq: Connect4Square) -> bool {
        self.0.has(sq.to_util_sq())
    }

    pub const fn bit_and(self, rhs: Self) -> Self {
        Self(self.0.bit_and(rhs.0))
    }

    pub const fn bit_or(self, rhs: Self) -> Self {
        Self(self.0.bit_or(rhs.0))
    }

    pub const fn bit_xor(self, rhs: Self) -> Self {
        Self(self.0.bit_xor(rhs.0))
    }

    pub const fn bit_not(self) -> Self {
        Self(self.0.bit_not())
    }
}

impl ops::BitAnd for Connect4Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.bit_and(rhs)
    }
}

impl ops::BitAndAssign for Connect4Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bit_and(rhs);
    }
}

impl ops::BitOr for Connect4Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit_or(rhs)
    }
}

impl ops::BitOrAssign for Connect4Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bit_or(rhs);
    }
}

impl ops::BitXor for Connect4Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bit_xor(rhs)
    }
}

impl ops::BitXorAssign for Connect4Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bit_xor(rhs)
    }
}

impl ops::Not for Connect4Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.bit_not()
    }
}
