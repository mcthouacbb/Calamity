use core::fmt;
use std::{collections::HashMap, result};

use arrayvec::ArrayVec;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{Bitboard, Square, parse_fen_pieces},
};

pub type AtaxxSquare = Square<7, 7>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AtaxxMove {
    data: u16,
}

impl AtaxxMove {
    pub fn null() -> Self {
        Self { data: 0 }
    }
    pub fn single(sq: AtaxxSquare) -> Self {
        Self {
            data: sq.value() | (63 << 6),
        }
    }

    pub fn double(from: AtaxxSquare, to: AtaxxSquare) -> Self {
        Self {
            data: to.value() | (from.value() << 6),
        }
    }

    fn from_sq_raw(&self) -> u16 {
        self.data >> 6
    }

    fn to_sq_raw(&self) -> u16 {
        self.data & 63
    }

    pub fn from_sq(&self) -> AtaxxSquare {
        AtaxxSquare::from_raw(self.from_sq_raw())
    }

    pub fn to_sq(&self) -> AtaxxSquare {
        AtaxxSquare::from_raw(self.to_sq_raw())
    }

    pub fn is_null(&self) -> bool {
        self.data == 0
    }

    pub fn is_single(&self) -> bool {
        self.from_sq_raw() == 63
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtaxxColor {
    Black,
    White,
}

impl AtaxxColor {
    pub fn flip(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtaxxPiece {
    Black,
    White,
    Blocker,
}

const fn single_moves(pieces: Bitboard<7, 7>) -> Bitboard<7, 7> {
    let we = pieces.west().bit_or(pieces.east());
    let ns = pieces.north().bit_or(pieces.south());
    we.bit_or(ns.west()).bit_or(ns.east()).bit_or(ns)
}

const ADJACENT_SQUARES: [Bitboard<7, 7>; 49] = {
    let mut result = [Bitboard::NONE; 49];
    let mut i = 0;
    while i < 49 {
        let sq_bb: Bitboard<7, 7> = Bitboard::from_square(Square::from_raw(i as u16));
        result[i] = single_moves(sq_bb);
        i += 1;
    }
    result
};

const DOUBLE_MOVES: [Bitboard<7, 7>; 49] = {
    let mut result = [Bitboard::NONE; 49];
    let mut i = 0;
    while i < 49 {
        let sq_bb: Bitboard<7, 7> = Bitboard::from_square(Square::from_raw(i as u16));
        let nn = sq_bb.north().north();
        let ss = sq_bb.south().south();
        let mut top_bottom = nn.bit_or(ss);
        top_bottom = top_bottom.bit_or(top_bottom.west());
        top_bottom = top_bottom.bit_or(top_bottom.west());
        top_bottom = top_bottom.bit_or(top_bottom.east());
        top_bottom = top_bottom.bit_or(top_bottom.east());

        let ee = sq_bb.east().east();
        let ww = sq_bb.west().west();
        let mut left_right = ee.bit_or(ww);
        left_right = left_right.bit_or(left_right.north());
        left_right = left_right.bit_or(left_right.south());

        result[i] = top_bottom.bit_or(left_right);
        i += 1;
    }
    result
};

#[derive(Debug, Clone)]
pub struct AtaxxState {
    pieces: [Bitboard<7, 7>; 2],
    blockers: Bitboard<7, 7>,
    half_move_clock: u8,
    stm: AtaxxColor,
}

impl AtaxxState {
    pub fn pieces(&self, c: AtaxxColor) -> Bitboard<7, 7> {
        self.pieces[c as usize]
    }

    pub fn pieces_mut(&mut self, c: AtaxxColor) -> &mut Bitboard<7, 7> {
        &mut self.pieces[c as usize]
    }

    pub fn occ(&self) -> Bitboard<7, 7> {
        self.pieces[0] | self.pieces[1] | self.blockers
    }
}

impl CopyMakeBoard for AtaxxState {
    type Color = AtaxxColor;
    type Piece = AtaxxPiece;
    type Square = AtaxxSquare;
    type Move = AtaxxMove;
    type MoveList = ArrayVec<AtaxxMove, 256>;

    fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self {
            pieces: [Bitboard::NONE; 2],
            blockers: Bitboard::NONE,
            stm: AtaxxColor::White,
            half_move_clock: 0,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let result = parse_fen_pieces(
            |sq: i32, piece: AtaxxPiece| {
                if piece == AtaxxPiece::Black {
                    board
                        .pieces_mut(AtaxxColor::Black)
                        .set(Square::from_raw(sq as u16));
                } else if piece == AtaxxPiece::White {
                    board
                        .pieces_mut(AtaxxColor::White)
                        .set(Square::from_raw(sq as u16));
                } else if piece == AtaxxPiece::Blocker {
                    board.blockers.set(Square::from_raw(sq as u16));
                }
            },
            parts[0],
            7,
            7,
            HashMap::from([
                ('X', AtaxxPiece::Black),
                ('x', AtaxxPiece::Black),
                ('B', AtaxxPiece::Black),
                ('b', AtaxxPiece::Black),
                ('O', AtaxxPiece::White),
                ('o', AtaxxPiece::White),
                ('W', AtaxxPiece::White),
                ('w', AtaxxPiece::White),
                ('-', AtaxxPiece::Blocker),
            ]),
        );
        // todo: add better error handling
        if result.is_err() {
            return None;
        }

        if parts[1] == "X" || parts[1] == "x" || parts[1] == "B" || parts[1] == "b" {
            board.stm = AtaxxColor::Black;
        } else if parts[1] == "O" || parts[1] == "o" || parts[1] == "W" || parts[1] == "w" {
            board.stm = AtaxxColor::White;
        } else {
            // invalid stm
            return None;
        }

        let Ok(hmc) = parts[2].parse() else {
            return None;
        };
        board.half_move_clock = hmc;

        Some(board)
    }

    fn startpos() -> Self {
        Self::from_fen("x5o/7/7/7/7/7/o5x x 0 1").unwrap()
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        if self.pieces(AtaxxColor::Black).has(sq) {
            Some(AtaxxPiece::Black)
        } else if self.pieces(AtaxxColor::White).has(sq) {
            Some(AtaxxPiece::White)
        } else if self.blockers.has(sq) {
            Some(AtaxxPiece::Blocker)
        } else {
            None
        }
    }

    fn game_result(&self) -> GameResult {
        if self.half_move_clock >= 100 {
            return GameResult::DRAW;
        }
        if self.pieces(self.stm).empty() {
            return GameResult::LOSS;
        }
        if self.pieces(self.stm.flip()).empty() {
            return GameResult::WIN;
        }
        let possible_moves = !self.occ() & single_moves(single_moves(self.occ()));
        if possible_moves.empty() {
            let score = self.pieces(self.stm).popcount() as i32
                - self.pieces(self.stm.flip()).popcount() as i32;
            if score > 0 {
                return GameResult::WIN;
            } else if score < 0 {
                return GameResult::LOSS;
            } else {
                return GameResult::DRAW;
            }
        }
        GameResult::NONE
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = ArrayVec::new();
        if self.game_result() != GameResult::NONE {
            return moves;
        }

        let move_mask = !self.occ();
        let mut singles = single_moves(self.pieces(self.stm)) & move_mask;
        while singles.any() {
            moves.push(AtaxxMove::single(singles.poplsb()));
        }

        let mut stm_pieces = self.pieces(self.stm);
        while stm_pieces.any() {
            let piece = stm_pieces.poplsb();
            let mut doubles = DOUBLE_MOVES[piece.value() as usize] & move_mask;
            while doubles.any() {
                moves.push(AtaxxMove::double(piece, doubles.poplsb()));
            }
        }

        if moves.len() == 0 {
            moves.push(AtaxxMove::null());
        }

        moves
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        if mv.is_null() {
            self.stm = self.stm.flip();
            return true;
        }

        self.half_move_clock += 1;

        if mv.is_single() {
            self.half_move_clock = 0;
        } else {
            self.pieces[self.stm as usize].toggle(mv.from_sq());
        }

        self.pieces[self.stm as usize].set(mv.to_sq());

        let adj_opps = self.pieces(self.stm.flip()) & ADJACENT_SQUARES[mv.to_sq().value() as usize];
        self.pieces[self.stm as usize] |= adj_opps;
        self.pieces[self.stm.flip() as usize] ^= adj_opps;

        self.stm = self.stm.flip();

        true
    }
}

impl fmt::Display for AtaxxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---------\n")?;
        for rank in (0..6).rev() {
            write!(f, "|")?;
            for file in 0..7 {
                let sq = rank * 7 + file;
                match self.piece_on(AtaxxSquare::from_raw(sq)) {
                    Some(AtaxxPiece::Black) => write!(f, "x")?,
                    Some(AtaxxPiece::White) => write!(f, "o")?,
                    Some(AtaxxPiece::Blocker) => write!(f, "-")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "---------\n")?;
        write!(f, "stm: {:?}", self.stm)?;
        write!(f, "half move clock: {}", self.half_move_clock)?;

        Ok(())
    }
}

pub type AtaxxBoard = CopyMakeWrapper<AtaxxState>;
