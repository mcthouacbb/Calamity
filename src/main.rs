mod games;
mod perft;
mod search;
mod util;

use games::{
    board::Board, connect4::Connect4Board, hexapawn::HexapawnBoard, tictactoe::TicTacToeBoard,
};
use search::{
    ab_solver::ABSolver,
    c4_solver::{C4Benchmark, run_benchmark},
    search::{Search, SearchLimits},
};

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
    run_benchmark(C4Benchmark::EndEasy);
    run_benchmark(C4Benchmark::MidEasy);
    run_benchmark(C4Benchmark::MidMedium);
    run_benchmark(C4Benchmark::BeginEasy);
    run_benchmark(C4Benchmark::BeginMedium);
    run_benchmark(C4Benchmark::BeginHard);
}
