use core::fmt;
use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{Square, parse_fen_pieces},
};

pub type TicTacToeSquare = Square<3, 3>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TicTacToeMove(TicTacToeSquare);

impl TicTacToeMove {
    pub fn new(sq: TicTacToeSquare) -> Self {
        Self(sq)
    }

    pub fn to_sq(self) -> TicTacToeSquare {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TicTacToeColor {
    X,
    O,
}

impl TicTacToeColor {
    pub fn flip(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

type Piece = TicTacToeColor;

#[derive(Debug, Clone)]
pub struct TicTacToeState {
    squares: [Option<Piece>; 9],
    stm: TicTacToeColor,
}

impl CopyMakeBoard for TicTacToeState {
    type Color = TicTacToeColor;
    type Piece = TicTacToeColor;
    type Square = TicTacToeSquare;
    type Move = TicTacToeMove;
    type MoveList = ArrayVec<TicTacToeMove, 9>;

    fn startpos() -> Self {
        Self::from_fen("3/3/3 X").unwrap()
    }

    fn from_fen(fen: &str) -> Option<Self> {
        let mut board = TicTacToeState {
            squares: [None; 9],
            stm: TicTacToeColor::X,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let result = parse_fen_pieces(
            |sq: i32, piece: TicTacToeColor| board.squares[sq as usize] = Some(piece),
            parts[0],
            3,
            3,
            HashMap::from([('X', TicTacToeColor::X), ('O', TicTacToeColor::O)]),
        );
        // todo: add better error handling
        if result.is_err() {
            return None;
        }

        if parts[1] == "X" {
            board.stm = TicTacToeColor::X;
        } else if parts[1] == "O" {
            board.stm = TicTacToeColor::O;
        } else {
            // invalid stm
            return None;
        }

        Some(board)
    }

    fn game_result(&self) -> GameResult {
        // only opponent could have won last turn
        let opp = Some(self.stm.flip());
        for i in 0..3 {
            if self.squares[3 * i] == opp
                && self.squares[3 * i + 1] == opp
                && self.squares[3 * i + 2] == opp
            {
                return GameResult::LOSS;
            }

            if self.squares[i] == opp && self.squares[i + 3] == opp && self.squares[i + 6] == opp {
                return GameResult::LOSS;
            }
        }
        if self.squares[0] == opp && self.squares[4] == opp && self.squares[8] == opp {
            return GameResult::LOSS;
        }
        if self.squares[2] == opp && self.squares[4] == opp && self.squares[6] == opp {
            return GameResult::LOSS;
        }
        for i in 0..9 {
            if self.squares[i].is_none() {
                return GameResult::NONE;
            }
        }
        GameResult::DRAW
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        self.squares[sq.value() as usize]
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = Self::MoveList::new();
        for i in 0..9 {
            if self.squares[i].is_none() {
                moves.push(TicTacToeMove::new(Square::from_raw(i as u16)));
            }
        }
        moves
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        self.squares[mv.to_sq().value() as usize] = Some(self.stm);
        self.stm = self.stm.flip();
        true
    }
}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-----\n")?;
        for rank in (0..3).rev() {
            write!(f, "|")?;
            for file in 0..3 {
                let sq = rank * 3 + file;
                match self.squares[sq] {
                    Some(c) => write!(f, "{:?}", c)?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "-----\n")?;
        write!(f, "stm: {:?}", self.stm)?;

        Ok(())
    }
}

pub type TicTacToeBoard = CopyMakeWrapper<TicTacToeState>;
