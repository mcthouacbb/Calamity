use core::fmt;
use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{parse_fen_pieces, Bitboard, Square},
};

pub type Connect4Square = Square<7, 6>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Connect4Move {
    from: Connect4Square,
    to: Connect4Square,
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
    pieces: [Bitboard<7, 6>; 2],
    stm: Connect4Color,
}


