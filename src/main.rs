mod games;
mod perft;
mod search;
mod util;

use games::{
    board::Board, connect4::Connect4Board, hexapawn::HexapawnBoard, tictactoe::TicTacToeBoard,
};
use search::{
    ab_solver::ABSolver,
    search::{Search, SearchLimits},
};

use perft::run_perft_suite_file;

fn main() {
    run_perft_suite_file::<Connect4Board>("res/c4_perft.txt");

    let board = TicTacToeBoard::from_fen("3/O2/X2 X").unwrap();
    println!("{}", board);

    let mut solver = ABSolver::<TicTacToeBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);

    let board = HexapawnBoard::startpos();
    println!("{}", board);

    let mut solver = ABSolver::<HexapawnBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);
}
