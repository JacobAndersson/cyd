use crate::search;
use pleco::Board;

use crate::utils;
use std::{io, thread, time};

#[allow(dead_code)]
pub fn find_move_fen(fen: String, depth: u8, num_threads: u8) -> (String, f32) {
    match Board::from_fen(&fen) {
        Ok(board) => {
            let (mv, score) =
                search::search_parallel(board.clone(), depth, board.turn(), num_threads);
            (mv.stringify(), score)
        }
        Err(_) => ("".to_string(), 0.),
    }
}

pub fn find_move(moves: String, depth: u8, num_threads: u8) -> (String, f32) {
    let mut board = Board::start_pos();

    let mvs = moves.split(' ');
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), num_threads);
    (mv.stringify(), score)
}

#[allow(dead_code)]
pub fn from_start(depth: u8, n_threads: u8) {
    use std::time::Instant;

    let mut board = Board::start_pos();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), n_threads);
        let end = mv_start.elapsed();
        board.apply_move(mv);

        println!(
            "SCORE: {}, MOVE: {}, player: {}, time: {:?}\n{}\n",
            score,
            &mv,
            board.turn().other_player(),
            end,
            board,
        );
    }
}

pub fn get_move(wait: u64) -> String {
    let mut mv = String::new();
    let stdin = io::stdin();

    let ten_millis = time::Duration::from_millis(wait);

    while mv.is_empty() {
        match stdin.read_line(&mut mv) {
            Ok(b) => {
                if b <= 1 {
                    mv = "".to_string();
                    thread::sleep(ten_millis);
                }
            }
            Err(_) => {
                thread::sleep(ten_millis);
            }
        }
    }
    mv = mv.replace('\n', "");
    mv
}

pub fn check_if_game_over(board: &Board) -> bool {
    board.checkmate() || board.rule_50() == 50 || board.stalemate() || !board.is_ok_quick()
}

pub fn keep_alive(moves: String, depth: u8) {
    let mut board = Board::start_pos();
    let mut transposition_table = utils::new_tt_table();

    let mvs = moves.split(' ');
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    while !check_if_game_over(&board) {
        let (mv, _) = search::alpha_beta(
            board.clone(),
            depth,
            board.turn(),
            -9999.0,
            9999.0,
            &mut transposition_table,
            true,
            &None,
        );
        board.apply_move(mv);

        let new_mv = get_move(100);
        if new_mv == "stop" {
            break;
        }

        let valid = board.apply_uci_move(&new_mv);

        if !valid {
            break;
        }
    }
}