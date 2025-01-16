use core::fmt;

use arrayvec::ArrayVec;

use crate::board::{Board, GameResult};

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1,
    A2, B2, C2,
    A3, B3, C3,
}

impl Square {
    pub fn from_raw(raw: u8) -> Self {
        unsafe { std::mem::transmute(raw) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(Square);

impl Move {
    pub fn new(sq: Square) -> Self {
        Self(sq)
    }

    pub fn to_sq(self) -> Square {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Color {
    X,
    O,
}

impl Color {
    pub fn flip(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

type Piece = Color;

#[derive(Debug, Clone)]
pub struct TicTacToeBoard {
    squares: [Option<Piece>; 9],
    stm: Color,
}

impl Board for TicTacToeBoard {
    type Color = Color;
    // lol
    type PieceType = ();
    type Piece = Color;
    type Square = Square;
    type Move = Move;
    type MoveList = ArrayVec<Move, 9>;

    fn startpos() -> Self {
        TicTacToeBoard {
            squares: [None; 9],
            stm: Color::X,
        }
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
        GameResult::NONE
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        self.squares[sq as usize]
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = Self::MoveList::new();
        for i in 0..9 {
            if self.squares[i].is_none() {
                moves.push(Move::new(Square::from_raw(i as u8)));
            }
        }
        moves
    }

    fn make_move(&self, mv: Self::Move) -> Option<Self> {
        let mut new_board = self.clone();
        new_board.squares[mv.0 as usize] = Some(self.stm);
        new_board.stm = self.stm.flip();
        Some(new_board)
    }
}

impl fmt::Display for TicTacToeBoard {
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
