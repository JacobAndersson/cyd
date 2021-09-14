mod evaluate;
mod search;

use pleco::{Board, Player};

fn main() {
    let mut board = Board::start_pos();
    let SEARCH_DEPTH = 5;

    for i in 0..10 {
        let (mv, score) =
            search::alpha_beta(board.clone(), SEARCH_DEPTH, board.turn(), -9999., 9999.);
        board.apply_move(mv);
        println!("{}", board);
    }
}
