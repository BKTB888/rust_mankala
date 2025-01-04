use crate::board_state::BoardState;

struct AI {
    side: bool,
    eval_func: fn(&BoardState, u8) -> f64
}
impl AI {
    fn new(side: bool, eval_func: fn(&BoardState, u8) -> f64) -> Self {
        AI {
            side,
            eval_func
        }
    }
}