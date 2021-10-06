use crate::search::TtEntry;
use dashmap::DashMap;
use std::sync::Arc;
use pleco::Board;

use crate::search;

pub fn new_tt_table() -> Arc<DashMap<u64, TtEntry>> {
    Arc::new(DashMap::<u64, TtEntry>::new())
}

pub fn find_move_fen(fen: String, depth: u8, num_threads: u8) -> (String, f32) {
    match Board::from_fen(&fen){
        Ok(board) => {
            let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), num_threads);
            return (mv.stringify(), score);
        },
        Err(_) => ("".to_string(),0.)
    }
}

pub fn find_move(moves: String, depth: u8, num_threads: u8) -> (String, f32) {
    let mut board = Board::start_pos();

    let mvs = moves.split(" ");
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), num_threads);
    return (mv.stringify(), score);
}

#[allow(dead_code)]
fn from_start(depth: u8, n_threads: u8) {
    use std::time::Instant;

    let mut board = Board::start_pos();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) =
            search::search_parallel(board.clone(), depth, board.turn(), n_threads);
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
