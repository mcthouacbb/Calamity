mod games;
mod perft;
mod search;
mod util;

use std::fs::File;
use std::io::prelude::*;

use games::{
    board::Board, connect4::Connect4Board, hexapawn::HexapawnBoard, tictactoe::TicTacToeBoard,
};
use search::{
    ab_solver::ABSolver,
    search::{Search, SearchLimits},
};

use perft::run_perft_suite;

fn main() {
    let mut file = File::open("res/c4_perft.txt").unwrap();
    let mut c4_tests = String::new();
    let _ = file.read_to_string(&mut c4_tests);
    run_perft_suite::<Connect4Board>(&c4_tests);

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
