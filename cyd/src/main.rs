mod evaluate;
mod search;

use pleco::Board;

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let mut board = Board::start_pos();
    const SEARCH_DEPTH: u8 = 4;
    let mut tt: HashMap<u64, search::TtEntry> = HashMap::new();

    let t0 = Instant::now();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) =
            search::alpha_beta(board.clone(), SEARCH_DEPTH, board.turn(), -9999., 9999., &mut tt);
        let end = mv_start.elapsed();
        board.apply_move(mv);
        println!(
            "SCORE: {}, MOVE: {}, player: {}, time: {:?}",
            score,
            &mv,
            board.turn().other_player(),
            end
        );
        println!("{}", board);
    }

    println!("GAME TOOK: {:?}", t0.elapsed())
}
