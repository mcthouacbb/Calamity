use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

use crate::games::{
    board::{Board, GameResult},
    connect4::{Connect4Board, Connect4Move},
};

use super::search::{Search, SearchLimits, SearchResult};

pub struct Connect4Solver {
    nodes: u64,
    root_best_move: Option<Connect4Move>,
}

impl Connect4Solver {
    const SCORE_WIN: i32 = 1000;

    pub fn new() -> Self {
        Self {
            nodes: 0,
            root_best_move: None,
        }
    }

    fn alpha_beta(
        &mut self,
        board: &mut Connect4Board,
        ply: i32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        match board.game_result() {
            GameResult::WIN => return Self::SCORE_WIN - ply,
            GameResult::DRAW => return 0,
            GameResult::LOSS => return -Self::SCORE_WIN + ply,
            _ => {}
        }
        let moves = board.gen_moves();
        let mut best_score = -Self::SCORE_WIN;
        for mv in moves {
            // no illegal moves in connect 4
            board.make_move(mv);
            self.nodes += 1;

            let score = -self.alpha_beta(board, ply + 1, -beta, -alpha);

            board.unmake_move();

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
                if ply == 0 {
                    self.root_best_move = Some(mv);
                }
            }

            if score >= beta {
                break;
            }
        }

        best_score
    }
}

impl Search<Connect4Board> for Connect4Solver {
    fn search(
        &mut self,
        board: &Connect4Board,
        _limits: SearchLimits,
    ) -> SearchResult<Connect4Board> {
        self.nodes = 0;
        self.root_best_move = None;
        let mut tmp_board = board.clone();

        let start_time = Instant::now();
        let score = self.alpha_beta(&mut tmp_board, 0, -Self::SCORE_WIN, Self::SCORE_WIN);
        let end_time = Instant::now();

        SearchResult {
            nodes: self.nodes,
            time: end_time - start_time,
            best_move: self.root_best_move.unwrap(),
            score: score,
            pv: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum C4Benchmark {
    EndEasy,
    MidEasy,
    MidMedium,
    BeginEasy,
    BeginMedium,
    BeginHard,
}

pub fn run_benchmark(benchmark: C4Benchmark) {
    let filename = match benchmark {
        C4Benchmark::EndEasy => "res/c4_endgame_easy.txt",
        C4Benchmark::MidEasy => "res/c4_midgame_easy.txt",
        C4Benchmark::MidMedium => "res/c4_midgame_medium.txt",
        C4Benchmark::BeginEasy => "res/c4_opening_easy.txt",
        C4Benchmark::BeginMedium => "res/c4_opening_medium.txt",
        C4Benchmark::BeginHard => "res/c4_opening_hard.txt",
    };

    let mut file = File::open(filename).unwrap();
    let mut positions = String::new();
    let _ = file.read_to_string(&mut positions);

    let mut total_nodes = 0;
    let mut total_time: Duration = Duration::ZERO;

    println!("Running connect 4 benchmark {:?}", benchmark);
    for (it, line) in positions.lines().enumerate() {
        let mut parts = line.split(';');
        let fen = parts.next().unwrap();
        let expected_score = parts.next().unwrap().parse::<i32>().unwrap();

        let board = Connect4Board::from_fen(fen).unwrap();
        let mut solver = Connect4Solver::new();
        let limits = SearchLimits {
            max_nodes: None,
            max_depth: None,
            max_time: None,
        };
        let result = solver.search(&board, limits);
        if result.score != expected_score {
            println!(
                "Failed: incorrect score {} fen: {} expected score: {}",
                result.score, fen, expected_score
            );
        }
        total_nodes += result.nodes;
        total_time += result.time;
        if it % 32 == 0 {
            println!("{} / 1000 done", it);
        }
    }
    println!("Finished connect 4 benchmark {:?}", benchmark);
    println!(
        "Average time: {}\naverage nodes: {}\naverage nps: {}",
        total_time.as_secs_f64() / 1000.0,
        total_nodes / 1000,
        total_nodes as f64 / total_time.as_secs_f64()
    );
}
