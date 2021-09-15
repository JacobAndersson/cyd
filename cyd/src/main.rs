mod evaluate;
mod search;

use pleco::Board;

use std::time::Instant;

fn main() {
    let mut board = Board::start_pos();
    const SEARCH_DEPTH: u8 = 6;

    let t0 = Instant::now();
    while !board.checkmate() && board.rule_50() != 50 {
        let (mv, score) =
            search::alpha_beta(board.clone(), SEARCH_DEPTH, board.turn(), -9999., 9999.);
        board.apply_move(mv);
        println!(
            "SCORE: {}, MOVE: {}, player: {}",
            score,
            &mv,
            board.turn().other_player()
        );
        println!("{}", board);
    }

    println!("GAME TOOK: {:?}", t0.elapsed())
}
