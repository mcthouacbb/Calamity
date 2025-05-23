use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

use crate::games::{
    board::{Board, GameResult},
    connect4::{Connect4Board, Connect4Move},
};

use super::{
    search::{Search, SearchLimits, SearchResult},
    tt::{TT, TTBound, decisive_score_from_tt, decisive_score_to_tt},
};

#[derive(Default, Clone)]
pub struct C4TTEntry {
    score: i32,
    bound: TTBound,
}

impl C4TTEntry {
    fn adjust_from_tt(&mut self, ply: i32) {
        // non zero scores are terminal
        if self.score != 0 {
            self.score = decisive_score_from_tt(self.score, ply);
        }
    }

    fn to_tt(&self, ply: i32) -> Self {
        let mut result = self.clone();
        if result.score != 0 {
            result.score = decisive_score_to_tt(self.score, ply);
        }
        result
    }
}

use arrayvec::ArrayVec;

pub struct Connect4Solver {
    nodes: u64,
    root_best_move: Option<Connect4Move>,
    tt: TT<C4TTEntry>,
}

impl Connect4Solver {
    const SCORE_WIN: i32 = 1000;

    pub fn new() -> Self {
        Self {
            nodes: 0,
            root_best_move: None,
            tt: TT::new(32),
        }
    }

    fn score_move(&mut self, board: &mut Connect4Board, mv: Connect4Move) -> i32 {
        let col = mv.sq().column();
        let row = mv.sq().row();
        let base_score =
            -3 * (col.abs_diff(3) as i32) - (row.abs_diff(3) as i32) + 5 * (row % 2 == 1) as i32;

        let threats_after = board.curr_state().our_threats_after(mv);
        let moves_after = board.curr_state().move_locations_after(mv);
        let double_threat = (threats_after & moves_after).multiple()
            || (threats_after & threats_after.south() & moves_after).any();
        base_score
            + 20 * threats_after.popcount() as i32
            + 30 * (threats_after & moves_after).popcount() as i32
            + 100 * double_threat as i32
    }

    fn order_moves(&mut self, board: &mut Connect4Board, moves: &mut ArrayVec<Connect4Move, 7>) {
        moves.sort_by_key(|mv: &Connect4Move| -self.score_move(board, *mv));
    }

    fn alpha_beta<const PV: bool>(
        &mut self,
        board: &mut Connect4Board,
        ply: i32,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // mate distance pruning, prune if it's impossible to change the search result
        // even if we win in the current position
        alpha = alpha.max(-Self::SCORE_WIN + ply);
        beta = beta.min(Self::SCORE_WIN - ply);
        if alpha >= beta {
            return alpha;
        }

        // win on the next move
        let move_locations = board.curr_state().move_locations();
        if (board.curr_state().our_threats() & move_locations).any() {
            return Self::SCORE_WIN - (ply + 1);
        }
        // cannot stop the opponent from winning in 2 moves
        let opp_threats = board.curr_state().their_threats();
        if (opp_threats & move_locations).multiple()
            || (opp_threats & opp_threats.south() & move_locations).any()
        {
            return -Self::SCORE_WIN + (ply + 2);
        }

        let non_losing_moves = if (opp_threats & move_locations).any() {
            opp_threats & move_locations
        } else {
            move_locations & !opp_threats.south()
        };

        match board.game_result() {
            GameResult::WIN => return Self::SCORE_WIN - ply,
            GameResult::DRAW => return 0,
            GameResult::LOSS => return -Self::SCORE_WIN + ply,
            _ => {}
        }

        if let Some(mut data) = self.tt.probe(board.curr_state().key()) {
            data.adjust_from_tt(ply);

            if data.bound == TTBound::EXACT
                || (data.bound == TTBound::LOWER && data.score >= beta)
                || (data.bound == TTBound::UPPER && data.score <= alpha)
            {
                return data.score;
            }
        }

        let mut moves = board.gen_moves();
        self.order_moves(board, &mut moves);
        let mut best_score = -Self::SCORE_WIN;
        let mut moves_played = 0;

        let mut bound = TTBound::UPPER;
        for mv in moves.iter() {
            let mv = *mv;
            if !non_losing_moves.has(mv.sq()) {
                continue;
            }
            // no illegal moves in connect 4
            board.make_move(mv);
            self.nodes += 1;
            moves_played += 1;

            let mut score = 0;
            if !PV || moves_played > 1 {
                score = -self.alpha_beta::<false>(board, ply + 1, -alpha - 1, -alpha);
            }
            if PV && (moves_played == 1 || score > alpha) {
                score = -self.alpha_beta::<true>(board, ply + 1, -beta, -alpha);
            }

            board.unmake_move();

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                bound = TTBound::EXACT;
                alpha = score;
                if ply == 0 {
                    self.root_best_move = Some(mv);
                }
            }

            if score >= beta {
                bound = TTBound::LOWER;
                break;
            }
        }

        self.tt.store(
            board.curr_state().key(),
            C4TTEntry {
                score: best_score,
                bound: bound,
            }
            .to_tt(ply),
        );

        best_score
    }

    pub fn clear(&mut self) {
        self.tt.clear();
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
        let score = self.alpha_beta::<true>(&mut tmp_board, 0, -Self::SCORE_WIN, Self::SCORE_WIN);
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
    let mut solver = Connect4Solver::new();

    println!("Running connect 4 benchmark {:?}", benchmark);
    for (it, line) in positions.lines().enumerate() {
        let mut parts = line.split(';');
        let fen = parts.next().unwrap();
        let expected_score = parts.next().unwrap().parse::<i32>().unwrap();

        let board = Connect4Board::from_fen(fen).unwrap();
        solver.clear();
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
        "Average time: {}\nAverage nodes: {}\nAverage nps: {}",
        total_time.as_secs_f64() / 1000.0,
        total_nodes / 1000,
        total_nodes as f64 / total_time.as_secs_f64()
    );
}
