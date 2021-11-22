use crate::search;
use pleco::Board;
use std::{io, thread, time};

pub fn find_move_fen(fen: String, depth: u8, num_threads: u8) -> (String, i64) {
    println!("HERE");
    match Board::from_fen(&fen) {
        Ok(board) => {
            let (mv, score) =
                search::search_parallel(board.clone(), depth, board.turn(), num_threads, 5);
            (mv.stringify(), score)
        }
        Err(_) => ("".to_string(), 0),
    }
}

pub fn find_move(moves: String, depth: u8, num_threads: u8) -> (String, i64) {
    let mut board = Board::start_pos();

    let mvs = moves.split(' ');
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    let (mv, score) = search::search_parallel(board.clone(), depth, board.turn(), num_threads, 20);
    (mv.stringify(), score)
}

#[allow(dead_code)]
pub fn from_start(depth: u8, n_threads: u8) {
    use std::time::Instant;

    let mut board = Board::start_pos();
    while !board.checkmate() && board.rule_50() != 50 {
        let mv_start = Instant::now();
        let (mv, score) =
            search::search_parallel(board.clone(), depth, board.turn(), n_threads, 20);
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

pub fn keep_alive(moves: String, depth: u8, num_threads: u8) {
    let mut board = Board::start_pos();

    let mvs = moves.split(' ');
    for mv in mvs {
        board.apply_uci_move(mv);
    }

    while !check_if_game_over(&board) {
        let new_mv = get_move(100);
        if new_mv == "stop" {
            break;
        }

        if !new_mv.is_empty() && new_mv != "con" {
            let valid = board.apply_uci_move(&new_mv);

            if !valid {
                break;
            }
        }

        let (mv, score) =
            search::search_parallel(board.clone(), depth, board.turn(), num_threads, 20);
        println!("move{},{}", mv, score);

        board.apply_move(mv);
    }
}
