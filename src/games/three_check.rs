pub mod attacks;
mod castling_rooks;
mod movegen;
pub mod types;

use core::fmt;

use castling_rooks::CastlingRooks;
pub use movegen::MoveList;
pub use types::{Bitboard, Color, Move, MoveKind, Piece, PieceType, Square};

use super::board::{CopyMakeBoard, CopyMakeWrapper, GameResult};

#[derive(Debug, Clone)]
pub struct ThreeCheckState {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    squares: [Option<Piece>; 64],
    checkers: Bitboard,
    diag_pinned: Bitboard,
    hv_pinned: Bitboard,
    castling_rooks: CastlingRooks,
    stm: Color,
    ep_square: Option<Square>,
    half_move_clock: u8,
    check_count: [u8; 2], // zkey: ZobristKey,
}

impl ThreeCheckState {
    const STARTPOS_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3+3 0 1";

    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self::empty();

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 7 {
            return None;
        }

        let mut curr = Square::from_rank_file(7, 0).value() as i32;
        let mut rows = 0;
        for c in parts[0].chars() {
            match c {
                '1'..='9' => {
                    curr += c as i32 - '0' as i32;
                    // cancel out extra += 1 at the end
                    curr -= 1;
                }
                'P' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::Pawn),
                ),
                'N' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::Knight),
                ),
                'B' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::Bishop),
                ),
                'R' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::Rook),
                ),
                'Q' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::Queen),
                ),
                'K' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::White, PieceType::King),
                ),
                'p' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::Pawn),
                ),
                'n' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::Knight),
                ),
                'b' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::Bishop),
                ),
                'r' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::Rook),
                ),
                'q' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::Queen),
                ),
                'k' => board.add_piece(
                    Square::from_raw(curr as u16),
                    Piece::new(Color::Black, PieceType::King),
                ),
                '/' => {
                    if curr != 64 - rows * 8 {
                        return None;
                    }
                    rows += 1;
                    curr -= 16;
                    // cancel out extra += 1 at the end
                    curr -= 1;
                }
                _ => return None,
            };
            curr += 1;
        }

        if curr != 8 || rows != 7 {
            return None;
        }

        if parts[1].len() != 1 {
            return None;
        }

        let stm = parts[1].chars().next().unwrap();
        board.stm = if stm == 'w' {
            Color::White
        } else if stm == 'b' {
            Color::Black
        } else {
            return None;
        };

        if parts[2].len() == 0 || parts[2].len() > 4 {
            return None;
        }

        for c in parts[2].chars() {
            match c {
                'K' => {
                    board.castling_rooks.color_mut(Color::White).king_side =
                        Some(Square::from_rank_file(0, 7));
                }
                'Q' => {
                    board.castling_rooks.color_mut(Color::White).queen_side =
                        Some(Square::from_rank_file(0, 0));
                }
                'k' => {
                    board.castling_rooks.color_mut(Color::Black).king_side =
                        Some(Square::from_rank_file(7, 7));
                }
                'q' => {
                    board.castling_rooks.color_mut(Color::Black).queen_side =
                        Some(Square::from_rank_file(7, 0));
                }
                '-' => {
                    if parts[2].len() != 1 {
                        return None;
                    }
                }
                _ => {
                    return None;
                }
            }
        }

        if parts[3].len() == 0 || parts[3].len() > 2 {
            return None;
        }

        if parts[3].len() == 1 && parts[3].chars().next().unwrap() != '-' {
            return None;
        }

        if parts[3].len() == 2 {
            // trash
            let mut iter = parts[3].chars();
            let file = iter.next().unwrap();
            let rank = iter.next().unwrap();
            board.ep_square = Some(Square::from_rank_file(
                rank as u8 - '1' as u8,
                file as u8 - 'a' as u8,
            ));
        }

        let checks: Vec<&str> = parts[4].split('+').collect();
        if checks.len() != 2 {
            return None;
        }
        match checks[0].parse::<u8>() {
            Ok(n) => {
                board.check_count[0] = 3 - n;
            }
            Err(_) => {
                return None;
            }
        }
        match checks[1].parse::<u8>() {
            Ok(n) => {
                board.check_count[1] = 3 - n;
            }
            Err(_) => {
                return None;
            }
        }

        match parts[5].parse::<u8>() {
            Ok(n) => {
                board.half_move_clock = n;
            }
            Err(_) => {
                return None;
            }
        }
        if board.half_move_clock > 100 {
            return None;
        }

        board.update_check_info();
        /*board.zkey.toggle_castle_rights(board.castling_rooks());
        if let Some(ep_square) = board.ep_square() {
            board.zkey.toggle_ep_square(ep_square);
        }

        if board.stm() == Color::Black {
            board.zkey.toggle_stm();
        }*/

        Some(board)
    }

    pub fn startpos() -> Self {
        Self::from_fen(Self::STARTPOS_FEN).unwrap()
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut last_file = -1;
            for file in 0..8 {
                let sq = Square::from_rank_file(rank, file);
                match self.piece_at(sq) {
                    Some(piece) => {
                        let diff = sq.value() as i32 - rank as i32 * 8 - last_file - 1;
                        if diff > 0 {
                            fen.push(std::char::from_digit(diff as u32, 10).unwrap());
                        }
                        fen.push(piece.char_repr());
                        last_file = sq.value() as i32 - rank as i32 * 8;
                    }
                    None => {}
                }
            }
            let diff: i32 = 7 - last_file;
            if diff > 0 {
                fen.push(std::char::from_digit(diff as u32, 10).unwrap());
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        fen += if self.stm == Color::White {
            " w "
        } else {
            " b "
        };
        fen += format!("{}", self.castling_rooks).as_str();
        match self.ep_square {
            Some(sq) => {
                fen += format!(" {} ", sq).to_lowercase().as_str();
            }
            None => {
                fen += " - ";
            }
        }

        fen += format!(
            "{}+{} ",
            3 - self.check_count(Color::White),
            3 - self.check_count(Color::Black)
        )
        .as_str();

        fen += format!("{} 1 ", self.half_move_clock).as_str();

        fen
    }

    pub fn make_move(&mut self, mv: Move) {
        /*if let Some(ep_square) = self.ep_square() {
            self.zkey.toggle_ep_square(ep_square);
        }
        self.zkey.toggle_castle_rights(self.castling_rooks);*/

        let from = mv.from_sq();
        let to = mv.to_sq();
        let from_pce = self.piece_at(from).unwrap();
        let mut captured: Option<Piece> = None;
        self.ep_square = None;
        self.half_move_clock += 1;

        match mv.kind() {
            MoveKind::None => {
                captured = self.piece_at(to);
                if captured.is_some() {
                    self.remove_piece(to);
                }

                self.move_piece(from, to);
            }
            MoveKind::Promotion => {
                captured = self.piece_at(to);
                if captured.is_some() {
                    self.remove_piece(to);
                }
                self.remove_piece(from);
                self.add_piece(to, Piece::new(self.stm, mv.promo_piece()))
            }
            MoveKind::Enpassant => {
                self.move_piece(from, to);

                let cap_sq = if self.stm == Color::White {
                    to - 8
                } else {
                    to + 8
                };
                captured = self.piece_at(cap_sq);
                self.remove_piece(cap_sq);
            }
            MoveKind::Castle => {
                let king_side = to > from;
                self.move_piece(from, CastlingRooks::king_to(king_side, self.stm()));
                self.move_piece(to, CastlingRooks::rook_to(king_side, self.stm()));
            }
        }

        if from_pce.piece_type() == PieceType::King {
            self.castling_rooks.color_mut(self.stm).remove_both();
        } else if from_pce.piece_type() == PieceType::Rook {
            self.castling_rooks.color_mut(self.stm).remove(from);
        } else if from_pce.piece_type() == PieceType::Pawn {
            self.half_move_clock = 0;
            if (from - to).abs() == 16 {
                self.ep_square = Some(Square::from_raw(
                    ((from.value() as i32 + to.value() as i32) / 2) as u16,
                ));
            }
        }

        if let Some(cap) = captured {
            self.half_move_clock = 0;
            if cap.piece_type() == PieceType::Rook {
                self.castling_rooks.color_mut(self.stm().flip()).remove(to);
            }
        }

        self.stm = self.stm.flip();

        /*if let Some(ep_square) = self.ep_square() {
            self.zkey.toggle_ep_square(ep_square);
        }
        self.zkey.toggle_castle_rights(self.castling_rooks);
        self.zkey.toggle_stm();*/

        self.update_check_info();
    }

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn colors(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    pub fn occ(&self) -> Bitboard {
        self.colors(Color::White) | self.colors(Color::Black)
    }

    pub fn pieces(&self, piece: PieceType) -> Bitboard {
        self.pieces[piece as usize]
    }

    pub fn colored_pieces(&self, piece: Piece) -> Bitboard {
        self.colors(piece.color()) & self.pieces(piece.piece_type())
    }

    pub fn piece_count(&self, c: Color, pt: PieceType) -> i32 {
        self.colored_pieces(Piece::new(c, pt)).popcount() as i32
    }

    pub fn king_sq(&self, color: Color) -> Square {
        self.colored_pieces(Piece::new(color, PieceType::King))
            .lsb()
    }

    pub fn castling_rooks(&self) -> CastlingRooks {
        self.castling_rooks
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.squares[sq.value() as usize]
    }

    pub fn check_count(&self, c: Color) -> u8 {
        self.check_count[c as usize]
    }

    pub fn attacked_by(&self, sq: Square, c: Color) -> bool {
        let diags =
            self.colors(c) & (self.pieces(PieceType::Bishop) | self.pieces(PieceType::Queen));
        let hvs = self.colors(c) & (self.pieces(PieceType::Rook) | self.pieces(PieceType::Queen));
        let pawns = self.colored_pieces(Piece::new(c, PieceType::Pawn));
        let knights = self.colored_pieces(Piece::new(c, PieceType::Knight));
        let king = self.colored_pieces(Piece::new(c, PieceType::King));
        let occ = self.occ() ^ self.colored_pieces(Piece::new(c.flip(), PieceType::King));

        (attacks::pawn_attacks(c.flip(), sq) & pawns).any()
            || (attacks::knight_attacks(sq) & knights).any()
            || (attacks::king_attacks(sq) & king).any()
            || (attacks::bishop_attacks(sq, occ) & diags).any()
            || (attacks::rook_attacks(sq, occ) & hvs).any()
    }

    pub fn any_attacked_by(&self, mut bb: Bitboard, c: Color) -> bool {
        while bb.any() {
            if self.attacked_by(bb.poplsb(), c) {
                return true;
            }
        }
        return false;
    }

    pub fn attackers_to(&self, sq: Square, c: Color) -> Bitboard {
        let diags =
            self.colors(c) & (self.pieces(PieceType::Bishop) | self.pieces(PieceType::Queen));
        let hvs = self.colors(c) & (self.pieces(PieceType::Rook) | self.pieces(PieceType::Queen));
        let pawns = self.colored_pieces(Piece::new(c, PieceType::Pawn));
        let knights = self.colored_pieces(Piece::new(c, PieceType::Knight));
        let king = self.colored_pieces(Piece::new(c, PieceType::King));
        let occ = self.occ() ^ self.colored_pieces(Piece::new(c.flip(), PieceType::King));

        (attacks::pawn_attacks(c.flip(), sq) & pawns)
            | (attacks::knight_attacks(sq) & knights)
            | (attacks::king_attacks(sq) & king)
            | (attacks::bishop_attacks(sq, occ) & diags)
            | (attacks::rook_attacks(sq, occ) & hvs)
    }

    pub fn is_drawn(&self /*, keys: &Vec<ZobristKey>*/) -> bool {
        if self.half_move_clock >= 100 {
            return true;
        }
        /*let mut count = 1;
        for &hash in keys
            .iter()
            .rev()
            .take(self.half_move_clock as usize + 1)
            .skip(3)
            .step_by(2)
        {
            if hash == self.zkey() {
                count += 1;
                if count == 3 {
                    return true;
                }
            }
        }*/
        return false;
    }

    pub fn checkers(&self) -> Bitboard {
        self.checkers
    }

    pub fn pinned(&self) -> Bitboard {
        self.hv_pinned | self.diag_pinned
    }

    pub fn diag_pinned(&self) -> Bitboard {
        self.diag_pinned
    }

    pub fn hv_pinned(&self) -> Bitboard {
        self.hv_pinned
    }

    pub fn ep_square(&self) -> Option<Square> {
        self.ep_square
    }

    /*pub fn zkey(&self) -> ZobristKey {
        self.zkey
    }

    pub fn recompute_zkey(&self) -> ZobristKey {
        let mut key = ZobristKey::new();
        for i in 0..64 {
            let sq = Square::from_raw(i);
            if let Some(piece) = self.piece_at(sq) {
                key.toggle_piece(piece, sq);
            }
        }
        key.toggle_castle_rights(self.castling_rooks);
        if let Some(ep_square) = self.ep_square() {
            key.toggle_ep_square(ep_square);
        }
        if self.stm() == Color::Black {
            key.toggle_stm();
        }
        key
    }*/

    fn empty() -> ThreeCheckState {
        Self {
            pieces: [Bitboard::NONE; 6],
            colors: [Bitboard::NONE; 2],
            squares: [None; 64],
            checkers: Bitboard::NONE,
            diag_pinned: Bitboard::NONE,
            hv_pinned: Bitboard::NONE,
            castling_rooks: CastlingRooks::DEFAULT,
            stm: Color::White,
            ep_square: None,
            half_move_clock: 0,
            check_count: [0; 2],
            // zkey: ZobristKey::new(),
        }
    }

    fn add_piece(&mut self, sq: Square, piece: Piece) {
        let sq_bb = Bitboard::from_square(sq);
        self.pieces[piece.piece_type() as usize] |= sq_bb;
        self.colors[piece.color() as usize] |= sq_bb;
        self.squares[sq.value() as usize] = Some(piece);

        // self.zkey.toggle_piece(piece, sq);
    }

    fn remove_piece(&mut self, sq: Square) {
        let piece = self.piece_at(sq).unwrap();
        let sq_bb = Bitboard::from_square(sq);
        self.pieces[piece.piece_type() as usize] ^= sq_bb;
        self.colors[piece.color() as usize] ^= sq_bb;
        self.squares[sq.value() as usize] = None;

        // self.zkey.toggle_piece(piece, sq);
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.piece_at(from).unwrap();
        let bb = Bitboard::from_square(from) | Bitboard::from_square(to);
        self.pieces[piece.piece_type() as usize] ^= bb;
        self.colors[piece.color() as usize] ^= bb;
        self.squares[from.value() as usize] = None;
        self.squares[to.value() as usize] = Some(piece);

        // self.zkey.toggle_piece(piece, from);
        // self.zkey.toggle_piece(piece, to);
    }

    fn update_check_info(&mut self) {
        let king_sq = self.king_sq(self.stm());
        self.checkers = self.attackers_to(king_sq, self.stm().flip());

        if self.checkers().any() {
            self.check_count[self.stm as usize] += 1;
        }

        // this includes enemy pieces as pinned but they are ignored so it is fine
        self.diag_pinned = Bitboard::NONE;
        self.hv_pinned = Bitboard::NONE;

        let queens = self.colored_pieces(Piece::new(self.stm().flip(), PieceType::Queen));
        let rooks = self.colored_pieces(Piece::new(self.stm().flip(), PieceType::Rook));
        let bishops = self.colored_pieces(Piece::new(self.stm().flip(), PieceType::Bishop));

        let mut diag_attackers =
            attacks::bishop_attacks(king_sq, Bitboard::NONE) & (bishops | queens);

        let block_mask = self.occ() ^ diag_attackers;

        while diag_attackers.any() {
            let attacker = diag_attackers.poplsb();

            let between = attacks::line_between(king_sq, attacker) & block_mask;
            if between.one() {
                self.diag_pinned |= between;
            }
        }

        let mut hv_attackers = attacks::rook_attacks(king_sq, Bitboard::NONE) & (rooks | queens);

        let block_mask = self.occ() ^ hv_attackers;

        while hv_attackers.any() {
            let attacker = hv_attackers.poplsb();

            let between = attacks::line_between(king_sq, attacker) & block_mask;
            if between.one() {
                self.hv_pinned |= between;
            }
        }
    }
}

impl CopyMakeBoard for ThreeCheckState {
    type Move = Move;
    type Square = Square;
    type Color = Color;
    type Piece = Piece;
    type MoveList = MoveList;

    fn startpos() -> Self {
        Self::startpos()
    }

    fn from_fen(fen: &str) -> Option<Self> {
        Self::from_fen(fen)
    }

    fn game_result(&self) -> super::board::GameResult {
        if self.is_drawn() {
            return GameResult::DRAW;
        }

        if self.check_count(self.stm()) >= 3 {
            return GameResult::LOSS;
        }

        if self.gen_moves().len() == 0 {
            if self.checkers().any() {
                return GameResult::LOSS;
            } else {
                return GameResult::DRAW;
            }
        }

        return GameResult::NONE;
    }

    fn gen_moves(&self) -> Self::MoveList {
        let mut moves = MoveList::new();
        movegen::movegen(self, &mut moves);
        moves
    }

    fn make_move(&mut self, mv: Self::Move) -> bool {
        self.make_move(mv);
        true
    }

    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece> {
        self.piece_at(sq)
    }
}

impl fmt::Display for ThreeCheckState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::from_rank_file(rank, file);
                let p = self.piece_at(sq);
                match p {
                    Some(piece) => {
                        write!(f, "{}", piece.char_repr())?;
                    }
                    None => {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "stm: {}", self.stm)?;
        writeln!(f, "castling rights: {}", self.castling_rooks)?;
        match self.ep_square {
            Some(sq) => {
                writeln!(f, "ep square: {}", sq)?;
            }
            None => {
                writeln!(f, "ep square: N/A")?;
            }
        }
        writeln!(f, "half move clock: {}", self.half_move_clock)?;
        writeln!(f, "White checks: {}", self.check_count(Color::White))?;
        writeln!(f, "Black checks: {}", self.check_count(Color::Black))?;
        writeln!(f, "fen: {}", self.to_fen())?;
        Ok(())
    }
}

pub type ThreeCheckBoard = CopyMakeWrapper<ThreeCheckState>;
