mod board;
mod games;

use board::Board;
use games::tictactoe::{Move, Square, TicTacToeBoard};

fn main() {
    let board = TicTacToeBoard::startpos();
    let board = board.make_move(Move::new(Square::A1)).unwrap();
    let board = board.make_move(Move::new(Square::A2)).unwrap();
    println!("{}", board);
}
