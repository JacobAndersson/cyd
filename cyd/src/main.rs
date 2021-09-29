mod evaluate;
mod search;
mod utils;
mod cli;

use pleco::Board;
extern crate clap;

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
    let config = cli::get_config();
    let (mv, score) = utils::find_move(config.moves, config.depth, config.num_threads);
    println!("{}, {}", mv, score);
}
