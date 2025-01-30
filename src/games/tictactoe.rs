use core::fmt;

use arrayvec::ArrayVec;

use crate::board::{Board, GameResult};
use crate::util::Square;

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
pub struct TicTacToeBoard {
    squares: [Option<Piece>; 9],
    stack: ArrayVec<TicTacToeMove, 9>,
    stm: TicTacToeColor,
}

impl Board for TicTacToeBoard {
    type Color = TicTacToeColor;
    // only one type of piece
    type PieceType = ();
    type Piece = TicTacToeColor;
    type Square = TicTacToeSquare;
    type Move = TicTacToeMove;
    type MoveList = ArrayVec<TicTacToeMove, 9>;

    fn startpos() -> Self {
        TicTacToeBoard {
            squares: [None; 9],
            stack: ArrayVec::new(),
            stm: TicTacToeColor::X,
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
        self.stack.push(mv);
        self.squares[mv.to_sq().value() as usize] = Some(self.stm);
        self.stm = self.stm.flip();
        true
    }

    fn unmake_move(&mut self) {
        let prev_move = self.stack.pop().unwrap();
        self.squares[prev_move.to_sq().value() as usize] = None;
        self.stm = self.stm.flip();
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
