use crate::evaluate::eval;
use crate::utils;

use pleco::{BitMove, Board, Player};

use std::thread;
use std::sync::Arc;
use std::time;

use dashmap::DashMap;

const DELTA_PRUNING_DIFF: f32 = 200.;

#[derive(PartialEq)]
enum EntryFlag {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct TtEntry {
    mv: BitMove,
    depth: u8,
    flag: EntryFlag,
    value: f32,
}

fn color_value(player: Player) -> f32 {
    return match player {
        Player::White => 1.,
        Player::Black => -1.,
    };
}

#[allow(dead_code)] //For benchmarks
pub fn nega_max(mut board: Board, depth: u8, color: Player) -> (BitMove, f32) {
    if depth == 0 {
        return (BitMove::null(), color_value(color) * eval(&board));
    }

    let mut max = -999999.;
    let mut best_move = BitMove::null();

    for mv in board.generate_moves() {
        board.apply_move(mv);
        let (_, mut score) = nega_max(board.shallow_clone(), depth - 1, color.other_player());
        score = -score;

        if score > max {
            max = score;
            best_move = mv;
        }
        board.undo_move();
    }

    (best_move, max)
}

fn quiesce(mut board: Board, depth: u8, color: Player, mut alpha: f32, beta: f32) -> f32 {
    let standpat = color_value(color) * eval(&board);
    if depth == 0 {
        return standpat;
    } else if standpat >= beta {
        return beta;
    } else if alpha < standpat {
        alpha = standpat;
    }

    if standpat < alpha - DELTA_PRUNING_DIFF {
        return alpha;
    }

    let moves = board.generate_moves();
    for mv in moves {
        //Should be possible to only generate capturing moves. Problem with check
        if !board.is_capture(mv) {
            continue;
        }

        board.apply_move(mv);
        let score = -quiesce(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
        );
        board.undo_move();

        if score >= beta {
            return beta;
        } else if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}

pub fn alpha_beta(
    mut board: Board,
    depth: u8,
    color: Player,
    mut alpha: f32,
    mut beta: f32,
    tt_table: &mut Arc<DashMap<u64, TtEntry>>
) -> (BitMove, f32) {
    let moves = board.generate_moves();
    let zobrist = board.zobrist();
    let alphaorig = alpha;

    {
        match tt_table.get(&zobrist) {
            Some(tt_entry) => {
                if tt_entry.depth >= depth {
                    let flag = &tt_entry.flag;
                    if flag == &EntryFlag::Exact {
                        return (tt_entry.mv, tt_entry.value);
                    } else if flag == &EntryFlag::LowerBound {
                        alpha = alpha.max(tt_entry.value);
                    } else if flag == &EntryFlag::UpperBound {
                        beta = beta.min(tt_entry.value);
                    }
                    if alpha >= beta {
                        return (tt_entry.mv, tt_entry.value);
                    }
                }
            }
            None => {}
        }
    }

    if depth == 0 || board.checkmate() || moves.is_empty() {
        return (BitMove::null(), quiesce(board, 10, color, alpha, beta));
    }

    let mut best_move = BitMove::null();
    for mv in moves {
        board.apply_move(mv);
        let (_, mut score) = alpha_beta(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
            tt_table,
        );
        board.undo_move();
        score = -score;

        if score >= beta {
            return (mv, beta);
        } else if score > alpha {
            alpha = score;
            best_move = mv;
        }

        if alpha >= beta {
            break;
        }
    }

    let value = alpha;
    let mut flag = EntryFlag::Exact;

    if value <= alphaorig {
        flag = EntryFlag::UpperBound;
    } else if value >= beta {
        flag = EntryFlag::LowerBound;
    }

    let entry = TtEntry {
        mv: best_move.clone(),
        depth,
        flag,
        value,
    };
    {
        tt_table.insert(zobrist, entry);
    }

    (best_move, alpha)
}

pub fn search_parallel(board: Board, depth: u8, color: Player,  n_threads: u8) -> (BitMove, f32) {
    let transposition_table = utils::new_tt_table();
    let mut threads = vec![];
    for i in 0..n_threads {
        let mut tt = transposition_table.clone();
        let b = board.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(100*i as u64));
            let mv = alpha_beta(
                b,
                depth,
                color,
                -9999.0,
                9999.0,
                &mut tt
           );
           mv
        }));
    }

    let mut max = (BitMove::null(), 0.);
    for t in threads {
        max = t.join().unwrap(); 
    }
    max
}

