#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
use pleco::Board;

mod evaluate;
mod search;

pub use search::search;
pub use evaluate::eval;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn find_move(move_string: String, depth: u8) -> String {
    let moves = move_string.split(" ");
    let mut board = Board::start_pos(); 

    for mv in moves {
        board.apply_uci_move(mv);
    }
    let turn = &board.turn();
    let (best_move, _score) = search(board, depth, *turn);
    best_move.stringify()
}
