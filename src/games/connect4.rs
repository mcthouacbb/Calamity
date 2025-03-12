use core::fmt;
use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{Bitboard, Square, hash_combine, murmur_hash3, parse_fen_pieces},
};

pub type Connect4Square = Square<7, 6>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Connect4Move(Connect4Square);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Connect4Color {
    Red,
    Yellow,
}

impl Connect4Color {
    pub fn flip(self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Connect4State {
    pieces: [Bitboard<7, 6>; 2],
    stm: Connect4Color,
}

impl Connect4State {
    pub fn pieces(&self, c: Connect4Color) -> Bitboard<7, 6> {
        self.pieces[c as usize]
    }

    pub fn occ(&self) -> Bitboard<7, 6> {
        self.pieces[0] | self.pieces[1]
    }

    // a perfect hash is possible but I'm too lazy to do that. This should be good enough
    pub fn key(&self) -> u64 {
        hash_combine(
            murmur_hash3(self.pieces[0].value()),
            murmur_hash3(self.pieces[1].value()),
        )
    }

    fn is_loss(&self) -> bool {
        let pieces = self.pieces(self.stm.flip());
        let m = pieces & pieces.west();
        if (m & m.west().west()).any() {
            return true;
        }

        let m = pieces & pieces.south();
        if (m & m.south().south()).any() {
            return true;
        }

        let m = pieces & pieces.south().west();
        if (m & m.south().south().west().west()).any() {
            return true;
        }

        let m = pieces & pieces.south().east();
        if (m & m.south().south().east().east()).any() {
            return true;
        }
        false
    }
}

impl CopyMakeBoard for Connect4State {
    type Color = Connect4Color;
    type Piece = Connect4Color;
    type Square = Connect4Square;
    type Move = Connect4Move;
    type MoveList = ArrayVec<Connect4Move, 7>;

    fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self {
            pieces: [Bitboard::NONE; 2],
            stm: Connect4Color::Red,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let result = parse_fen_pieces(
            |sq: i32, piece: Connect4Color| {
                board.pieces[piece as usize].set(Square::from_raw(sq as u16))
            },
            parts[0],
            7,
            6,
            HashMap::from([('r', Connect4Color::Red), ('y', Connect4Color::Yellow)]),
        );
        // todo: add better error handling
        if result.is_err() {
            return None;
        }

        if parts[1] == "r" {
            board.stm = Connect4Color::Red;
        } else if parts[1] == "y" {
            board.stm = Connect4Color::Yellow;
        } else {
            // invalid stm
            return None;
        }

        Some(board)
    }

    fn startpos() -> Self {
        Self::from_fen("7/7/7/7/7/7 r").unwrap()
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        if self.pieces(Connect4Color::Red).has(sq) {
            Some(Connect4Color::Red)
        } else if self.pieces(Connect4Color::Yellow).has(sq) {
            Some(Connect4Color::Yellow)
        } else {
            None
        }
    }

    fn game_result(&self) -> GameResult {
        if self.is_loss() {
            return GameResult::LOSS;
        }
        if self.occ() == Bitboard::<7, 6>::ALL {
            return GameResult::DRAW;
        }

        GameResult::NONE
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = ArrayVec::new();
        for file in 0..7 {
            let col = self.occ() & Bitboard::<7, 6>::file(file);
            if col == Bitboard::<7, 6>::file(file) {
                continue;
            }
            let rank = if col.empty() { 0 } else { col.msb().rank() + 1 };
            moves.push(Connect4Move(Connect4Square::from_rank_file(rank, file)));
        }
        moves
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        self.pieces[self.stm as usize].set(mv.0);
        self.stm = self.stm.flip();
        true
    }
}

impl fmt::Display for Connect4State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---------\n")?;
        for rank in (0..6).rev() {
            write!(f, "|")?;
            for file in 0..7 {
                let sq = rank * 7 + file;
                match self.piece_on(Connect4Square::from_raw(sq)) {
                    Some(Connect4Color::Red) => write!(f, "r")?,
                    Some(Connect4Color::Yellow) => write!(f, "y")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "---------\n")?;
        write!(f, "stm: {:?}", self.stm)?;

        Ok(())
    }
}

pub type Connect4Board = CopyMakeWrapper<Connect4State>;
