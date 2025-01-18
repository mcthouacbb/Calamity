mod board;
mod games;
mod search;

use board::Board;
use games::tictactoe::{Move, Square, TicTacToeBoard};
use search::{
    ab_solver::ABSolver,
    search::{Search, SearchLimits},
};

fn main() {
    let mut board = TicTacToeBoard::startpos();
    board.make_move(Move::new(Square::A1));
    board.make_move(Move::new(Square::A2));
    println!("{}", board);

    let mut solver = ABSolver::<TicTacToeBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);
}