#[cfg(test)]
mod search_test {
    use super::*;

    fn test_position_nega_max(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let board = Board::from_fen(fen).unwrap();
        let (mv, score) = nega_max(board, depth, player);

        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn test_position_alpha_beta(fen: &str, depth: u8, player: Player, correct_move: &str) -> bool {
        let mut tt = utils::new_tt_table();
        let board = Board::from_fen(fen).unwrap();
        let (mv, score) = alpha_beta(board, depth, player, -9999.0, 9999.0, &mut tt);
        println!("depth: {}, move: {}, score: {}", depth, mv, score);
        correct_move == mv.stringify()
    }

    fn play_x_moves(fen: &str, depth: u8, plies: u8) -> Board {
        let mut board = Board::from_fen(fen).unwrap();
        for _i in 0..plies {
            let mut tt = utils::new_tt_table();

            let (mv, _score) =
                alpha_beta(board.clone(), depth, board.turn(), -9999.0, 9999.0, &mut tt);
            board.apply_move(mv)
        }
        board
    }

    #[test]
    fn queen_take_white() {
        let fen = "2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1";
        let correct_move = "c4e6";
        for depth in 1..3 {
            let found = test_position_nega_max(&fen, depth, Player::White, &correct_move);
            assert!(found);
        }
    }

    #[test]
    fn queen_take_black() {
        let fen = "2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1";
        let correct_move = "e7b4";
        for depth in 1..3 {
            let found = test_position_nega_max(&fen, depth, Player::Black, &correct_move);
            assert!(found);
        }
    }

    #[test]
    fn queen_take_white_alpha_beta() {
        let fen = "2k5/8/4q3/8/2B5/8/8/1K6 w - - 0 1";
        let correct_move = "c4e6";
        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn queen_take_black_alpha_beta() {
        let fen = "2k5/4b1n1/5P2/8/1Q3P2/4n3/2P3n1/1K6 b - - 0 1";
        let correct_move = "e7b4";
        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::Black, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn knight_take_white_alpha_beta() {
        let fen = "2k4r/6pp/8/2p1n3/8/3N4/4PPPP/2K4R w - - 0 1";
        let correct_move = "d3e5";

        for depth in 1..3 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn pin_knight_white_alpha_beta() {
        let fen = "2k4r/6pp/4n3/2p5/8/5B2/4PPPP/2K4R w - - 0 1";
        let correct_move = "f3g4";

        for depth in 4..5 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_one_white() {
        let fen = "k7/5R2/6R1/8/8/8/4K3/8 w - - 0 1";
        let correct_move = "g6g8";

        for depth in 1..4 {
            let find = test_position_alpha_beta(fen, depth, Player::White, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_one_black() {
        let fen = "1k6/8/8/8/8/3n4/6PR/6RK b Q - 0 1";
        let correct_move = "d3f2";
        for depth in 1..4 {
            let find = test_position_nega_max(fen, depth, Player::Black, correct_move);
            assert!(find);
        }
    }

    #[test]
    fn mate_in_two_white() {
        let fen = "k7/4R3/8/8/8/4R3/8/3K4 w - - 0 1";
        for depth in 4..6 {
            let board = play_x_moves(fen, depth, 3);
            assert!(board.checkmate());
        }
    }

    #[test]
    fn mate_in_two_2() {
        let fen = "k7/4R3/2p5/p7/1p6/2P1R2P/1P4P1/3K4 w - - 0 1";
        for depth in 4..6 {
            let board = play_x_moves(fen, depth, 3);
            assert!(board.checkmate());
        }
    }
}
