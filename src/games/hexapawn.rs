use core::fmt;
use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{
    games::board::{CopyMakeBoard, CopyMakeWrapper, GameResult},
    util::{parse_fen_pieces, Bitboard, Square},
};

pub type HexapawnSquare = Square<3, 3>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HexapawnMove {
    from: HexapawnSquare,
    to: HexapawnSquare,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexapawnColor {
    White,
    Black,
}

impl HexapawnColor {
    pub fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HexapawnState {
    pawns: [Bitboard<3, 3>; 2],
    stm: HexapawnColor,
}

impl CopyMakeBoard for HexapawnState {
    type Color = HexapawnColor;
    // only one type of piece
    type PieceType = ();
    type Piece = HexapawnColor;
    type Square = HexapawnSquare;
    type Move = HexapawnMove;
    type MoveList = ArrayVec<HexapawnMove, 9>;

    fn startpos() -> Self {
        Self::from_fen("ppp/3/PPP w").unwrap()
    }

    fn from_fen(fen: &str) -> Option<Self> {
        let mut board = HexapawnState {
            pawns: [Bitboard::NONE; 2],
            stm: HexapawnColor::White,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let result = parse_fen_pieces(
            |sq: i32, piece: HexapawnColor| {
                board.pawns[piece as usize].set(Square::from_raw(sq as u16))
            },
            parts[0],
            3,
            3,
            HashMap::from([('P', HexapawnColor::White), ('p', HexapawnColor::Black)]),
        );
        // todo: add better error handling
        if result.is_err() {
            return None;
        }

        if parts[1] == "w" {
            board.stm = HexapawnColor::White;
        } else if parts[1] == "b" {
            board.stm = HexapawnColor::Black;
        } else {
            // invalid stm
            return None;
        }

        Some(board)
    }

    fn game_result(&self) -> GameResult {
        // loss if no legal moves
        if self.gen_moves().is_empty() {
            return GameResult::LOSS;
        }

        let opp_back_rank: Bitboard<3, 3> = if self.stm == HexapawnColor::White {
            Bitboard::RANK_0
        } else {
            Bitboard::LAST_RANK
        };

        // loss if opponent just reached back rank
        if (opp_back_rank & self.pawns[self.stm.flip() as usize]).any() {
            return GameResult::LOSS;
        }

        GameResult::NONE
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        if self.pawns[HexapawnColor::White as usize].has(sq) {
            Some(HexapawnColor::White)
        } else if self.pawns[HexapawnColor::Black as usize].has(sq) {
            Some(HexapawnColor::Black)
        } else {
            None
        }
    }

    fn gen_moves(&self) -> Self::MoveList {
        let our_pawns = self.pawns[self.stm as usize];
        let their_pawns = self.pawns[self.stm.flip() as usize];
        let mut moves = ArrayVec::new();
        let push_offset = if self.stm == HexapawnColor::White {
            3
        } else {
            -3
        };

        let mut pushes = if self.stm == HexapawnColor::White {
            our_pawns.north()
        } else {
            our_pawns.south()
        };
        let mut west_caps = pushes.west() & their_pawns;
        let mut east_caps = pushes.east() & their_pawns;
        pushes &= !(our_pawns | their_pawns);

        while pushes.any() {
            let to_sq = pushes.poplsb();
            moves.push(HexapawnMove {
                from: to_sq - push_offset,
                to: to_sq,
            });
        }

        while west_caps.any() {
            let to_sq = west_caps.poplsb();
            moves.push(HexapawnMove {
                from: to_sq - push_offset + 1,
                to: to_sq,
            });
        }

        while east_caps.any() {
            let to_sq = east_caps.poplsb();
            moves.push(HexapawnMove {
                from: to_sq - push_offset - 1,
                to: to_sq,
            });
        }

        moves
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        self.pawns[self.stm.flip() as usize].unset(mv.to);
        self.pawns[self.stm as usize] ^=
            Bitboard::from_square(mv.from) | Bitboard::from_square(mv.to);
        self.stm = self.stm.flip();
        true
    }
}

impl fmt::Display for HexapawnState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-----\n")?;
        for rank in (0..3).rev() {
            write!(f, "|")?;
            for file in 0..3 {
                let sq = rank * 3 + file;
                match self.piece_on(HexapawnSquare::from_raw(sq)) {
                    Some(HexapawnColor::White) => write!(f, "P")?,
                    Some(HexapawnColor::Black) => write!(f, "p")?,
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

pub type HexapawnBoard = CopyMakeWrapper<HexapawnState>;
