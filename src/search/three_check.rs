use std::time::{Duration, Instant};

use crate::{
    eval::{Eval, ThreeCheckEval},
    games::{
        board::{Board, GameResult},
        three_check::{Move, ThreeCheckBoard},
    },
};

use super::search::{Search, SearchLimits, SearchResult};

pub struct ThreeCheckSearch {
    nodes: u64,
    root_best_move: Option<Move>,
    root_depth: i32,
    stop: bool,
    start_time: Instant,
    limits: SearchLimits,
}

impl ThreeCheckSearch {
    const SCORE_WIN: i32 = 32000;

    pub fn new() -> Self {
        Self {
            nodes: 0,
            root_best_move: None,
            stop: false,
            root_depth: 0,
            start_time: Instant::now(),
            limits: SearchLimits::default(),
        }
    }

    // fn score_move(&mut self, board: &mut Connect4Board, mv: Connect4Move) -> i32 {
    //     let col = mv.sq().column();
    //     let row = mv.sq().row();
    //     let base_score =
    //         -3 * (col.abs_diff(3) as i32) - (row.abs_diff(3) as i32) + 5 * (row % 2 == 1) as i32;

    //     let threats_after = board.curr_state().our_threats_after(mv);
    //     let moves_after = board.curr_state().move_locations_after(mv);
    //     let double_threat = (threats_after & moves_after).multiple()
    //         || (threats_after & threats_after.south() & moves_after).any();
    //     base_score
    //         + 20 * threats_after.popcount() as i32
    //         + 30 * (threats_after & moves_after).popcount() as i32
    //         + 100 * double_threat as i32
    // }

    // fn order_moves(&mut self, board: &mut Connect4Board, moves: &mut ArrayVec<Connect4Move, 7>) {
    //     moves.sort_by_key(|mv: &Connect4Move| -self.score_move(board, *mv));
    // }

    fn alpha_beta(
        &mut self,
        board: &mut ThreeCheckBoard,
        depth: i32,
        ply: i32,
        mut _alpha: i32,
        mut _beta: i32,
    ) -> i32 {
        // mate distance pruning, prune if it's impossible to change the search result
        // even if we win in the current position
        // alpha = alpha.max(-Self::SCORE_WIN + ply);
        // beta = beta.min(Self::SCORE_WIN - ply);
        // if alpha >= beta {
        //     return alpha;
        // }

        if let Some(max_time) = self.limits.max_time {
            if self.root_depth > 1
                && self.nodes % 1024 == 0
                && Instant::now() - self.start_time > Duration::from_millis(max_time)
            {
                self.stop = true;
                return 0;
            }
        }

        match board.game_result() {
            GameResult::WIN => return Self::SCORE_WIN - ply,
            GameResult::DRAW => return 0,
            GameResult::LOSS => return -Self::SCORE_WIN + ply,
            _ => {}
        }

        if depth <= 0 {
            return ThreeCheckEval::evaluate(board);
        }

        let moves = board.gen_moves();
        let mut best_score = -Self::SCORE_WIN;

        for mv in moves.iter() {
            let mv = *mv;
            // three_check uses legal movegen
            board.make_move(mv);
            self.nodes += 1;

            let mut score = 0;
            score = -self.alpha_beta(board, depth - 1, ply + 1, -_beta, -_alpha);

            board.unmake_move();
            if self.stop {
                return 0;
            }

            if score > best_score {
                best_score = score;
                if ply == 0 {
                    self.root_best_move = Some(mv);
                }
            }
        }

        best_score
    }

    pub fn clear(&mut self) {}
}

impl Search<ThreeCheckBoard> for ThreeCheckSearch {
    fn search(
        &mut self,
        board: &ThreeCheckBoard,
        limits: SearchLimits,
    ) -> SearchResult<ThreeCheckBoard> {
        self.nodes = 0;
        self.root_best_move = None;
        self.stop = false;
        let mut tmp_board = board.clone();

        self.start_time = Instant::now();
        self.limits = limits;
        let mut score = 0;
        let mut max_depth = 128;
        if let Some(max) = limits.max_depth {
            max_depth = max_depth.min(max as i32);
        }
        let mut best_move = Move::NULL;
        for depth in 1..max_depth {
            self.root_depth = depth;
            let iter_score =
                self.alpha_beta(&mut tmp_board, depth, 0, -Self::SCORE_WIN, Self::SCORE_WIN);
            if self.stop {
                break;
            }

            score = iter_score;
            best_move = self.root_best_move.unwrap();
            let elapsed = Instant::now() - self.start_time;
            println!(
                "info depth {} nodes {} time {} score cp {} nps {} pv {}",
                depth,
                self.nodes,
                elapsed.as_millis(),
                score,
                (self.nodes as f64 / elapsed.as_secs_f64()) as i32,
                best_move
            );
        }
        let end_time = Instant::now();

        SearchResult {
            nodes: self.nodes,
            time: end_time - self.start_time,
            best_move: best_move,
            score: score,
            pv: Vec::new(),
        }
    }
}
