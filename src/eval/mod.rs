pub mod three_check;

pub use three_check::ThreeCheckEval;

use crate::games::board::Board;

pub trait Eval<B: Board> {
    fn evaluate(board: &B) -> i32;
}
