pub enum GameResult {
    NONE,
    WIN,
    DRAW,
    LOSS,
}

pub trait Board: Sized {
    type Move;
    type Square;
    type PieceType;
    type Color;
    type Piece;
    type MoveList: IntoIterator;

    fn startpos() -> Self;

    fn game_result(&self) -> GameResult;
    fn piece_on(&self, sq: Self::Square) -> Option<Self::Piece>;
    fn gen_moves(&self) -> Self::MoveList;

    fn make_move(&self, mv: Self::Move) -> Option<Self>;
}
