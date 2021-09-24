use crate::evaluate::eval;
use crate::utils;

use pleco::{BitMove, Board, Player};

use std::sync::Arc;
use std::thread;
use std::time;

use dashmap::DashMap;

const DELTA_PRUNING_DIFF: f32 = 200.;
const NULL_MOVE_DEPTH_REDUCTION: u8 = 10;

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
        if !(board.is_capture(mv) || board.gives_check(mv)){
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
    tt_table: &mut Arc<DashMap<u64, TtEntry>>,
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

    if !board.in_check()
        && board.ply() > 0
        && depth > NULL_MOVE_DEPTH_REDUCTION + 1
        && board.non_pawn_material(color) > 0
    {
        unsafe {
            board.apply_null_move();
            let (_, mut score) = alpha_beta(
                board.shallow_clone(),
                depth - 1 - NULL_MOVE_DEPTH_REDUCTION,
                color.other_player(),
                -beta,
                -beta + 1.,
                tt_table,
            );
            score = -score;
            board.undo_null_move();
            if score >= beta {
                return (BitMove::null(), beta);
            }
        }
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

pub fn search_parallel(board: Board, depth: u8, color: Player, n_threads: u8) -> (BitMove, f32) {
    let transposition_table = utils::new_tt_table();
    let mut threads = vec![];
    for i in 0..n_threads {
        let mut tt = transposition_table.clone();
        let b = board.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(100 * i as u64));
            let mv = alpha_beta(b, depth, color, -9999.0, 9999.0, &mut tt);
            mv
        }));
    }

    let mut max = (BitMove::null(), 0.);
    for t in threads {
        max = t.join().unwrap();
    }
    max
}
