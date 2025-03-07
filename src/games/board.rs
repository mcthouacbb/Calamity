use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
    NONE,
    WIN,
    DRAW,
    LOSS,
}

pub trait Board: Sized + Clone {
    type Move: Debug + Copy + Clone + PartialEq + Eq;
    type Square: Copy + Clone + PartialEq + Eq + PartialEq + Ord;
    type PieceType: Copy + Clone;
    type Color: Copy + Clone;
    type Piece: Copy + Clone;
    type MoveList: IntoIterator<Item = Self::Move>;

    fn startpos() -> Self;
    // todo: make this a Result instead of Option
    fn from_fen(fen: &str) -> Option<Self>;

    fn game_result(&self) -> GameResult;
    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece>;
    fn gen_moves(&self) -> Self::MoveList;

    fn make_move(&mut self, mv: Self::Move) -> bool;
    fn unmake_move(&mut self);
}

pub trait CopyMakeBoard: Sized + Clone {
    type Move: Debug + Copy + Clone + PartialEq + Eq;
    type Square: Copy + Clone + PartialEq + Eq + PartialEq + Ord;
    type PieceType: Copy + Clone;
    type Color: Copy + Clone;
    type Piece: Copy + Clone;
    type MoveList: IntoIterator<Item = Self::Move>;

    fn startpos() -> Self;
    // todo: make this a Result instead of Option
    fn from_fen(fen: &str) -> Option<Self>;

    fn game_result(&self) -> GameResult;
    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece>;
    fn gen_moves(&self) -> Self::MoveList;

    fn make_move(&mut self, mv: Self::Move) -> bool;
}

#[derive(Debug, Clone)]
pub struct CopyMakeWrapper<T>
where
    T: CopyMakeBoard,
{
    stack: Vec<T>,
}

impl<T> CopyMakeWrapper<T>
where
    T: CopyMakeBoard,
{
    pub fn curr_state(&self) -> &T {
        self.stack.last().unwrap()
    }

    fn curr_state_mut(&mut self) -> &mut T {
        self.stack.last_mut().unwrap()
    }
}

impl<T> Board for CopyMakeWrapper<T>
where
    T: CopyMakeBoard,
{
    type Move = T::Move;
    type Square = T::Square;
    type PieceType = T::PieceType;
    type Color = T::Color;
    type Piece = T::Piece;
    type MoveList = T::MoveList;

    fn startpos() -> Self {
        Self {
            stack: vec![T::startpos()],
        }
    }

    fn from_fen(fen: &str) -> Option<Self> {
        let result = T::from_fen(fen);
        match result {
            Some(board) => Some(Self { stack: vec![board] }),
            _ => None,
        }
    }

    fn game_result(&self) -> GameResult {
        self.curr_state().game_result()
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        self.curr_state().piece_on(sq)
    }

    fn gen_moves(&self) -> Self::MoveList {
        self.curr_state().gen_moves()
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        self.stack.push(self.curr_state().clone());
        if self.curr_state_mut().make_move(mv) {
            true
        } else {
            self.stack.pop();
            false
        }
    }

    fn unmake_move(&mut self) {
        assert!(self.stack.len() > 1);
        self.stack.pop();
    }
}

impl<T> fmt::Display for CopyMakeWrapper<T>
where
    T: CopyMakeBoard + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.curr_state())
    }
}
