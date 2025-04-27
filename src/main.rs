mod games;
mod perft;
mod search;
mod util;

use games::{
    board::Board,
    connect4::Connect4Board,
    hexapawn::HexapawnBoard,
    three_check::{self, ThreeCheckBoard, ThreeCheckState},
    tictactoe::TicTacToeBoard,
};
use search::{
    ab_solver::ABSolver,
    c4_solver::{C4Benchmark, run_benchmark},
    search::{Search, SearchLimits},
};
use util::Square;

fn main() {
    // perft::run_perft_suite_file::<Connect4Board>("res/c4_perft.txt");
    /*let board = TicTacToeBoard::from_fen("3/O2/X2 X").unwrap();
    println!("{}", board);

    let mut solver = ABSolver::<TicTacToeBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);

    let board = HexapawnBoard::startpos();
    println!("{}", board);

    let mut solver = ABSolver::<HexapawnBoard>::new();
    let result = solver.search(&board, SearchLimits::default());
    println!("{:?}", result);*/

    let mut board = three_check::ThreeCheckBoard::startpos();

    println!("{}", board);
}
