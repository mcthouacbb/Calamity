use std::{
    default,
    time::{Duration, Instant},
};

use crate::{
    eval::{Eval, ThreeCheckEval},
    games::{
        board::{Board, GameResult},
        three_check::{Move, MoveList, PieceType, ThreeCheckBoard, ZobristKey},
    },
};

use super::{
    search::{Search, SearchLimits, SearchResult},
    tt::TT,
};

fn mvv_lva(captured: PieceType, moving: PieceType) -> i32 {
    return 8 * captured as i32 - moving as i32;
}

#[derive(Debug, Default, Clone)]
struct TTEntry {
    mv: Option<Move>,
}

pub struct ThreeCheckSearch {
    nodes: u64,
    root_best_move: Option<Move>,
    root_depth: i32,
    stop: bool,
    start_time: Instant,
    limits: SearchLimits,
    tt: TT<TTEntry>,
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
            tt: TT::new(16),
        }
    }

    fn score_move(&mut self, board: &mut ThreeCheckBoard, mv: Move, tt_move: Option<Move>) -> i32 {
        if Some(mv) == tt_move {
            return 1000000;
        }
        let state = board.curr_state();
        let moving = state.piece_at(mv.from_sq()).unwrap().piece_type();
        if let Some(captured) = state.piece_at(mv.to_sq()) {
            return mvv_lva(captured.piece_type(), moving) + 100;
        }
        0
    }

    fn order_moves(
        &mut self,
        board: &mut ThreeCheckBoard,
        moves: &mut MoveList,
        tt_move: Option<Move>,
    ) {
        moves.sort_by_key(|mv: &Move| -self.score_move(board, *mv, tt_move));
    }

    fn alpha_beta(
        &mut self,
        board: &mut ThreeCheckBoard,
        depth: i32,
        ply: i32,
        mut alpha: i32,
        mut beta: i32,
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

        if board.curr_state().check_count(board.curr_state().stm()) >= 3 {
            return -Self::SCORE_WIN + ply;
        }

        if board.curr_state().is_drawn() {
            return 0;
        }

        let ttEntry = self.tt.probe(board.curr_state().zkey().value());

        if depth <= 0 {
            return ThreeCheckEval::evaluate(board);
        }

        let in_check = board.curr_state().checkers().any();
        let root = ply == 0;

        if !in_check && !root {
            let static_eval = ThreeCheckEval::evaluate(board);
            if depth <= 4 && static_eval - 100 * depth >= beta {
                return static_eval;
            }
        }

        let mut moves = board.gen_moves();
        if moves.len() == 0 {
            if board.curr_state().checkers().any() {
                return -Self::SCORE_WIN + ply;
            }
            return 0;
        }

        self.order_moves(board, &mut moves, ttEntry.and_then(|tte| tte.mv));
        let mut best_score = -Self::SCORE_WIN;
        let mut best_move = None;

        for mv in moves.iter() {
            let mv = *mv;
            // three_check uses legal movegen
            board.make_move(mv);
            self.nodes += 1;

            let mut score = 0;
            let new_depth = depth - 1 + board.curr_state().checkers().any() as i32;
            score = -self.alpha_beta(board, new_depth, ply + 1, -beta, -alpha);

            board.unmake_move();
            if self.stop {
                return 0;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
                best_move = Some(mv);
                if ply == 0 {
                    self.root_best_move = Some(mv);
                }
            }

            if score >= beta {
                break;
            }
        }

        self.tt
            .store(board.curr_state().zkey().value(), TTEntry { mv: best_move });

        best_score
    }

    pub fn clear(&mut self) {
        self.tt.clear();
    }
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
        let mut best_move = None;
        for depth in 1..max_depth {
            self.root_depth = depth;
            let iter_score =
                self.alpha_beta(&mut tmp_board, depth, 0, -Self::SCORE_WIN, Self::SCORE_WIN);
            if self.stop {
                break;
            }

            score = iter_score;
            best_move = self.root_best_move;
            let elapsed = Instant::now() - self.start_time;
            println!(
                "info depth {} nodes {} time {} score cp {} nps {} pv {}",
                depth,
                self.nodes,
                elapsed.as_millis(),
                score,
                (self.nodes as f64 / elapsed.as_secs_f64()) as i32,
                best_move.unwrap()
            );
        }
        let end_time = Instant::now();

        SearchResult {
            nodes: self.nodes,
            time: end_time - self.start_time,
            best_move: best_move.unwrap(),
            score: score,
            pv: Vec::new(),
        }
    }
}
