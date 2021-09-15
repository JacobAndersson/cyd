extern crate cyd;
use cyd::{alpha_beta, nega_max};

use criterion::{criterion_group, criterion_main, Criterion};
use pleco::{Board, Player};

fn bench_nega_max_start_pos(c: &mut Criterion) {
    let board = Board::start_pos();
    for depth in 1..4 {
        c.bench_function(
            format!("nega_max depth {} start position", depth).as_str(),
            |b| b.iter(|| nega_max(board.clone(), depth, Player::White)),
        );
    }
}

fn bench_alpha_beta_start_pos(c: &mut Criterion) {
    let board = Board::start_pos();
    for depth in 1..4 {
        c.bench_function(
            format!("alpha beta depth {} start position", depth).as_str(),
            |b| b.iter(|| alpha_beta(board.clone(), depth, Player::White, -9999.0, 9999.0)),
        );
    }
}

fn bench_alpha_beta_queen_take(c: &mut Criterion) {
    let board = Board::from_fen("3k4/ppp1b3/5q2/5p2/8/8/1BB1PPPP/3K4 w - - 0 1").unwrap();
    for depth in 1..4 {
        c.bench_function(
            format!("alpha beta depth {} take queen", depth).as_str(),
            |b| b.iter(|| alpha_beta(board.clone(), depth, Player::White, -9999.0, 9999.0)),
        );
    }
}

criterion_group!(
    benches,
    bench_nega_max_start_pos,
    bench_alpha_beta_start_pos,
    bench_alpha_beta_queen_take
);

criterion_main!(benches);
