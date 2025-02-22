mod games;
mod search;
mod util;

use games::{board::Board, hexapawn::HexapawnBoard, tictactoe::TicTacToeBoard};
use search::{
    ab_solver::ABSolver,
    perft::perft,
    search::{Search, SearchLimits},
};

fn main() {
    let nodes = perft::<true, TicTacToeBoard>(&mut TicTacToeBoard::startpos(), 100);
    println!("Nodes: {}", nodes);

    let nodes = perft::<true, HexapawnBoard>(&mut HexapawnBoard::startpos(), 100);
    println!("Nodes: {}", nodes);

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
