use std::time::Duration;

use crate::games::board::Board;

#[derive(Debug, Clone, Default)]
pub struct SearchResult<B: Board> {
    pub nodes: u64,
    pub time: Duration,
    pub best_move: B::Move,
    pub score: i32,
    pub pv: Vec<B::Move>,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SearchLimits {
    pub max_nodes: Option<u64>,
    pub max_time: Option<u64>,
    pub max_depth: Option<u64>,
}

pub trait Search<B: Board> {
    fn search(&mut self, board: &B, limits: SearchLimits) -> SearchResult<B>;
}
