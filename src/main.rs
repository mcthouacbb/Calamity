mod board;
mod games;
mod search;
mod util;

use board::Board;
use games::tictactoe::{TicTacToeBoard, TicTacToeMove, TicTacToeSquare};
use search::{
    ab_solver::ABSolver,
    search::{Search, SearchLimits},
};

fn main() {
    let board = TicTacToeBoard::from_fen("3/O2/X2 X").unwrap();
    println!("{}", board);

    let mut solver = ABSolver::<TicTacToeBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);
}
