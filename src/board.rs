#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    NONE,
    WIN,
    DRAW,
    LOSS,
}

pub trait Board: Sized {
    type Move: Copy + Clone + PartialEq + Eq;
    type Square: Copy + Clone + PartialEq + Eq + PartialEq + Ord;
    type PieceType: Copy + Clone;
    type Color: Copy + Clone;
    type Piece: Copy + Clone;
    type MoveList: IntoIterator<Item = Self::Move>;

    fn startpos() -> Self;

    fn game_result(&self) -> GameResult;
    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece>;
    fn gen_moves(&self) -> Self::MoveList;

    fn make_move(&self, mv: Self::Move) -> Option<Self>;
}
