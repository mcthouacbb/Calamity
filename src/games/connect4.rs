mod bitboard;
mod square;

use core::fmt;
use std::collections::HashMap;

use arrayvec::ArrayVec;
use bitboard::Connect4Bitboard;
use square::Connect4Square;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{Square, hash_combine, murmur_hash3, parse_fen_pieces},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Connect4Move(Connect4Square);

impl Connect4Move {
    pub fn sq(&self) -> Connect4Square {
        self.0
    }
}

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
    pieces: [Connect4Bitboard; 2],
    stm: Connect4Color,
}

impl Connect4State {
    pub fn pieces(&self, c: Connect4Color) -> Connect4Bitboard {
        self.pieces[c as usize]
    }

    pub fn stm(&self) -> Connect4Color {
        self.stm
    }

    pub fn occ(&self) -> Connect4Bitboard {
        self.pieces[0] | self.pieces[1]
    }

    pub fn above_pieces(&self) -> Connect4Bitboard {
        Connect4Bitboard::from_raw(self.occ().value() + Connect4Bitboard::row(0).value())
    }

    pub fn move_locations(&self) -> Connect4Bitboard {
        self.above_pieces() & Connect4Bitboard::VALID
    }

    // a perfect hash is possible but I'm too lazy to do that. This should be good enough
    pub fn key(&self) -> u64 {
        murmur_hash3((self.above_pieces() | self.pieces(Connect4Color::Red)).value())
    }

    pub fn threats(&self) -> Connect4Bitboard {
        Self::compute_threats(self.pieces(self.stm()), self.occ())
    }

    fn compute_threats(pieces: Connect4Bitboard, occ: Connect4Bitboard) -> Connect4Bitboard {
        let pieces = pieces.value();
        let vertical = (pieces << 1) & (pieces << 2) & (pieces << 3);

        let mut tmp = (pieces << 7) & (pieces << 2 * 7);
        let mut horizontal = tmp & (pieces << 3 * 7);
        horizontal |= tmp & (pieces >> 7);
        tmp = (pieces >> 7) & (pieces >> 2 * 7);
        horizontal |= tmp & (pieces >> 3 * 7);
        horizontal |= tmp & (pieces << 7);

        tmp = (pieces << 6) & (pieces << 2 * 6);
        let mut diag1 = tmp & (pieces << 3 * 6);
        diag1 |= tmp & (pieces >> 6);
        tmp = (pieces >> 6) & (pieces >> 2 * 6);
        diag1 |= tmp & (pieces >> 3 * 6);
        diag1 |= tmp & (pieces << 6);

        tmp = (pieces << 8) & (pieces << 2 * 8);
        let mut diag2 = tmp & (pieces << 3 * 8);
        diag2 |= tmp & (pieces >> 8);
        tmp = (pieces >> 8) & (pieces >> 2 * 8);
        diag2 |= tmp & (pieces >> 3 * 8);
        diag2 |= tmp & (pieces << 8);

        let result = (vertical | horizontal | diag1 | diag2) & Connect4Bitboard::VALID.value();
        Connect4Bitboard::from_raw(result) & !occ
    }

    fn is_loss(&self) -> bool {
        let pieces = self.pieces(self.stm.flip()).value();
        let m = pieces & (pieces >> 7);
        if (m & (m >> 14)) != 0 {
            return true;
        }

        let m = pieces & (pieces >> 1);
        if (m & (m >> 2)) != 0 {
            return true;
        }

        let m = pieces & (pieces >> 6);
        if (m & (m >> 12)) != 0 {
            return true;
        }

        let m = pieces & (pieces >> 8);
        if (m & (m >> 16)) != 0 {
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
            pieces: [Connect4Bitboard::NONE; 2],
            stm: Connect4Color::Red,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let result = parse_fen_pieces(
            |sq: i32, piece: Connect4Color| {
                let conventional_sq = Square::<7, 6>::from_raw(sq as u16);
                board.pieces[piece as usize].set(Connect4Square::from_row_column(
                    conventional_sq.rank(),
                    conventional_sq.file(),
                ));
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
        if self.occ() == Connect4Bitboard::VALID {
            return GameResult::DRAW;
        }

        GameResult::NONE
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = ArrayVec::new();
        let mut move_locations = self.move_locations();
        while move_locations.any() {
            moves.push(Connect4Move(move_locations.poplsb()));
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
        for row in (0..6).rev() {
            write!(f, "|")?;
            for column in 0..7 {
                match self.piece_on(Connect4Square::from_row_column(row, column)) {
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
