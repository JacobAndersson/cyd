use crate::evaluate::{eval, EvalParameters};
use crate::utils;
use crate::search::transposition_table::{TtEntry, EntryFlag, TranspositionTable};
use pleco::{BitMove, Board, Player};

const DELTA_PRUNING_DIFF: i64 = 200;
const NULL_MOVE_DEPTH_REDUCTION: u8 = 2;

fn color_value(player: Player) -> i64 {
    match player {
        Player::White => 1,
        Player::Black => -1,
    }
}

fn score_move(mv: &BitMove, board: &Board) -> u32 {
    if board.gives_check(*mv) {
        20
    } else if mv.is_capture() {
        10
    } else if mv.is_quiet_move() {
        0
    } else {
        5
    }
}

fn generate_scored_moves(board: &Board, tt_table: &TranspositionTable) -> Vec<(BitMove, u32)> {
    let pv_move = tt_table.get(&board.zobrist());

    let mut moves: Vec<(BitMove, u32)> = board
        .generate_moves()
        .into_iter()
        .map(|mv| (mv, score_move(&mv, board)))
        .collect();

    if let Some(tt_entry) = pv_move {
        moves = moves
            .into_iter()
            .map(|mv| {
                if mv.0 == tt_entry.mv {
                    (mv.0, 1000)
                } else {
                    mv
                }
            })
            .collect()
    }

    moves.sort_by(|a, b| (b.1).cmp(&a.1));
    moves
}

#[allow(dead_code)] //For benchmarks
pub fn nega_max(mut board: Board, depth: u8, color: Player) -> (BitMove, i64) {
    if depth == 0 {
        return (BitMove::null(), color_value(color) * eval(&board, &None));
    }

    let mut max: i64 = -999999;
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

fn quiesce(
    mut board: Board,
    depth: u8,
    color: Player,
    mut alpha: i64,
    beta: i64,
    eval_params: &Option<EvalParameters>,
    tt_table: &mut TranspositionTable,
) -> i64 {
    let standpat = color_value(color) * eval(&board, eval_params);
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
    let moves = generate_scored_moves(&board, tt_table);

    for (mv, _) in moves {
        //Should be possible to only generate capturing moves. Problem with check
        if !(board.is_capture(mv) || board.gives_check(mv)) {
            continue;
        }

        board.apply_move(mv);
        let score = -quiesce(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
            eval_params,
            tt_table,
        );
        board.undo_move();

        if score >= beta {
            return beta;
        } else if score > alpha {
            alpha = score;
        }
    }
    alpha
}

#[allow(clippy::too_many_arguments)]
pub fn _alpha_beta(
    mut board: Board,
    depth: u8,
    color: Player,
    mut alpha: i64,
    mut beta: i64,
    tt_table: &mut TranspositionTable,
    do_null: bool,
    eval_params: &Option<EvalParameters>,
) -> (BitMove, i64) {
    let zobrist = board.zobrist();
    let alphaorig = alpha;

    if let Some(tt_entry) = tt_table.get(&zobrist) {
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

    let moves = generate_scored_moves(&board, tt_table);
    if depth == 0 || board.checkmate() || moves.is_empty() {
        return (
            BitMove::null(),
            quiesce(board, 10, color, alpha, beta, eval_params, tt_table),
        );
    }

    if do_null
        && !board.in_check()
        && board.ply() > 0
        && depth > NULL_MOVE_DEPTH_REDUCTION + 1
        && depth < 4
        && board.non_pawn_material(color) > 0
    {
        unsafe {
            board.apply_null_move();
            let (_, mut score) = _alpha_beta(
                board.shallow_clone(),
                depth - 1 - NULL_MOVE_DEPTH_REDUCTION,
                color.other_player(),
                -beta,
                -beta + 1,
                tt_table,
                false,
                eval_params,
            );
            score = -score;
            board.undo_null_move();
            if score >= beta {
                return (BitMove::null(), beta);
            }
        }
    }

    let mut best_move = BitMove::null();
    for (mv, _) in moves {
        board.apply_move(mv);
        let (_, mut score) = _alpha_beta(
            board.shallow_clone(),
            depth - 1,
            color.other_player(),
            -beta,
            -alpha,
            tt_table,
            true,
            eval_params,
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
        mv: best_move,
        depth,
        flag,
        value,
    };

    tt_table.insert(zobrist, entry);

    (best_move, alpha)
}

#[allow(clippy::too_many_arguments)]
pub fn alpha_beta(
    board: Board,
    depth: u8,
    color: Player,
    alpha: i64,
    beta: i64,
    tt_table: &mut TranspositionTable,
    do_null: bool,
    eval_params: &Option<EvalParameters>,
) -> (BitMove, i64) {
    let mut mv = BitMove::null();
    let mut latest_score: i64 = 0;

    for d in 1..(depth + 2) {
        let (m, sc) = _alpha_beta(
            board.clone(),
            d,
            color,
            alpha,
            beta,
            tt_table,
            do_null,
            eval_params,
        );
        mv = m;
        latest_score = sc;
    }

    (mv, latest_score)
}

pub fn search_parallel(board: Board, depth: u8, color: Player, _n_threads: u8) -> (BitMove, i64) {
    let mut transposition_table = utils::new_tt_table();
    let b = board.parallel_clone();

    alpha_beta(
        b,
        depth,
        color,
        -9999,
        9999,
        &mut transposition_table,
        true,
        &None,
    )
}
