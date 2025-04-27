use core::fmt;

use crate::util;

use std::str::FromStr;

pub type Square = util::Square<8, 8>;
pub type Bitboard = util::Bitboard<8, 8>;

pub struct SquareParseErr;

fn sq_from_str(s: &str) -> Result<Square, SquareParseErr> {
    let mut chrs = s.trim().chars();
    let Some(mut file) = chrs.next() else {
        return Err(SquareParseErr);
    };

    file = file.to_ascii_lowercase();

    let Some(rank) = chrs.next() else {
        return Err(SquareParseErr);
    };

    if file.is_ascii_alphabetic()
        && file <= 'h'
        && rank.is_ascii_digit()
        && rank >= '1'
        && rank <= '8'
    {
        return Ok(Square::from_rank_file(
            rank as u8 - '1' as u8,
            file as u8 - 'a' as u8,
        ));
    }

    Err(SquareParseErr)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value <= Self::Black as u8);
        unsafe { std::mem::transmute(value) }
    }

    pub const fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value <= Self::King as u8);
        unsafe { std::mem::transmute(value) }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl Piece {
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value <= Self::BlackKing as u8);
        unsafe { std::mem::transmute(value) }
    }

    pub const fn new(c: Color, pt: PieceType) -> Self {
        Self::from_raw((c as u8) | ((pt as u8) << 1))
    }

    pub const fn color(self) -> Color {
        Color::from_raw((self as u8) & 1)
    }

    pub const fn piece_type(self) -> PieceType {
        PieceType::from_raw((self as u8) >> 1)
    }

    pub const fn char_repr(self) -> char {
        let chars = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k'];
        chars[self as usize]
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color(), self.piece_type())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MoveKind {
    None = 0,
    Enpassant,
    Castle,
    Promotion,
}

impl MoveKind {
    pub const fn from_raw(value: u8) -> Self {
        debug_assert!(value <= MoveKind::Promotion as u8);
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    data: u16,
}

impl Move {
    pub const NULL: Move = Move { data: 0 };

    const fn new(from: Square, to: Square, kind: MoveKind, promo: u8) -> Self {
        Self {
            data: from.value()
                | ((to.value()) << 6)
                | ((kind as u16) << 12)
                | ((promo as u16) << 14),
        }
    }

    pub const fn normal(from: Square, to: Square) -> Self {
        Self::new(from, to, MoveKind::None, 0)
    }

    pub const fn castle(from: Square, to: Square) -> Self {
        Self::new(from, to, MoveKind::Castle, 0)
    }

    pub const fn enpassant(from: Square, to: Square) -> Self {
        Self::new(from, to, MoveKind::Enpassant, 0)
    }

    pub const fn promo(from: Square, to: Square, promo: PieceType) -> Self {
        Self::new(
            from,
            to,
            MoveKind::Promotion,
            promo as u8 - PieceType::Knight as u8,
        )
    }

    pub const fn from_sq(&self) -> Square {
        Square::from_raw(self.data & 63)
    }

    pub const fn to_sq(&self) -> Square {
        Square::from_raw((self.data >> 6) & 63)
    }

    pub const fn kind(&self) -> MoveKind {
        MoveKind::from_raw(((self.data >> 12) & 3) as u8)
    }

    pub const fn promo_piece(&self) -> PieceType {
        PieceType::from_raw(((self.data >> 14) + PieceType::Knight as u16) as u8)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kind() == MoveKind::Castle {
            let offset = if self.to_sq() > self.from_sq() {
                2
            } else {
                -2
            };
            return write!(f, "{}{}", self.from_sq(), self.from_sq() + offset);
        }
        write!(f, "{}{}", self.from_sq(), self.to_sq())?;
        if self.kind() == MoveKind::Promotion {
            write!(
                f,
                "{}",
                Piece::new(Color::Black, self.promo_piece()).char_repr()
            )?;
        }
        Ok(())
    }
}

pub struct MoveParseErr;

impl FromStr for Move {
    type Err = MoveParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_str, rest) = s.split_at(2);
        let (to_str, promo) = rest.split_at(2);

        let from_sq = sq_from_str(from_str).map_err(|_| MoveParseErr)?;
        let to_sq = sq_from_str(to_str).map_err(|_| MoveParseErr)?;

        if promo.starts_with('n') || promo.starts_with('N') {
            return Ok(Move::promo(from_sq, to_sq, PieceType::Knight));
        } else if promo.starts_with('b') || promo.starts_with('B') {
            return Ok(Move::promo(from_sq, to_sq, PieceType::Bishop));
        } else if promo.starts_with('r') || promo.starts_with('R') {
            return Ok(Move::promo(from_sq, to_sq, PieceType::Rook));
        } else if promo.starts_with('q') || promo.starts_with('Q') {
            return Ok(Move::promo(from_sq, to_sq, PieceType::Queen));
        }

        Ok(Move::normal(from_sq, to_sq))
    }
}
