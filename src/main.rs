mod games;
mod search;
mod util;

use games::{board::Board, connect4::Connect4Board, hexapawn::HexapawnBoard, tictactoe::TicTacToeBoard};
use search::{
    ab_solver::ABSolver,
    perft::perft,
    search::{Search, SearchLimits},
};

fn main() {
    /*let nodes = perft::<true, TicTacToeBoard>(&mut TicTacToeBoard::startpos(), 100);
    println!("Nodes: {}", nodes);

    let nodes = perft::<true, HexapawnBoard>(&mut HexapawnBoard::startpos(), 100);
    println!("Nodes: {}", nodes);*/

    let mut board = Connect4Board::from_fen("7/7/3y3/1yrr3/1rryyy1/1ryryr1 r").unwrap();
    let nodes = perft::<true, Connect4Board>(&mut board, 11);
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
