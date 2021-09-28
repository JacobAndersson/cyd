mod evaluate;
mod search;
mod utils;

use pleco::Board;

use std::time::Instant;

const SEARCH_DEPTH: u8 = 5;
const NUM_THREADS: u8 = 3;

fn from_start() {
    let mut board = Board::start_pos();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) =
            search::search_parallel(board.clone(), SEARCH_DEPTH, board.turn(), NUM_THREADS);
        let end = mv_start.elapsed();
        board.apply_move(mv);

        println!("{}", board);
        println!(
            "SCORE: {}, MOVE: {}, player: {}, time: {:?}",
            score,
            &mv,
            board.turn().other_player(),
            end
        );
    }
}


fn main() {
    let mut board = Board::from_fen("4r1k1/1pb3pp/2p5/p2p4/P2P4/2B1rBqP/1P3QP1/3K1R2 w - - 4 27").unwrap();
    
    for _i in 0..5 {
        println!("{}", board);
        let mut tt = utils::new_tt_table();
        let (mv, score) = search::alpha_beta(board.clone(), 7, board.turn(), -9999.0, 9999.0, &mut tt, true);
        println!("{} {}", &mv.stringify(), score);
        board.apply_move(mv);
    }

}
