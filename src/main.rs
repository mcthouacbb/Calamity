mod games;
mod perft;
mod search;
mod util;

use std::{io::stdin, str::SplitWhitespace};

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

pub fn parse_move(board: &ThreeCheckBoard, str: &str) -> three_check::Move {
    let legal_moves = board.gen_moves();
    for mv in legal_moves {
        if mv.to_string() == str {
            return mv;
        }
    }
    panic!("WTF {}", str);
    three_check::Move::NULL
}

fn parse_startpos(curr_board: &mut ThreeCheckBoard, toks: &mut SplitWhitespace<'_>) {
    *curr_board = ThreeCheckBoard::startpos();
    if toks.next() == Some("moves") {
        loop {
            let mv_str = toks.next();
            if let Some(str) = mv_str {
                curr_board.make_move(parse_move(&curr_board, str));
            } else {
                break;
            }
        }
    }
}

fn parse_fen(curr_board: &mut ThreeCheckBoard, toks: &mut SplitWhitespace<'_>) {
    let mut fen = toks.next().unwrap().to_string();
    let mut tok = toks.next();
    loop {
        if tok == Some("moves") || tok == None {
            break;
        }
        fen += " ";
        fen += tok.unwrap();
        tok = toks.next();
    }
    *curr_board = ThreeCheckBoard::from_fen(fen.as_str()).unwrap();
    if tok == Some("moves") {
        loop {
            let mv_str = toks.next();
            if let Some(str) = mv_str {
                curr_board.make_move(parse_move(&curr_board, str));
            } else {
                break;
            }
        }
    }
}

fn select_random_move(board: &ThreeCheckBoard) -> three_check::Move {
    let moves = board.gen_moves();
    moves[rand::random_range(0..moves.len())]
}

fn run_three_check() {
    let mut curr_board = ThreeCheckBoard::startpos();
    loop {
        let mut command = String::new();
        stdin().read_line(&mut command).expect("Bad input");
        let mut toks = command.split_whitespace();
        match toks.next() {
            Some("uci") => {
                println!("id name calamity");
                println!("id author mcthouacbb");
                println!("option name Hash type spin default 1 min 1 max 1");
                println!("option name UCI_3Check type check default true");
                // println!("option name UCI_Variant type combo default chess var 3check var 5check");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                // empty for now
            }
            Some("position") => match toks.next() {
                Some("startpos") => {
                    parse_startpos(&mut curr_board, &mut toks);
                }
                Some("fen") => {
                    parse_fen(&mut curr_board, &mut toks);
                }
                _ => {
                    println!("info string invalid command");
                }
            },
            Some("go") => {
                println!("bestmove {}", select_random_move(&curr_board));
            }
            Some("aaa") => {
                for mv in curr_board.gen_moves() {
                    println!("{}", mv);
                }
            }
            Some("d") => {
                println!("{}", &mut curr_board);
            }
            Some("quit") => {
                return;
            }
            _ => {
                println!("info string invalid command");
            }
        }
    }
}

fn main() {
    run_three_check();
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
}
