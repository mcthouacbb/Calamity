use std::time::{Duration, Instant};

use crate::board::{Board, GameResult};

use super::search::{Search, SearchLimits, SearchResult};

pub struct ABSolver<B: Board> {
    nodes: u64,
    root_best_move: Option<B::Move>,
}

impl<B: Board> ABSolver<B> {
    const SCORE_WIN: i32 = 1000;

    pub fn new() -> Self {
        Self {
            nodes: 0,
            root_best_move: None,
        }
    }

    fn alpha_beta(&mut self, board: &mut B, ply: i32, mut alpha: i32, beta: i32) -> i32 {
        match board.game_result() {
            GameResult::WIN => return Self::SCORE_WIN - ply,
            GameResult::DRAW => return 0,
            GameResult::LOSS => return -Self::SCORE_WIN + ply,
            _ => {}
        }
        let moves = board.gen_moves();
        let mut best_score = -Self::SCORE_WIN;
        for mv in moves {
            if !board.make_move(mv) {
                continue;
            }
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

impl<B: Board> Search<B> for ABSolver<B> {
    fn search(&mut self, board: &B, limits: SearchLimits) -> SearchResult<B> {
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
