extern crate cyd;
use cyd::{alpha_beta, nega_max, new_tt_table};

use criterion::{criterion_group, Criterion};
use pleco::{Board, Player};

fn nega_max_start_pos(c: &mut Criterion) {
    let board = Board::start_pos();

    for depth in 1..4 {
        c.bench_function(
            format!("nega_max depth {} start position", depth).as_str(),
            |b| b.iter(|| nega_max(board.clone(), depth, Player::White)),
        );
    }
}

fn alpha_beta_start_pos(c: &mut Criterion) {
    let board = Board::start_pos();
    for depth in 1..4 {
        c.bench_function(
            format!("alpha beta depth {} start position", depth).as_str(),
            |b| {
                b.iter(|| {
                    let mut tt = new_tt_table();
                    alpha_beta(
                        board.clone(),
                        depth,
                        Player::White,
                        -9999.0,
                        9999.0,
                        &mut tt,
                    )
                })
            },
        );
    }
}

fn alpha_beta_queen_take(c: &mut Criterion) {
    let board = Board::from_fen("3k4/ppp1b3/5q2/5p2/8/8/1BB1PPPP/3K4 w - - 0 1").unwrap();
    for depth in 1..4 {
        c.bench_function(
            format!("alpha beta depth {} take queen", depth).as_str(),
            |b| {
                b.iter(|| {
                    let mut tt = new_tt_table();
                    alpha_beta(
                        board.clone(),
                        depth,
                        Player::White,
                        -9999.0,
                        9999.0,
                        &mut tt,
                    )
                })
            },
        );
    }
}

fn play_game(mut board: Board, depth: u8) {
    while !board.checkmate() && board.rule_50() != 50 && !board.stalemate() && board.is_ok_quick() {
        let mut tt = new_tt_table();
        let (mv, _score) = alpha_beta(board.clone(), depth, board.turn(), -9999., 9999., &mut tt);
        board.apply_move(mv);
    }
}

fn play_through_game(c: &mut Criterion) {
    let board = Board::start_pos();
    for depth in 2..5 {
        c.bench_function(
            format!("Play through Alpha beta depth {}", depth).as_str(),
            |b| b.iter(|| play_game(board.clone(), depth)),
        );
    }
}

criterion_group!(
    search_benches,
    nega_max_start_pos,
    alpha_beta_start_pos,
    alpha_beta_queen_take,
    play_through_game
);
