#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    NONE,
    WIN,
    DRAW,
    LOSS,
}

pub trait Board: Sized + Clone {
    type Move: Copy + Clone + PartialEq + Eq;
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
