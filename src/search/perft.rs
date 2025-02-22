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
