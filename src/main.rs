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
    let mut board = TicTacToeBoard::startpos();
    board.make_move(TicTacToeMove::new(TicTacToeSquare::from_rank_file(0, 0)));
    board.make_move(TicTacToeMove::new(TicTacToeSquare::from_rank_file(1, 0)));
    println!("{}", board);

    let mut solver = ABSolver::<TicTacToeBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);
}
