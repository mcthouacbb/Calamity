use std::fs::File;
use std::io::prelude::*;

use crate::games::board::{Board, GameResult};

pub fn perft<const SPLIT: bool, B: Board>(board: &mut B, depth: u32) -> u64 {
    if depth == 0 || board.game_result() != GameResult::NONE {
        return 1;
    }

    let mut nodes = 0;
    for mv in board.gen_moves() {
        board.make_move(mv);
        let sub_nodes = perft_impl(board, depth - 1);
        if SPLIT {
            println!("{:?}: {}", mv, sub_nodes);
        }
        nodes += sub_nodes;
        board.unmake_move();
    }
    nodes
}

fn perft_impl<B: Board>(board: &mut B, depth: u32) -> u64 {
    if depth == 0 || board.game_result() != GameResult::NONE {
        return 1;
    }

    let mut nodes = 0;
    for mv in board.gen_moves() {
        board.make_move(mv);
        nodes += perft_impl(board, depth - 1);
        board.unmake_move();
    }
    nodes
}

pub fn run_perft_suite_file<B: Board>(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut tests = String::new();
    let _ = file.read_to_string(&mut tests);
    run_perft_suite::<B>(&tests);
}

pub fn run_perft_suite<B: Board>(tests: &str) {
    let stripped = tests.replace('\r', "");
    let lines = stripped.split("\n");
    let mut passes = 0;
    let mut fails = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(';').collect();
        let fen = parts[0];
        let mut board = B::from_fen(fen).unwrap();
        let mut failed = false;
        for test in parts.into_iter().skip(1) {
            let mut depth_nodes = test.split(' ');
            let depth = depth_nodes.next().unwrap().parse().unwrap();
            let a = depth_nodes.next().unwrap();
            let exp_nodes = a.parse::<u64>().unwrap();

            let actual_nodes = perft::<false, B>(&mut board, depth);
            if exp_nodes == actual_nodes {
                passes += 1;
            } else {
                failed = true;
                fails += 1;
                println!(
                    "Failed: {} depth {} Expected {} got {}",
                    fen, depth, exp_nodes, actual_nodes
                );
            }
        }
        if !failed {
            println!("Passed {}", fen);
        }
    }
    println!("Passed: {} / {}", passes, passes + fails);
}
