use std::time::{Duration, Instant};

use crate::{
    eval::{Eval, ThreeCheckEval},
    games::{
        board::Board,
        three_check::{Move, MoveList, PieceType, ThreeCheckBoard, see},
    },
};

use super::{
    search::{Search, SearchLimits, SearchResult},
    tt::{TT, TTBound, decisive_score_from_tt},
};

fn mvv_lva(captured: PieceType, moving: PieceType) -> i32 {
    return 8 * captured as i32 - moving as i32;
}

#[derive(Debug, Default, Clone, Copy)]
struct TTEntry {
    mv: Option<Move>,
    score: i16,
    depth: u8,
    bound: TTBound,
}

impl TTEntry {
    fn adjust_from_tt(&mut self, ply: i32) {
        // non zero scores are terminal
        if self.score.abs() as i32 >= ThreeCheckSearch::SCORE_WIN - 128 {
            self.score = decisive_score_from_tt(self.score as i32, ply) as i16;
        }
    }

    fn to_tt(&self, ply: i32) -> Self {
        let mut result = self.clone();
        if result.score.abs() as i32 >= ThreeCheckSearch::SCORE_WIN - 128 {
            result.score = decisive_score_from_tt(self.score as i32, ply) as i16;
        }
        result
    }
}

pub struct ThreeCheckSearch {
    nodes: u64,
    root_best_move: Option<Move>,
    root_depth: i32,
    stop: bool,
    start_time: Instant,
    limits: SearchLimits,
    tt: TT<TTEntry>,
    history: [[[i32; 64]; 64]; 2],
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
            history: [[[0; 64]; 64]; 2],
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
        self.history[state.stm() as usize][mv.from_sq().value() as usize]
            [mv.to_sq().value() as usize]
            - 10000000
    }

    fn order_moves(
        &mut self,
        board: &mut ThreeCheckBoard,
        moves: &mut MoveList,
        tt_move: Option<Move>,
    ) {
        moves.sort_by_key(|mv: &Move| -self.score_move(board, *mv, tt_move));
    }

    fn qsearch(&mut self, board: &mut ThreeCheckBoard, ply: i32, mut alpha: i32, beta: i32) -> i32 {
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

        let static_eval = ThreeCheckEval::evaluate(board);
        if static_eval >= beta {
            return static_eval;
        }

        if static_eval > alpha {
            alpha = static_eval;
        }

        let mut moves = board.gen_moves();
        if moves.len() == 0 {
            if board.curr_state().checkers().any() {
                return -Self::SCORE_WIN + ply;
            }
            return 0;
        }
        self.order_moves(board, &mut moves, None);

        let mut best_score = static_eval;

        for mv in moves.iter() {
            let mv = *mv;
            let capture = board.piece_on(mv.to_sq()).is_some();
            if !capture {
                continue;
            }

            if !see(board.curr_state(), mv, 0) {
                continue;
            }

            board.make_move(mv);
            self.nodes += 1;

            let score = -self.qsearch(board, ply + 1, -beta, -alpha);
            board.unmake_move();

            if self.stop {
                return 0;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }

            if score >= beta {
                break;
            }
        }

        best_score
    }

    fn alpha_beta<const PV: bool>(
        &mut self,
        board: &mut ThreeCheckBoard,
        depth: i32,
        ply: i32,
        mut alpha: i32,
        beta: i32,
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

        let root = ply == 0;

        if board.curr_state().check_count(board.curr_state().stm()) >= 3 {
            return -Self::SCORE_WIN + ply;
        }

        if !root && board.curr_state().is_drawn() {
            return 0;
        }

        let in_check = board.curr_state().checkers().any();

        let tt_entry = self.tt.probe(board.curr_state().zkey().value());

        if !PV {
            if let Some(entry) = tt_entry {
                if entry.depth as i32 >= depth
                    && (entry.bound == TTBound::EXACT
                        || (entry.bound == TTBound::UPPER && entry.score as i32 <= alpha)
                        || (entry.bound == TTBound::LOWER && entry.score as i32 >= beta))
                {
                    return entry.score as i32;
                }
            }
        }

        if depth <= 0 {
            return self.qsearch(board, ply, alpha, beta);
        }

        let static_eval = ThreeCheckEval::evaluate(board);

        if !in_check && !PV {
            if depth <= 4 && static_eval - 100 * depth >= beta {
                return static_eval;
            }

            if depth >= 3 {
                let r = 3;
                board.make_move(Move::NULL);
                let score = -self.alpha_beta::<false>(board, depth - r, ply + 1, -beta, -beta + 1);
                board.unmake_move();

                if score >= beta {
                    return score;
                }
            }
        }

        let mut moves = board.gen_moves();
        if moves.len() == 0 {
            if board.curr_state().checkers().any() {
                return -Self::SCORE_WIN + ply;
            }
            return 0;
        }

        self.order_moves(board, &mut moves, tt_entry.and_then(|tte| tte.mv));
        let mut best_score = -Self::SCORE_WIN;
        let mut best_move = None;
        let mut tt_bound = TTBound::UPPER;
        let mut moves_played = 0;

        for mv in moves.iter() {
            let mv = *mv;
            let capture = board.piece_on(mv.to_sq()).is_some();
            // three_check uses legal movegen
            board.make_move(mv);
            let gives_check = board.curr_state().checkers().any();

            if !root && best_score > -Self::SCORE_WIN + 128 && !capture && !gives_check {
                if !in_check && depth <= 4 && static_eval + 100 + 150 * depth <= alpha {
                    board.unmake_move();
                    continue;
                }
            }

            self.nodes += 1;
            moves_played += 1;

            let mut score = 0;
            let new_depth = depth - 1 + gives_check as i32;
            if moves_played >= 4 && depth >= 3 && !capture && !gives_check {
                let reduction =
                    (0.77 + (moves_played as f64).ln() * (depth as f64).ln() / 2.36) as i32;
                score = -self.alpha_beta::<false>(
                    board,
                    new_depth - reduction,
                    ply + 1,
                    -alpha - 1,
                    -alpha,
                );
                if score > alpha && reduction > 0 {
                    score =
                        -self.alpha_beta::<false>(board, new_depth, ply + 1, -alpha - 1, -alpha);
                }
            } else if !PV || moves_played > 1 {
                score = -self.alpha_beta::<false>(board, new_depth, ply + 1, -alpha - 1, -alpha);
            }

            if PV && (moves_played == 1 || score > alpha) {
                score = -self.alpha_beta::<true>(board, new_depth, ply + 1, -beta, -alpha);
            }

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
                tt_bound = TTBound::EXACT;

                if ply == 0 {
                    self.root_best_move = Some(mv);
                }
            }

            if score >= beta {
                tt_bound = TTBound::LOWER;
                if !capture {
                    self.history[board.curr_state().stm() as usize]
                        [mv.from_sq().value() as usize][mv.to_sq().value() as usize] +=
                        depth * depth;
                }
                break;
            }
        }

        self.tt.store(
            board.curr_state().zkey().value(),
            TTEntry {
                mv: best_move,
                depth: depth as u8,
                score: best_score as i16,
                bound: tt_bound,
            },
        );

        best_score
    }

    fn asp_windows(&mut self, board: &mut ThreeCheckBoard, depth: i32, prev_score: i32) -> i32 {
        let mut alpha = -Self::SCORE_WIN;
        let mut beta = Self::SCORE_WIN;
        let mut delta = 30;
        if depth >= 5 {
            alpha = prev_score - delta;
            beta = prev_score + delta;
        }
        loop {
            let iter_score = self.alpha_beta::<true>(board, depth, 0, alpha, beta);

            if self.stop {
                return 0;
            }

            if alpha < iter_score && iter_score < beta {
                return iter_score;
            } else if iter_score <= alpha {
                alpha = iter_score - delta;
            } else {
                beta = iter_score + delta;
            }
            delta *= 2;
        }
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
        self.history = [[[0; 64]; 64]; 2];
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
            let iter_score = self.asp_windows(&mut tmp_board, depth, score);
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
